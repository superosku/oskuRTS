
use super::point;
use super::map;

// #[derive(Copy, Clone)]
pub struct Entity {
    pub location: point::Point
}

impl Entity {
    pub fn new(x: f32, y: f32) -> Entity {
        Entity {
            location: point::Point::new(x, y)
        }
    }

    pub fn add_force_vect(&mut self, force_vect: &point::Vector) {
        self.location.add(&force_vect);
    }

    pub fn interact_with(&self, other: &Entity) -> Option<point::Vector> {
        let max_dist = 0.55;

        let dist_vect = self.location.dist_vect(&other.location);
        let distance = dist_vect.length();
        if distance == 0.0 {
            return None
        } else if distance < max_dist {
            let move_vect = dist_vect.normalized().multiplied((max_dist - distance) * 0.1);
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
        // TODO: Replace this with findin closest open space
        if !map.point_moveable(int_loc) {
            if abs_loc_x_rem > abs_loc_y_rem {
                if loc_x_rem > 0.5 {
                    self.location = self.location.add_vect(&point::Vector::new( 0.1,  0.0));
                } else {
                    self.location = self.location.add_vect(&point::Vector::new(-0.1,  0.0));
                }
            } else {
                if loc_y_rem > 0.5 {
                    self.location = self.location.add_vect(&point::Vector::new( 0.0,  0.1));
                } else {
                    self.location = self.location.add_vect(&point::Vector::new( 0.0, -0.1));
                }
            }
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

