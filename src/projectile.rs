use super::point::Point;
use super::map;


use super::binary_helpers::Binaryable;
use super::binary_helpers;


pub struct Projectile {
    location: Point,
    start_point: Point,
    end_point: Point,
    angle: f32,
}


impl Binaryable for Projectile {
    fn as_binary(&self) -> Vec<u8> {
        let mut binary_data: Vec<u8> = Vec::new();

        binary_data.extend(binary_helpers::f32_as_bytes(self.location.x));
        binary_data.extend(binary_helpers::f32_as_bytes(self.location.y));
        binary_data.extend(binary_helpers::f32_as_bytes(self.start_point.x));
        binary_data.extend(binary_helpers::f32_as_bytes(self.start_point.y));
        binary_data.extend(binary_helpers::f32_as_bytes(self.end_point.x));
        binary_data.extend(binary_helpers::f32_as_bytes(self.end_point.y));
        binary_data.extend(binary_helpers::f32_as_bytes(self.angle));

        binary_data
    }

    fn from_binary(binary_data: Vec<u8>) -> Projectile {
        let (location_x, binary_data) = binary_helpers::pop_f32(binary_data);
        let (location_y, binary_data) = binary_helpers::pop_f32(binary_data);
        let (start_point_x, binary_data) = binary_helpers::pop_f32(binary_data);
        let (start_point_y, binary_data) = binary_helpers::pop_f32(binary_data);
        let (end_point_x, binary_data) = binary_helpers::pop_f32(binary_data);
        let (end_point_y, binary_data) = binary_helpers::pop_f32(binary_data);
        let (angle, binary_data) = binary_helpers::pop_f32(binary_data);
        Projectile {
            location: Point::new(location_x, location_y),
            start_point: Point::new(start_point_x, start_point_y),
            end_point: Point::new(end_point_x, end_point_y),
            angle: angle
        }
    }
}


impl Projectile {
    pub fn new(start_point: &Point, end_point: &Point) -> Projectile {
        let vector = end_point.dist_to(start_point);
        let angle = vector.angle();
        Projectile {
            location: Point::new(start_point.x, start_point.y),
            start_point: Point::new(start_point.x, start_point.y),
            end_point: Point::new(end_point.x, end_point.y),
            angle: angle,
        }
    }

    pub fn location(&self) -> &Point { &self.location }
    pub fn angle(&self) -> f32 { self.angle}

    pub fn increment(&mut self) {
        let dist_vect = self.location.dist_to(&self.end_point);
        if dist_vect.length() < 0.11 {
            self.location.x = self.end_point.x;
            self.location.y = self.end_point.y;
        } else {
            self.location = self.location.added(&self.location.dist_to(&self.end_point).normalized().multiplied(-0.2));
        }
    }

    pub fn at_location(&self) -> bool {
        return self.location.x == self.end_point.x && self.location.y == self.end_point.y;
    }

    pub fn get_height(&self) -> f32 {
        let total_length = self.end_point.dist_to(&self.start_point).length();
        let length_remaining = self.end_point.dist_to(&self.location).length();

        let progress = length_remaining / total_length;

        let x = 2.0 * progress - 1.0;
        let y = -x * x + 1.0;
        
        y * total_length
    }
}
