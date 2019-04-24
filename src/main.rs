extern crate multi_mut;
extern crate sdl2;
extern crate byteorder;


use std::collections::HashMap;
use std::cmp::Ordering;
use std::env;

use sdl2::image::{LoadTexture, InitFlag};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::mouse::{MouseState, MouseButton};
use sdl2::rect::{Rect, Point};
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::video::{WindowContext, Window};

use std::time::Instant;
use std::cmp;

mod point;
mod camera;
mod map;
mod path_finder;
mod entity;
mod entity_holder;
mod noise;
mod texture_holder;
mod projectile;
mod building;
mod game_state;
mod binary_helpers;

use game_state::{GameState, GameEvent};


pub fn main() -> Result<(), String> {

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window: Window = video_subsystem
        .window("rust demo", 1400, 1000)
        .resizable()
        .build()
        .expect("Error building window");

    // let mut canvas: Canvas<Window> = window
    let mut canvas: WindowCanvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

    let mut tick: u32 = 0;

    let texture_creator: TextureCreator<WindowContext> = canvas.texture_creator();
    
    let shadow_texture = texture_creator.load_texture("src/images/shadow.png")?;
    let small_shadow_texture = texture_creator.load_texture("src/images/small_shadow.png")?;

    let mut camera: camera::Camera = camera::Camera::new(600, 600);

    let args: Vec<String> = env::args().collect();

    let mut game_state = match args.get(1) {
        Some(argument) => {
            println!("Loading game state from file: {}", argument);
            GameState::from_file_name(argument.to_string())
        },
        None => {
            println!("Initializing new game state");
            GameState::new()
        },
    };

    let start_time = Instant::now();
    let mut last_time = start_time.elapsed();
    let mut elapsed_time = 1_000_000_000;

    let mut left_pressed: bool = false;
    let mut mouse_start_game_pos: (f32, f32) = (0.0, 0.0);

    let mut debug_enabled = false;
    
    let texture_holder: texture_holder::TextureHolder = texture_holder::TextureHolder::new(&texture_creator)?;

    let mut selected_entity_ids: HashMap<u32, bool> = HashMap::new();

    // return Ok(());

    loop {
        // Events
        let mouse_state: MouseState = event_pump.mouse_state();
        let mouse_game_pos: (f32, f32) = camera.screen_to_game(mouse_state.x(), mouse_state.y());
        let mouse_game_point: point::Point = point::Point::new(mouse_game_pos.0, mouse_game_pos.1);
        let keyboard_state = event_pump.keyboard_state();

        if keyboard_state.is_scancode_pressed(Scancode::S) {camera.move_center( 0.0,  0.5)}
        if keyboard_state.is_scancode_pressed(Scancode::W) {camera.move_center( 0.0, -0.5)}
        if keyboard_state.is_scancode_pressed(Scancode::A) {camera.move_center(-0.5,  0.0)}
        if keyboard_state.is_scancode_pressed(Scancode::D) {camera.move_center( 0.5,  0.0)}

        let mut attack_move = false;
        if keyboard_state.is_scancode_pressed(Scancode::Q) {attack_move = true};

        if mouse_state.left() {
            if left_pressed == false {
                mouse_start_game_pos = mouse_game_pos;
            }
            left_pressed = true;
        } else {
            if left_pressed == true {
                { // Setting selected entities
                    selected_entity_ids.clear();
                    for entity in game_state.entity_holder().entities_iter() {
                        if entity.is_inside(mouse_game_pos, mouse_start_game_pos) && entity.team_id() == 0 {
                            selected_entity_ids.insert(entity.id(), true);
                        }
                    }
                }
            }
            left_pressed = false;
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return Ok(()),
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return Ok(()),
                Event::KeyDown { keycode: Some(Keycode::I), .. } => (camera.zoom_in()),
                Event::KeyDown { keycode: Some(Keycode::O), .. } => (camera.zoom_out()),
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {debug_enabled = !debug_enabled;},
                Event::MouseWheel { .. } => {
                    println!("Scroll happened");
                },
                Event::KeyDown { keycode: Some(Keycode::J), .. } => {
                    game_state.dispatch_event(GameEvent::SetMapPoint{
                        location: mouse_game_point.as_i(),
                        ground_type: map::GroundType::Grass
                    });
                },
                Event::KeyDown { keycode: Some(Keycode::K), .. } => {
                    game_state.dispatch_event(GameEvent::SetMapPoint{
                        location: mouse_game_point.as_i(),
                        ground_type: map::GroundType::Water
                    });
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Right, .. } => {
                    let task: entity::Task = if attack_move {
                        entity::Task::AttackMove {point: mouse_game_point}
                    } else {
                        entity::Task::Move {point: mouse_game_point}
                    };
                    game_state.dispatch_event(GameEvent::OrderUnits{
                        task: task,
                        unit_ids: selected_entity_ids.clone(),
                    })
                },
                Event::KeyDown { keycode: Some(Keycode::B), .. } => {
                    game_state.dispatch_event(GameEvent::AddBuilding{
                        location: mouse_game_point.as_i()
                    })
                },
                Event::KeyDown { keycode: Some(Keycode::N), .. } => {
                    game_state.dispatch_event(GameEvent::InsertUnit{
                        location: mouse_game_point.clone(),
                        team_id: 0,
                        unit_type: entity::EntityType::Ranged,
                    });
                },
                Event::KeyDown { keycode: Some(Keycode::M), .. } => {
                    game_state.dispatch_event(GameEvent::InsertUnit{
                        location: mouse_game_point.clone(),
                        team_id: 1,
                        unit_type: entity::EntityType::Ranged,
                    });
                },
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    game_state.dispatch_event(GameEvent::OrderUnits{
                        task: entity::Task::Idle,
                        unit_ids: selected_entity_ids.clone(),
                    })
                },
                Event::KeyDown { keycode: Some(Keycode::T), .. } => {
                    game_state.save_to_file();
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
                    game_state.entity_holder().entities_iter().len()
                );
                mut_window.set_title(&title).map_err(|e| e.to_string())?;
            }
        }

        // Game handling
        game_state.do_tick();


        { // Draw
            canvas.set_draw_color(Color::RGB(55, 55, 55));
            canvas.clear();

            // Draw ground
            {
                let map = game_state.map();
                let tile_size = camera.get_tile_size();
                let first_screen_pos_f = camera.game_to_screen(0.0, 0.0);
                let first_screen_pos = (first_screen_pos_f.0 as i32, first_screen_pos_f.1 as i32);
                for x in 0..map.width() {
                    for y in 0..map.height() {
                        let texture_id: u32 = match map.get_at(x as i32, y as i32) {
                            map::GroundType::Grass => Ok(0),
                            map::GroundType::Water => Ok(2),
                            // map::GroundType::Forest=> Ok(1),
                            map::GroundType::Sand => Ok(3),
                            map::GroundType::Rock => Ok(4),
                            // map::GroundType::CutTrees => Ok(5),
                            _ => Err("Invalid GroundType for drawing".to_string())
                        }?;
                        canvas.copy(
                            &texture_holder.ground_texture,
                            Rect::new(texture_id as i32 * 64, 0, 64, 64),
                            Rect::new(
                                first_screen_pos.0 + (x * tile_size) as i32,
                                first_screen_pos.1 + (y * tile_size) as i32,
                                tile_size,
                                tile_size,
                            )
                        )?;

                        let texture_id: i32 = match map.get_at_second_level(x as i32, y as i32) {
                            map::SecondLevelType::Tree => Ok(1),
                            map::SecondLevelType::CutTree=> Ok(5),
                            map::SecondLevelType::Building=> Ok(6),
                            map::SecondLevelType::Empty => Ok(-1),
                            _ => Err("Invalid GroundType for drawing".to_string())
                        }?;
                        if texture_id != -1 {
                            canvas.copy(
                                &texture_holder.ground_texture,
                                Rect::new(texture_id * 64, 0, 64, 64),
                                Rect::new(
                                    first_screen_pos.0 + (x * tile_size) as i32,
                                    first_screen_pos.1 + (y * tile_size) as i32,
                                    tile_size,
                                    tile_size,
                                )
                            )?;
                        }
                    }
                }
            }

            // Draw last search tree size
            if debug_enabled {
                for point in game_state.entity_holder().debug_search_tree.keys() {
                    let screen_pos = camera.game_to_screen(point.0 as f32 + 0.5, point.1 as f32 + 0.5);
                    canvas.draw_rect(Rect::new(screen_pos.0 as i32 - 2, screen_pos.1 as i32 - 2, 4, 4))?;
                }
            }

            // Draw entities
            let unit_tile_size = (64.0 / camera.zoom) as u32;

            // let entities = game_state.entity_holder().get_entity_refs();
            // let ref_to_entities: &Vec<entity::Entity> = game_state.entity_holder().entities_iter(); //.map(|e| &e).collect();
            let ref_to_entities = game_state.entity_holder().entities_iter(); //.map(|e| &e).collect();
            let mut entity_refs: Vec<&entity::Entity> = ref_to_entities.map(|e| e).collect();

            entity_refs.sort_by(
                |a, b|
                if a.location().y > b.location().y {Ordering::Greater} else {Ordering::Less}
            );

            // entities_sorted.sort_by(|a, b| a.location().y > b.location().y);

            // for entity in game_state.entity_holder().get_entity_refs() {
            for entity in entity_refs {
                canvas.set_draw_color(Color::RGB(0, 0, 255));

                let entity_location = entity.location();
                let screen_center_pos = camera.game_to_screen(entity_location.x, entity_location.y);

                let rect = Rect::new(
                    (screen_center_pos.0 - 1.0 * 32.0 / camera.zoom) as i32,
                    (screen_center_pos.1 - 1.0 * 32.0 / camera.zoom) as i32,
                    unit_tile_size,
                    unit_tile_size,
                );
                let unit_texture_rect = Rect::new(
                    (screen_center_pos.0 - 1.0 * 32.0 / camera.zoom) as i32,
                    (screen_center_pos.1 - 1.0 * 32.0 / camera.zoom) as i32 - unit_tile_size as i32,
                    unit_tile_size,
                    unit_tile_size * 2,
                );
                canvas.copy(&shadow_texture, None, rect).map_err(|e| e.to_string())?;
                let entity_type_id: u32 = match entity.entity_type() {
                    entity::EntityType::Peasant => 0,
                    entity::EntityType::Meelee => 1,
                    entity::EntityType::Ranged => 2,
                };
                canvas.copy(
                    texture_holder.get_team_texture((entity.team_id()) as usize)?,
                    Rect::new(
                        64 * entity.orientation() as i32,
                        128 * entity_type_id as i32,
                        64,
                        128
                    ),
                    unit_texture_rect
                )?;

                canvas.set_draw_color(Color::RGB(255, 255, 255));
                if selected_entity_ids.contains_key(&entity.id()) {
                    canvas.draw_rect(rect)?;
                }
                canvas.set_draw_color(Color::RGB(0, 0, 255));
                if debug_enabled {
                    match &entity.get_waypoint() {
                        Some(w) => {
                            let screen_end_pos = camera.game_to_screen(w.x, w.y);
                            canvas.draw_line(
                                Point::new(screen_center_pos.0 as i32, screen_center_pos.1 as i32),
                                Point::new(screen_end_pos.0 as i32, screen_end_pos.1 as i32)
                            )?;
                        },
                        _ => {}
                    }
                    canvas.set_draw_color(Color::RGB(255, 0, 255));
                    let stuff: Vec<Point> = entity.path().iter().map(|point| {
                        let screen_point= camera.game_to_screen(point.x, point.y);
                        return Point::new(screen_point.0 as i32, screen_point.1 as i32);
                    }).collect();
                    canvas.draw_lines(stuff.as_slice())?;

                    canvas.set_draw_color(Color::RGB(0, 255, 255));
                    match &entity.closest_seen_enemy_point() { Some(point) => {
                        let screen_end_pos = camera.game_to_screen(point.x, point.y);
                        canvas.draw_line(
                            Point::new(screen_center_pos.0 as i32, screen_center_pos.1 as i32),
                            Point::new(screen_end_pos.0 as i32, screen_end_pos.1 as i32)
                        )?;
                    }, _ => {} }
                }

                // HP bar
                let entity_max_hp = entity.max_hp();
                let health_persentage = entity.hp() as f32 / entity_max_hp as f32;

                let max_hp_rect = Rect::new(
                    (screen_center_pos.0 - 1.0 * 32.0 / camera.zoom) as i32 - (unit_tile_size as f32 * 0.2) as i32,
                    (screen_center_pos.1 + 1.0 * 32.0 / camera.zoom) as i32,
                    (unit_tile_size as f32 * 1.4) as u32,
                    unit_tile_size * 2 / 7,
                );
                let hp_rect = Rect::new(
                    (screen_center_pos.0 - 1.0 * 32.0 / camera.zoom) as i32 - (unit_tile_size as f32 * 0.2) as i32,
                    (screen_center_pos.1 + 1.0 * 32.0 / camera.zoom) as i32,
                    ((unit_tile_size as f32 * 1.4) as f32 * health_persentage) as u32,
                    unit_tile_size * 2 / 7,
                );
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.fill_rect(max_hp_rect)?;
                canvas.set_draw_color(Color::RGB(0, 255, 0));
                canvas.fill_rect(hp_rect)?;
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.draw_rect(max_hp_rect)?;
            }

            // Draw buildings
            for building in game_state.entity_holder().buildings.iter() {
                let screen_top_left_pos = camera.game_to_screen(building.x() as f32, building.y() as f32);
                let tile_size = camera.get_tile_size();
                let rect = Rect::new(
                    screen_top_left_pos.0 as i32,
                    screen_top_left_pos.1 as i32 - tile_size as i32,
                    (building.width() * tile_size as i32) as u32,
                    ((building.height() + 1) * tile_size as i32) as u32,
                );

                canvas.copy(
                   &texture_holder.building_1_texture,
                   None,
                   rect
               );
            }

            // Draw projectiles
            for projectile in game_state.entity_holder().projectiles.iter() {
                let screen_center_pos = camera.game_to_screen(projectile.location().x, projectile.location().y);
                let shadow_rect = Rect::new(
                    (screen_center_pos.0 - 1.0 * 32.0 / camera.zoom) as i32,
                    (screen_center_pos.1 - 1.0 * 32.0 / camera.zoom) as i32,
                    unit_tile_size,
                    unit_tile_size,
                );
                let rect = Rect::new(
                    (screen_center_pos.0 - 1.0 * 32.0 / camera.zoom) as i32,
                    (
                        screen_center_pos.1 - 1.0 * 32.0 / camera.zoom -
                        camera.get_tile_size() as f32 * 0.5 - // Throw should not start from ground
                        camera.get_tile_size() as f32 * projectile.get_height() * 0.2 // Parabel
                    ) as i32,
                    unit_tile_size,
                    unit_tile_size,
                );

                canvas.copy(
                    &small_shadow_texture,
                    None,
                    shadow_rect
                )?;
                canvas.copy_ex(
                    &texture_holder.arrow_texture,
                    None,
                    rect,
                    (-57.2958 * projectile.angle() as f64) - (45.0 + 180.0),
                    None,
                    false,
                    false,
                ).map_err(|e| e.to_string())?;

                /*
                let screen_pos = camera.game_to_screen(projectile.location.x, projectile.location.y);

                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.fill_rect(Rect::new(screen_pos.0 as i32 - 4, screen_pos.1 as i32 - 4, 8, 8))?;
                */
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
