
use super::point;
use super::map;
use std::collections::VecDeque;


pub struct Entity {
    pub location: point::Point,
    pub id: u32,
    pub waypoint: Option<point::Point>,
    pub path: VecDeque<point::Point>,
}

impl Entity {
    pub fn new(x: f32, y: f32, id: u32) -> Entity {
        Entity {
            location: point::Point::new(x, y),
            id: id,
            waypoint: None,
            path: VecDeque::new()
        }
    }

    pub fn add_force_vect(&mut self, force_vect: &point::Vector) {
        self.location.add(&force_vect);
    }

    pub fn is_inside(&self, corner_1: (f32, f32), corner_2: (f32, f32)) -> bool {
        let min_x = corner_1.0.min(corner_2.0);
        let max_x = corner_1.0.max(corner_2.0);
        let min_y = corner_1.1.min(corner_2.1);
        let max_y = corner_1.1.max(corner_2.1);

        return self.location.x > min_x && self.location.x < max_x && self.location.y > min_y && self.location.y < max_y
    }

    pub fn ai_stuff(&mut self, map: &map::Map) {

        let mut matched = false;
        match self.path.front() {
            Some(point) => {
                if map.line_of_sight(&self.location, point) {
                    // println!("FOUND LINE OF SIGHT STUFF");
                    matched = true;
                    self.waypoint = Some(point::Point::new(point.x, point.y));
                }
            },
            _ => {}
        }
        if matched {
            self.path.pop_front();
        }

        match &self.waypoint {
            Some(point) => {
                self.location.add(&self.location.dist_vect(point).normalized().negated().multiplied(0.04));
            }
            _ => {}
        }
    }

    pub fn set_path(&mut self, path: VecDeque<point::Point>) {
        self.path = path;
        // println!("Entity at {:?} got new path queue {:?}", self.location, self.path);
    }

    pub fn interact_with(&self, other: &Entity) -> Option<point::Vector> {
        let max_dist = 0.55;

        let dist_vect = self.location.dist_vect(&other.location);
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
        let int_loc = self.location.as_int();
        let loc_x_rem = self.location.x % 1.0;
        let loc_y_rem = self.location.y % 1.0;
        let abs_loc_x_rem = (loc_x_rem - 0.5).abs();
        let abs_loc_y_rem = (loc_y_rem - 0.5).abs();

        // Inside box?
        if !map.point_moveable(int_loc) {
            self.location = point::Point::new(
                self.location.x + 0.0,
                self.location.y + 1.0 
            )
        }

        let treshold = 0.25;
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

