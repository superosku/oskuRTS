use rand::{self, Rng};
use super::point;

#[derive(Copy, Clone, PartialEq)]
pub enum GroundType {
    Empty,
    Grass,
    Water
}

pub struct Map {
    pub height: u32,
    pub width: u32,
    pub data: Vec<GroundType>
}

impl Map {
    pub fn new_random(width: u32, height: u32) -> Map {
        let data_size: u32 = width * height;
        let mut new_map: Map = Map {
            height: height,
            width: width,
            data: vec![GroundType::Grass; (data_size) as usize] // Vec::new()
        };

        let mut randomizer = rand::thread_rng();
        for n in 0..data_size {
            let randint: u32 = randomizer.gen_range(0,7);
            if randint % 7 == 0 {
            // if n % 3 == 0 {
                new_map.data[n as usize] = GroundType::Water;
            }
        }

        return new_map;
    }

    pub fn get_at(&self, x: u32, y: u32) -> GroundType {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return GroundType::Empty
        }
        let index: usize = (x + y * self.width) as usize;
        return self.data[index];
    }

    pub fn point_moveable(&self, point: (i32, i32)) -> bool {
        if point.0 < 0 || point.1 < 0 {
            return true;
        }
        // println!("Checking {} {} {} {}", point.x, point.y, point.x as u32, point.y as u32);
        let ground_type = self.get_at(
            point.0 as u32,
            point.1 as u32
        );
        ground_type != GroundType::Water
    }
}

