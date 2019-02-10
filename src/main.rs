extern crate sdl2;

use sdl2::image::{LoadTexture, InitFlag};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::mouse::{MouseState, MouseButton};
use sdl2::rect::Rect;

use std::time::Instant;
use std::cmp;

mod point;
mod camera;
mod map;
mod path_finder;
mod entity;
mod entity_holder;


pub fn main() -> Result<(), String> {

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
    let window = video_subsystem
        .window("rust demo", 1000, 800)
        .resizable()
        .build()
        .expect("Error building window");

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

    let mut tick: u32 = 0;

    let texture_creator = canvas.texture_creator();
    let person_texture = texture_creator.load_texture("src/images/ukko.png")?;
    let land_texture = texture_creator.load_texture("src/images/maa.png")?;
    let water_texture = texture_creator.load_texture("src/images/vesi.png")?;
    let shadow_texture = texture_creator.load_texture("src/images/shadow.png")?;

    let mut guy_x = 0;
    let mut guy_y = 0;

    let mut camera: camera::Camera = camera::Camera::new(600, 600);
    let mut map: map::Map = map::Map::new_random(200, 200);
    let mut entity_holder: entity_holder::EntityHolder = entity_holder::EntityHolder::new();

    let start_time = Instant::now();
    let mut last_time = start_time.elapsed();
    let mut elapsed_time = 1_000_000_000;

    let mut left_pressed: bool = false;
    let mut mouse_start_game_pos: (f32, f32) = (0.0, 0.0);

    // println!("HINT SET MAYBE, {}", sdl2::hint::set("SDL_HINT_RENDER_SCALE_QUALITY", "1"));

    loop {
        // Events
        let mouse_state: MouseState = event_pump.mouse_state();
        let mouse_game_pos: (f32, f32) = camera.screen_to_game(mouse_state.x(), mouse_state.y());
        let keyboard_state = event_pump.keyboard_state();

        if keyboard_state.is_scancode_pressed(Scancode::S) {camera.move_center( 0.0,  0.5)}
        if keyboard_state.is_scancode_pressed(Scancode::W) {camera.move_center( 0.0, -0.5)}
        if keyboard_state.is_scancode_pressed(Scancode::A) {camera.move_center(-0.5,  0.0)}
        if keyboard_state.is_scancode_pressed(Scancode::D) {camera.move_center( 0.5,  0.0)}

        if mouse_state.left() {
            if left_pressed == false {
                mouse_start_game_pos = mouse_game_pos;
            }
            left_pressed = true;
        } else {
            if left_pressed == true {
                entity_holder.set_selection(mouse_game_pos, mouse_start_game_pos);
            }
            left_pressed = false;
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return Ok(()),
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return Ok(()),
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => (guy_x += 1),
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => (guy_x -= 1),
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => (guy_y -= 1),
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => (guy_y += 1),
                Event::KeyDown { keycode: Some(Keycode::I), .. } => (camera.zoom_in()),
                Event::KeyDown { keycode: Some(Keycode::O), .. } => (camera.zoom_out()),
                Event::KeyDown { keycode: Some(Keycode::K), .. } => {
                    let game_pos: (f32, f32) = mouse_game_pos;
                    map.set_water(game_pos.0 as u32, game_pos.1 as u32);
                },
                Event::KeyDown { keycode: Some(Keycode::L), .. } => {
                    let game_pos: (f32, f32) = mouse_game_pos;
                    map.set_grass(game_pos.0 as u32, game_pos.1 as u32);
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Right, .. } => {
                    println!("Ordering units to go to {:?}", mouse_game_pos);
                    entity_holder.order_selected_units_to(&map, mouse_game_pos);
                },
                Event::KeyDown { keycode: Some(Keycode::N), .. }
                // | Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. }
                => {
                    let game_pos: (f32, f32) = mouse_game_pos;
                    println!(
                        "Mouse pos screen: ({}, {}), game: ({}, {})",
                        mouse_state.x(),
                        mouse_state.y(),
                        game_pos.0,
                        game_pos.1
                    );

                    entity_holder.add_new_entity(game_pos.0, game_pos.1);
                },
                _ => {}
            }
        }

        { // Window handling
            let mut_window = canvas.window_mut();
            let window_size = mut_window.size();
            camera.update_window_size(window_size.0, window_size.1);
            if tick % 60 == 0 {
                let title = format!(
                    "Oskun peli, tick: {}, fps: {}, entities: {}",
                    tick,
                    (1.0 / (elapsed_time as f32 / 1000000000.0)) as i32,
                    entity_holder.get_entity_refs().len()
                );
                mut_window.set_title(&title).map_err(|e| e.to_string())?;
            }
        }

        // Game handling
        entity_holder.entities_ai_stuff(&map);
        entity_holder.entities_interact_with_each_other();
        entity_holder.entities_interact_with_map(&map);

        { // Draw
            canvas.set_draw_color(Color::RGB(55, 55, 55));
            canvas.clear();

            // Draw ground
            for x in 0..map.width {
                for y in 0..map.height {
                    let texture_pointer = match map.get_at(x, y) {
                        map::GroundType::Grass => &land_texture,
                        map::GroundType::Water => &water_texture,
                        _ => &shadow_texture
                    };
                    canvas.copy(
                        texture_pointer,
                        None,
                        camera.game_to_rect_i(x as i32, y as i32)
                    ).map_err(|e| e.to_string())?;
                }
            }

            // Draw guy
            canvas
                .copy(&person_texture, None, camera.game_to_rect_i(guy_x, guy_y))
                .map_err(|e| e.to_string())?;

            // Draw entities
            canvas.set_draw_color(Color::RGB(0, 0, 255));
            for entity in entity_holder.get_entity_refs() {
                let screen_center_pos = camera.game_to_screen(entity.location.x, entity.location.y);
                let rect = Rect::new(
                    (screen_center_pos.0 - 1.0 * 32.0 / camera.zoom) as i32,
                    (screen_center_pos.1 - 1.0 * 32.0 / camera.zoom) as i32,
                    (64.0 / camera.zoom) as u32,
                    (64.0 / camera.zoom) as u32
                );
                canvas.copy(&shadow_texture, None, rect).map_err(|e| e.to_string())?;
                if entity_holder.entity_selected(&entity) {
                    canvas.draw_rect(rect)?;
                }
            }

            // Draw mouse selection box
            canvas.set_draw_color(Color::RGB(0, 0, 255));
            if left_pressed {
                let pos_1 = camera.game_to_screen(mouse_start_game_pos.0, mouse_start_game_pos.1);
                let pos_2 = camera.game_to_screen(mouse_game_pos.0, mouse_game_pos.1);

                canvas.draw_rect(Rect::new(
                    cmp::min(pos_1.0 as i32, pos_2.0 as i32),
                    cmp::min(pos_1.1 as i32, pos_2.1 as i32),
                    (pos_2.0 - pos_1.0).abs() as u32,
                    (pos_2.1 - pos_1.1).abs() as u32,
                ))?;
            }

            canvas.present();
        }

        // Sleep 
        let last_stamp = (last_time.as_secs() * 1_000_000_000) + (last_time.subsec_nanos()) as u64;
        last_time = start_time.elapsed();
        let now_stamp = (last_time.as_secs() * 1_000_000_000) + (last_time.subsec_nanos()) as u64;

        // let optimal_sleep_time = 1_000_000_000u64 / 60;

        elapsed_time = now_stamp - last_stamp;

        // ::std::thread::sleep(Duration::new(0, sleep_time as u32));
        tick += 1;
    }
}
