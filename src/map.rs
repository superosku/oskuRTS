use super::point;
use super::noise;

#[derive(Copy, Clone, PartialEq)]
pub enum GroundType {
    Empty,

    Grass,
    Water,
    Forest,

    Sand,
    Rock,
    CutTrees,
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

        let mut height_noise = noise::ComplexNoise::new(4);
        let mut tree_noise = noise::ComplexNoise::new(3);

        for n in 0..data_size {
            let x = (n % width) as i32;
            let y = (n / width) as i32;

            let height_noise_value = height_noise.value_at(x, y);
            let mut ground_type = GroundType::Grass;
            if height_noise_value < -0.2 {
                ground_type = GroundType::Water;
            } else if height_noise_value < -0.1 {
                ground_type = GroundType::Sand;
            } else if height_noise_value > 0.5 {
                ground_type = GroundType::Rock;
            }

            if ground_type == GroundType::Grass && tree_noise.value_at(x, y) < -0.1 {
                ground_type = GroundType::Forest;
            }
            /*
            else if ground_type == GroundType::Grass && tree_noise.value_at(x, y) < -0.0 {
                ground_type = GroundType::CutTrees;
            }
            */

            new_map.data[n as usize] = ground_type;
        }

        /*
        let mut water_noise1 = noise::PerlinNoise::new(3);
        let mut water_noise2 = noise::PerlinNoise::new(6);
        let mut water_noise3 = noise::PerlinNoise::new(9);

        let mut tree_noise1 = noise::PerlinNoise::new(2);
        let mut tree_noise2 = noise::PerlinNoise::new(4);
        let mut tree_noise3 = noise::PerlinNoise::new(8);

        for n in 0..data_size {
            let x = n % width;
            let y = n / width;

            let noise_value = 
                tree_noise1.value_at(x as i32, y as i32) * 1.0 +
                tree_noise2.value_at(x as i32, y as i32) * 2.0 +
                tree_noise3.value_at(x as i32, y as i32) * 4.0;

            if noise_value < -0.7 {
                new_map.data[n as usize] = GroundType::Forest;
            }

            let noise_value = 
                water_noise1.value_at(x as i32, y as i32) * 1.0 +
                water_noise2.value_at(x as i32, y as i32) * 2.0 +
                water_noise3.value_at(x as i32, y as i32) * 4.0;

            if noise_value < -0.7 {
                new_map.data[n as usize] = GroundType::Water;
            }
        }

        */

        return new_map;
    }

    pub fn set(&mut self, x: i32, y: i32, ground_type: GroundType) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }
        let index: usize = (x as u32 + (y as u32) * self.width) as usize;
        self.data[index] = ground_type;
    }

    pub fn line_of_sight(&self, point_1: &point::Point, point_2: &point::Point) -> bool {
        let distance_vec = point_2.dist_to(point_1);

        let mut current_point = point::Point::new(point_1.x, point_1.y);

        let mut counter = 0;

        let mut x: i32 = current_point.x as i32;
        let mut y: i32 = current_point.y as i32;

        loop {
            if x == point_2.x as i32 && y == point_2.y as i32 {
                return true;
            }

            if !self.point_moveable((x, y)) {
                return false;
            }
            let mut x_diff: f32 = current_point.x - x as f32;
            let mut y_diff: f32 = current_point.y - y as f32;
            if distance_vec.x > 0.0 {
                x_diff = 1.0 - x_diff
            }
            if distance_vec.y > 0.0 {
                y_diff = 1.0 - y_diff
            }

            let x_normalized_vec = distance_vec.x_normalized().multiplied(x_diff);
            let y_normalized_vec = distance_vec.y_normalized().multiplied(y_diff);

            if x_normalized_vec.length() < y_normalized_vec.length() {
                current_point.add(&x_normalized_vec);
                if distance_vec.x < 0.0 {
                    x -= 1;
                } else {
                    x += 1;
                }
            } else {
                current_point.add(&y_normalized_vec);
                if distance_vec.y < 0.0 {
                    y -= 1;
                } else {
                    y += 1;
                }
            }

            counter += 1;
            if counter > 1000 {
                println!("THIS SHOULD NOT HAPPEN!!! PANIC!");
                break;
            }
        }

        true
    }

    pub fn line_of_sight_fat(&self, point_1: &point::Point, point_2: &point::Point, radius: f32) -> bool {
        let normal_vec = point_2.dist_to(point_1).normalized();
        let ninety_degree_vec = point::Vector::new(normal_vec.y, -normal_vec.x);

        if point_1.x as i32 == point_2.x as i32 && point_1.y as i32 == point_2.y as i32 &&
            self.point_moveable((point_1.x as i32, point_1.y as i32)) {
                return true;
        }

        return 
            self.line_of_sight(
                &point_1
                .added(&ninety_degree_vec.multiplied(radius)),
                &point_2
                .added(&ninety_degree_vec.multiplied(radius)),
            ) &&
            self.line_of_sight(
                &point_1
                .added(&ninety_degree_vec.multiplied(-radius)),
                &point_2
                .added(&ninety_degree_vec.multiplied(-radius)),
            )
    }

    pub fn get_at(&self, x: i32, y: i32) -> GroundType {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return GroundType::Empty
        }
        let index: usize = (x as u32 + (y as u32) * self.width) as usize;
        return self.data[index];
    }

    pub fn closest_moveable_point(&self, x: i32, y: i32) -> (i32, i32) {
        for i in 1..20 {
            if self.point_moveable((x + i, y)) {return (x + i, y);};
            if self.point_moveable((x, y + i)) {return (x, y + i);};
            if self.point_moveable((x - i, y)) {return (x - i, y);};
            if self.point_moveable((x, y - i)) {return (x, y - i);};
        }

        (x, y)
    }

    pub fn point_moveable(&self, point: (i32, i32)) -> bool {
        let ground_type = self.get_at(point.0, point.1);
        ground_type == GroundType::Grass ||
            ground_type == GroundType::Sand ||
            ground_type == GroundType::Rock ||
            ground_type == GroundType::CutTrees
    }
}

