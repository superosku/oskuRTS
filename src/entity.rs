use rand::{Rng};

use super::point;
use super::map;


pub struct Projectile {
    pub location: point::Point,
    pub end_point: point::Point,
}


impl Projectile {
    pub fn new(start_point: &point::Point, end_point: &point::Point) -> Projectile {
        Projectile {
            location: point::Point::new(start_point.x, start_point.y),
            end_point: point::Point::new(end_point.x, end_point.y),
        }
    }

    pub fn increment(&mut self) {
        let dist_vect = self.location.dist_to(&self.end_point);
        if dist_vect.length() < 0.1 {
            self.location.x = self.end_point.x;
            self.location.y = self.end_point.y;
        } else {
            self.location = self.location.added(&self.location.dist_to(&self.end_point).normalized().multiplied(-0.2));
        }
    }

    pub fn at_location(&self) -> bool {
        return self.location.x == self.end_point.x && self.location.y == self.end_point.y;
    }
}


pub struct Entity {
    pub location: point::Point,
    pub id: u32,

    pub waypoint_index: u32,
    pub path: Vec<point::Point>,

    pub orientation: u32,

    pub team_id: u32,
    pub hp: i32,
    pub cooldown: u32,
    pub closest_seen_enemy_point: Option<point::Point>,
}

impl Entity {
    pub fn new(x: f32, y: f32, id: u32, team_id: u32) -> Entity {
        Entity {
            location: point::Point::new(x, y),
            id: id,

            waypoint_index: 0,
            path: Vec::new(),

            orientation: id % 8,

            team_id: team_id,
            hp: 100,
            cooldown: 0,
            closest_seen_enemy_point: None,
        }
    }

    pub fn alive(&self) -> bool {
        self.hp > 0
    }

    pub fn take_hit(&mut self, amount: u32) {
        self.hp -= amount as i32;
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

    pub fn ai_stuff(&mut self, map: &map::Map) -> Option<Projectile> {
        // Optionally returns projectile if such action was made
        
        if self.cooldown > 0 {
            self.cooldown -= 1;
        }

        let followed_path = self.follow_path_finding(map);

        if !followed_path {
            // Shoot possible enemy
            match &self.closest_seen_enemy_point {Some(point) => {
                let vector_to_enemy = self.location.dist_to(point);
                if vector_to_enemy.length() > 10.0 {
                    self.location.add(&vector_to_enemy.normalized().multiplied(-0.04));
                } else {
                    if self.cooldown == 0 {
                        self.cooldown = 45;
                        return Some(Projectile::new(&self.location, point))
                    }
                }
            }, _ => {}}
        }

        return None
    }

    fn follow_path_finding(&mut self, map: &map::Map) -> bool {
        if self.path.len() == 0 {
            return false;
        }

        // Do this until next point is reachable
        'outer1: loop {
            match self.path.get((self.waypoint_index + 1) as usize) {
                Some(point) => {
                    if map.line_of_sight_fat(&self.location, point, 0.25) {
                        self.waypoint_index += 1;
                    } else {
                        break 'outer1;
                    }
                },
                _ => {break 'outer1}
            }
        }
        // Back off in point queue if point is not reachable anymore
        'outer2: loop {
            if self.waypoint_index == 0 {
                break 'outer2;
            }
            match self.path.get(self.waypoint_index as usize) {
                Some(point) => {
                    if !map.line_of_sight_fat(&self.location, point, 0.25) {
                        self.waypoint_index -= 1;
                    } else {
                        break 'outer2;
                    }
                },
                _ => {break 'outer2}
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
        return true;
    }

    pub fn set_path(&mut self, path: Vec<point::Point>) {
        self.waypoint_index = 0;
        self.path = path;
    }

    pub fn order_stop(&mut self) {
        self.path = Vec::new();
        self.waypoint_index = 0;
    }

    pub fn clear_interaction_data(&mut self) {
        self.closest_seen_enemy_point = None;
    }

    pub fn interact_with(&mut self, other: &Entity, map: &map::Map) {
        let max_dist = 0.55;

        let dist_vect = self.location.dist_to(&other.location);
        let distance = dist_vect.length();

        // Storing closest seen enemy position
        if 
            other.team_id != self.team_id &&
            map.line_of_sight(&self.location, &other.location)  &&
            distance < 20.0
        {
            match &mut self.closest_seen_enemy_point {
                Some(point) => {
                    let currently_stored_points_distance = self.location.dist_to(&point).length();
                    if distance < currently_stored_points_distance {
                        // point.x = other.location.x;
                        // point.y = other.location.y;
                        self.closest_seen_enemy_point = Some(point::Point::new(other.location.x, other.location.y));
                    }
                },
                _ => {
                    self.closest_seen_enemy_point = Some(point::Point::new(other.location.x, other.location.y));
                }
            }
        }

        // Moving if another unit is too close
        if distance == 0.0 {
            // Two units at exact same location. Move by random value
            let mut randomizer = rand::thread_rng();
            let x_value: f32 = randomizer.gen_range(-0.1, 0.1);
            let y_value: f32 = randomizer.gen_range(-0.1, 0.1);
            self.location = self.location.added(&point::Vector::new(x_value, y_value));
        } else if distance < max_dist {
            let move_vect = dist_vect.normalized().multiplied((max_dist - distance) * 0.3);
            self.location = self.location.added(&move_vect);
        }
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

