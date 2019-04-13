use std::mem::transmute;
use super::point;
use super::noise;
use super::binary_helpers::Binaryable;
use super::binary_helpers;

#[derive(Copy, Clone, PartialEq)]
pub enum GroundType {
    Empty,

    Grass,
    Water,
    // Forest,

    Sand,
    Rock,
    // CutTrees,
}

#[derive(Copy, Clone, PartialEq)]
pub enum SecondLevelType {
    Empty,

    Building,

    Tree,
    CutTree,
}

pub struct Map {
    height: u32,
    width: u32,
    data: Vec<GroundType>,
    second_level_data: Vec<SecondLevelType>
}


impl Binaryable for Map {
    fn as_binary(&self) -> Vec<u8> {
        let mut binary_data: Vec<u8> = Vec::new();

        binary_data.extend(binary_helpers::u32_as_bytes(self.width));
        binary_data.extend(binary_helpers::u32_as_bytes(self.height));

        let map_data: Vec<u8> = self.data.iter().map(|i| *i as u8).collect();
        binary_data.extend(map_data);
        let map_data: Vec<u8> = self.second_level_data.iter().map(|i| *i as u8).collect();
        binary_data.extend(map_data);

        binary_data
    }

    fn from_binary(binary_data: Vec<u8>) -> Map {
        let (width, binary_data) = binary_helpers::pop_u32(binary_data);
        let (height, binary_data) = binary_helpers::pop_u32(binary_data);

        let data_size = width * height;
        let mut new_map = Map::new(width, height);

        let (first_level_data, second_level_data) = binary_helpers::pop_bytes_from_vec(binary_data, data_size);

        for n in 0..data_size {
            new_map.data[n as usize] = match first_level_data.get(n as usize) {
                Some(value) => {
                    match value {
                        0 => GroundType::Empty,
                        1 => GroundType::Grass,
                        2 => GroundType::Water,
                        3 => GroundType::Sand,
                        4 => GroundType::Rock,
                        _ => {
                            println!("Unhandled data in map loading");
                            GroundType::Empty
                        },
                    }
                    // GroundType::Grass
                },
                _ => {
                    println!("This should not happen");
                    GroundType::Empty
                },
            };
            new_map.second_level_data[n as usize] = match second_level_data.get(n as usize) {
                Some(value) => {
                    match value {
                        0 => SecondLevelType::Empty,
                        1 => SecondLevelType::Building,
                        2 => SecondLevelType::Tree,
                        3 => SecondLevelType::CutTree,
                        _ => {
                            println!("Unhandled data in map loading");
                            SecondLevelType::Empty
                        },
                    }
                },
                _ => {
                    println!("This should not happen");
                    SecondLevelType::Empty
                },
            };
        }

        new_map
    }
}


impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        let data_size: u32 = width * height;
        Map {
            height: height,
            width: width,
            data: vec![GroundType::Grass; (data_size) as usize],
            second_level_data: vec![SecondLevelType::Empty; (data_size) as usize],
        }
    }

    pub fn new_random(width: u32, height: u32) -> Map {
        let data_size = width * height;
        let mut new_map = Map::new(width, height);

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

            if ground_type != GroundType::Water && tree_noise.value_at(x, y) < -0.1 {
                new_map.second_level_data[n as usize] = SecondLevelType::Tree;
                // ground_type = GroundType::Forest;
            }

            new_map.data[n as usize] = ground_type;
        }

        return new_map;
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_second_layer(&mut self, x: i32, y: i32, second_type: SecondLevelType) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }
        let index: usize = (x as u32 + (y as u32) * self.width) as usize;
        self.second_level_data[index] = second_type;
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

    pub fn coord_to_index(&self, x:i32, y:i32) -> usize {
        let index: usize = (x as u32 + (y as u32) * self.width) as usize;
        index
    }

    pub fn get_at_second_level(&self, x: i32, y: i32) -> SecondLevelType {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return SecondLevelType::Empty
        }
        let index: usize = self.coord_to_index(x, y);
        return self.second_level_data[index];
    }

    pub fn get_at(&self, x: i32, y: i32) -> GroundType {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return GroundType::Empty
        }
        let index: usize = self.coord_to_index(x, y);
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
        let second_level_type = self.get_at_second_level(point.0, point.1);

        let base_moveable = ground_type == GroundType::Grass || ground_type == GroundType::Sand || ground_type == GroundType::Rock;
        let second_level_moveable = second_level_type == SecondLevelType::Empty || second_level_type == SecondLevelType::CutTree;

        base_moveable && second_level_moveable
    }
}

