use rand::{self, Rng};
use std::cmp;

use super::point;
use super::noise;

#[derive(Copy, Clone, PartialEq)]
pub enum GroundType {
    Empty,
    Grass,
    Water,
    Forest,
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

        let mut water_noise1 = noise::PerlinNoise::new(3);
        let mut water_noise2 = noise::PerlinNoise::new(6);
        let mut water_noise3 = noise::PerlinNoise::new(9);

        let mut tree_noise1 = noise::PerlinNoise::new(2);
        let mut tree_noise2 = noise::PerlinNoise::new(4);
        let mut tree_noise3 = noise::PerlinNoise::new(8);

        for n in 0..data_size {
            let x = n % width;
            let y = n / width;

            let noise_value = (
                tree_noise1.value_at(x as i32, y as i32) * 1.0 +
                tree_noise2.value_at(x as i32, y as i32) * 2.0 +
                tree_noise3.value_at(x as i32, y as i32) * 4.0
            );

            if noise_value < -0.7 {
                new_map.data[n as usize] = GroundType::Forest;
            }
        }

        for n in 0..data_size {
            let x = n % width;
            let y = n / width;

            let noise_value = (
                water_noise1.value_at(x as i32, y as i32) * 1.0 +
                water_noise2.value_at(x as i32, y as i32) * 2.0 +
                water_noise3.value_at(x as i32, y as i32) * 4.0
            );

            if noise_value < -0.7 {
                new_map.data[n as usize] = GroundType::Water;
            }
        }
        return new_map;
    }

    pub fn set_water(&mut self, x: u32, y: u32) {
        let index: usize = (x + y * self.width) as usize;
        self.data[index] = GroundType::Water;
    }

    pub fn set_grass(&mut self, x: u32, y: u32) {
        let index: usize = (x + y * self.width) as usize;
        self.data[index] = GroundType::Grass;
    }

    pub fn line_of_sight(&self, point_1: &point::Point, point_2: &point::Point) -> bool {
        let mut min_x = point_1.x.min(point_2.x);
        let mut min_y = point_1.y.min(point_2.y);
        let mut max_x = point_1.x.max(point_2.x);
        let mut max_y = point_1.y.max(point_2.y);

        if min_x % 1.0 < 0.25 {
            min_x -= 0.5
        }
        if min_y % 1.0 < 0.25 {
            min_y -= 0.5
        }
        if max_x % 1.0 > 0.75 {
            max_x += 0.5
        }
        if max_y % 1.0 > 0.75 {
            max_y += 0.5
        }

        /*
        let min_x = cmp::min(point_1.x as i32, point_2.x as i32);
        let max_x = cmp::max(point_1.x as i32, point_2.x as i32);
        let min_y = cmp::min(point_1.y as i32, point_2.y as i32);
        let max_y = cmp::max(point_1.y as i32, point_2.y as i32);
        */

        // println!("  Checking point moveability area ({} {}) ({} {})", min_x, min_y, max_x, max_y);
        for x in (min_x as i32)..(max_x as i32 + 1) {
            for y in (min_y as i32)..(max_y as i32 + 1) {
                // println!("    Point moveable {} {} {}", x, y, self.point_moveable((x, y)));
                if !self.point_moveable((x,y)) {
                    return false;
                }
            }
        }

        true
    }

    pub fn get_at(&self, x: u32, y: u32) -> GroundType {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return GroundType::Empty
        }
        let index: usize = (x + y * self.width) as usize;
        return self.data[index];
    }

    pub fn closest_moveable_point(&self, x: i32, y: i32) -> (i32, i32) {
        for i in 1..10 {
            if self.point_moveable((x + i, y)) {return (x + i, y);};
            if self.point_moveable((x, y + i)) {return (x, y + i);};
            if self.point_moveable((x - i, y)) {return (x - i, y);};
            if self.point_moveable((x, y - i)) {return (x, y - i);};
        }

        (x, y)
    }

    pub fn point_moveable(&self, point: (i32, i32)) -> bool {
        if point.0 < 0 || point.1 < 0 {
            return false;
        }
        // println!("Checking {} {} {} {}", point.x, point.y, point.x as u32, point.y as u32);
        let ground_type = self.get_at(
            point.0 as u32,
            point.1 as u32
        );
        ground_type == GroundType::Grass
    }
}

