

use sdl2::image::LoadSurface;


use sdl2::surface::{Surface};
use sdl2::video::WindowContext;

use sdl2::render::{Texture, TextureCreator};
use sdl2::pixels::PixelFormatEnum;

use sdl2::rect::Rect;
use sdl2::pixels::Color;


pub struct TextureHolder<'a> {
    pub ground_texture: Texture<'a>,
    unit_textures: Vec<Texture<'a>>,
    unit_surface: Surface<'a>,
    unit_surface_mask: Surface<'a>,
}

impl<'a> TextureHolder<'a> {
    fn surface_to_texture(
        surface: & Surface,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<Texture<'a>, String> {
        let texture: Texture<'a> = match texture_creator.create_texture_from_surface(surface) {
            Ok(x) => x,
            Err(_error) => {
                println!("How to handle this error?");
                return Err("Texture loading failed".to_string());
            }
        };
        Ok(texture)
    }

    pub fn generate_team_texture(
        &mut self,
        color: Color,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<i32, String> {
        let mut new_unit_surface: Surface = Surface::new(512, 3 * 128, PixelFormatEnum::RGBA32)?;
        let my_rect = Rect::new(0, 0, 512, 3 * 128);

        self.unit_surface.blit(my_rect, &mut new_unit_surface, my_rect)?;
        self.unit_surface_mask.set_color_mod(color);
        self.unit_surface_mask.blit(my_rect, &mut new_unit_surface, my_rect)?;

        let unit_texture = TextureHolder::surface_to_texture(&new_unit_surface, texture_creator)?;
        self.unit_textures.push(unit_texture);

        Ok(0)
    }

    pub fn get_team_texture(& self, team_id: usize) -> Result<&Texture, String> {
        match self.unit_textures.get(team_id) {
            Some(x) => Ok(x),
            _ => Err("get_team_texture: Invalid team id. Not pre loaded".to_string())
        }
    }

    pub fn new( texture_creator: &'a TextureCreator<WindowContext>,) -> Result<TextureHolder<'a>, String> {

        let ground_surface: Surface = LoadSurface::from_file("src/images/ground.png")?;
        let ground_texture: Texture<'a> = TextureHolder::surface_to_texture(&ground_surface, texture_creator)?;

        let unit_surface: Surface = LoadSurface::from_file("src/images/unit_roster.png")?;
        let unit_surface_mask: Surface = LoadSurface::from_file("src/images/unit_roster_mask.png")?;

        let mut texture_holder = TextureHolder {
            ground_texture: ground_texture,
            unit_surface: unit_surface,
            unit_surface_mask: unit_surface_mask,
            unit_textures: Vec::new(),
        };

        texture_holder.generate_team_texture(Color::RGB(255, 64, 32), texture_creator)?;
        texture_holder.generate_team_texture(Color::RGB(32, 64, 255), texture_creator)?;
        texture_holder.generate_team_texture(Color::RGB(0, 255, 0), texture_creator)?;
        texture_holder.generate_team_texture(Color::RGB(255, 128, 64), texture_creator)?;

        Ok(texture_holder)
    }
}


