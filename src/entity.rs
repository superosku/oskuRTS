
use super::point;
use super::map;
use std::collections::VecDeque;


pub struct Entity {
    pub location: point::Point,
    pub id: u32,

    // pub waypoint: Option<point::Point>,
    pub waypoint_index: u32,
    pub path: Vec<point::Point>,

    pub orientation: u32,
}

impl Entity {
    pub fn new(x: f32, y: f32, id: u32) -> Entity {
        Entity {
            location: point::Point::new(x, y),
            id: id,

            // waypoint: None,
            waypoint_index: 0,
            path: Vec::new(),

            orientation: id % 8,
        }
    }

    pub fn add_force_vect(&mut self, force_vect: &point::Vector) {
        self.location.add(&force_vect);
    }

    pub fn is_inside(&self, corner_1: (f32, f32), corner_2: (f32, f32)) -> bool {
        let min_x = corner_1.0.min(corner_2.0) - 0.25;
        let min_y = corner_1.1.min(corner_2.1) - 0.25;
        let max_x = corner_1.0.max(corner_2.0) + 0.25;
        let max_y = corner_1.1.max(corner_2.1) + 0.25;

        self.location.x > min_x && self.location.x < max_x && self.location.y > min_y && self.location.y < max_y
    }

    fn set_orientation_from_vector(&mut self, vector: &point::Vector) {
        let quarter_pi = 0.78539816;
        let eight_pi = 0.39269908;

        let mut angle = vector.x.atan2(vector.y);
        angle += quarter_pi * 4.0;
        angle -= eight_pi;

        let mut orientation_guess = 0;
        loop {
            if angle < 0.0 {
                break;
            }
            
            angle -= quarter_pi;
            orientation_guess += 1;
        }
        self.orientation = orientation_guess % 8;
    }

    pub fn get_waypoint(&self) -> Option<&point::Point> {
        match self.path.get(self.waypoint_index as usize) {
            Some(point) => {return Some(&point)}
            _ => {}
        }
        return None
    }

    pub fn ai_stuff(&mut self, map: &map::Map) {
        // Set the first waypoint if not yet set
        // So basically this: if (not self.waypoint) and (self.path)
        
        if self.path.len() == 0 {
            return;
        }

        // Do this until next point is reachable
        'outer: loop {
            match self.path.get((self.waypoint_index + 1) as usize) {
                Some(point) => {
                    if map.line_of_sight_fat(&self.location, point, 0.25) {
                        self.waypoint_index += 1;
                    } else {
                        break 'outer;
                    }
                },
                _ => {break 'outer}
            }
        }
        // Back off in point queue if point is not reachable anymore
        'outer: loop {
            if self.waypoint_index == 0 {
                break 'outer;
            }
            match self.path.get(self.waypoint_index as usize) {
                Some(point) => {
                    if !map.line_of_sight_fat(&self.location, point, 0.25) {
                        self.waypoint_index -= 1;
                    } else {
                        break 'outer;
                    }
                },
                _ => {break 'outer}
            }
        }

        // Move towards waypoint AND stop moving if end reached
        match self.path.get(self.waypoint_index as usize) {
            Some(point) => {
                let vec_to_waypoint = self.location.dist_to(point);
                if vec_to_waypoint.length() < 0.1 && self.path.len() - 1 == self.waypoint_index as usize {
                    self.location = point::Point::new(point.x, point.y);
                    self.set_path(Vec::new());
                } else {
                    let normalized = vec_to_waypoint.normalized();
                    self.location.add(&normalized.negated().multiplied(0.04));
                    self.set_orientation_from_vector(&normalized);
                }

            },
            _ => {
                println!("HMmm I dont think this shoudl ever happen");
            }
        }
    }

    pub fn set_path(&mut self, path: Vec<point::Point>) {
        self.waypoint_index = 0;
        self.path = path;
    }

    pub fn interact_with(&self, other: &Entity) -> Option<point::Vector> {
        let max_dist = 0.55;

        let dist_vect = self.location.dist_to(&other.location);
        let distance = dist_vect.length();
        if distance == 0.0 {
            return None
        } else if distance < max_dist {
            let move_vect = dist_vect.normalized().multiplied((max_dist - distance) * 0.3);
            return Some(move_vect);
        }
        return None
    }

    pub fn interact_with_map(&mut self, map: &map::Map) {
        let treshold = 0.25;

        let int_loc = self.location.as_int();
        let loc_x_rem = self.location.x % 1.0;
        let loc_y_rem = self.location.y % 1.0;
        let abs_loc_x_rem = (loc_x_rem - 0.5).abs();
        let abs_loc_y_rem = (loc_y_rem - 0.5).abs();

        // Inside box
        if !map.point_moveable(int_loc) {
            let closest_moveable_point = map.closest_moveable_point(int_loc.0, int_loc.1);
            self.location.x = closest_moveable_point.0 as f32 + 0.5;
            self.location.y = closest_moveable_point.1 as f32 + 0.5;
        }

        // Sides
        if loc_x_rem < treshold && !map.point_moveable((int_loc.0 - 1, int_loc.1)) {
            self.location.x = self.location.x as i32 as f32 + treshold;
        }
        if loc_x_rem > (1.0 - treshold) && !map.point_moveable((int_loc.0 + 1, int_loc.1)) {
            self.location.x = self.location.x as i32 as f32 + 1.0 - treshold;
        }
        if loc_y_rem < treshold && !map.point_moveable((int_loc.0, int_loc.1 - 1)) {
            self.location.y = self.location.y as i32 as f32 + treshold;
        }
        if loc_y_rem > (1.0 - treshold) && !map.point_moveable((int_loc.0, int_loc.1 + 1)) {
            self.location.y = self.location.y as i32 as f32 + 1.0 - treshold;
        }

        // Corners
        if loc_x_rem < treshold && loc_y_rem < treshold && !map.point_moveable((int_loc.0 - 1, int_loc.1 - 1)) {
            if abs_loc_x_rem > abs_loc_y_rem {
                self.location.y = self.location.y as i32 as f32 + treshold;
            } else {
                self.location.x = self.location.x as i32 as f32 + treshold;
            }
        }
        if loc_x_rem > (1.0 - treshold) && loc_y_rem < treshold && !map.point_moveable((int_loc.0 + 1, int_loc.1 - 1)) {
            if abs_loc_x_rem > abs_loc_y_rem {
                self.location.y = self.location.y as i32 as f32 + treshold;
            } else {
                self.location.x = self.location.x as i32 as f32 + 1.0 - treshold;
            }
        }
        if loc_x_rem < treshold && loc_y_rem > (1.0 - treshold) && !map.point_moveable((int_loc.0 - 1, int_loc.1 + 1)) {
            if abs_loc_x_rem > abs_loc_y_rem {
                self.location.y = self.location.y as i32 as f32 + 1.0 - treshold;
            } else {
                self.location.x = self.location.x as i32 as f32 + treshold;
            }
        }
        if loc_x_rem > (1.0 - treshold) && loc_y_rem > (1.0 - treshold) && !map.point_moveable((int_loc.0 + 1, int_loc.1 + 1)) {
            if abs_loc_x_rem > abs_loc_y_rem {
                self.location.y = self.location.y as i32 as f32 + 1.0 - treshold;
            } else {
                self.location.x = self.location.x as i32 as f32 + 1.0 - treshold;
            }
        }
    }
}

