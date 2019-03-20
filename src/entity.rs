use rand::{Rng};

use super::point;
use super::map;
use super::projectile::Projectile;


#[derive(Clone)]
pub enum ResourceType {
    Wood,
    Gold,
}


#[derive(Clone)]
pub enum Task {
    Idle,

    Move {point: point::Point},
    AttackMove {point: point::Point},

    Gather {point: point::Point, resource_type: ResourceType},
}


pub struct Entity {
    location: point::Point,
    id: u32,

    // Storing pathfinding information
    waypoint_index: u32,
    path: Vec<point::Point>,

    // For drawing
    orientation: u32,

    // For fighting
    team_id: u32,
    hp: i32,
    cooldown: u32,
    closest_seen_enemy_point: Option<point::Point>,

    // For ai handling
    task: Task,
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
            hp: 200,
            cooldown: 0,
            closest_seen_enemy_point: None,

            task: Task::Idle,
        }
    }

    // Getter methods
    pub fn orientation(&self) -> u32 { self.orientation }
    pub fn path(&self) -> &Vec<point::Point> { &self.path }
    pub fn closest_seen_enemy_point(&self) -> &Option<point::Point> { &self.closest_seen_enemy_point }
    pub fn location(&self) -> &point::Point { &self.location }
    pub fn id(&self) -> u32 { self.id }
    pub fn team_id(&self) -> u32 { self.team_id }
    pub fn hp(&self) -> i32 { self.hp}

    pub fn max_hp(&self) -> i32 {
        200
    }

    pub fn alive(&self) -> bool {
        self.hp > 0
    }

    pub fn take_hit(&mut self, amount: u32) {
        self.hp -= amount as i32;
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

        let mut angle = vector.angle();

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


        let mut seeing_enemy = false;
        match &self.closest_seen_enemy_point {
            Some(_point) => {
                seeing_enemy = true;
            },
            _ => {
            }
        }
        let mut moving = false;
        let mut attack_moving = false;

        match self.task {
            Task::Move{..} => {
                moving= true;
            },
            Task::AttackMove{..} => {
                attack_moving = true;
            },
            _ => {}
        }

        // Attack
        if seeing_enemy && !moving {
            return self.attack_enemy()
        }

        // Move
        if moving || attack_moving {
            self.follow_path_finding(map);
            return None
        }

        None
    }

    fn attack_enemy(&mut self) -> Option<Projectile> {
        match &self.closest_seen_enemy_point {
            Some(point) => {
                let vector_to_enemy = self.location.dist_to(point);
                // Move towards
                if vector_to_enemy.length() > 10.0 {
                    self.move_vector(&vector_to_enemy.normalized().multiplied(-0.04), true);
                }
                // Shoot
                else {
                    if self.cooldown == 0 {
                        self.cooldown = 45;
                        return Some(Projectile::new(&self.location, point))
                    }
                }
            }, _ => {
                println!("This should not happen");
            }
        }
        None
    }

    fn follow_path_finding(&mut self, map: &map::Map) -> bool {
        if self.path.len() == 0 {
            return false;
        }

        // Check if we can see further
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

        // If we cant see waypoint, loop backwards
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
                    self.order_stop();
                } else {
                    let normalized = vec_to_waypoint.normalized();
                    self.move_vector(&normalized.negated().multiplied(0.04), true);
                }
            },
            _ => {
                println!("HMmm I dont think this shoudl ever happen");
            }
        }
        return true;
    }

    pub fn set_path(&mut self, path: Vec<point::Point>, task: Task) {
        self.waypoint_index = 0;
        self.path = path;

        self.task = task;
    }

    pub fn order_stop(&mut self) {
        self.path = Vec::new();
        self.waypoint_index = 0;

        self.task = Task::Idle;
    }

    pub fn clear_interaction_data(&mut self) {
        self.closest_seen_enemy_point = None;
    }

    pub fn move_vector(&mut self, vector: &point::Vector, update_orientation: bool) {
        self.location.x += vector.x;
        self.location.y += vector.y;
        if update_orientation {
            self.set_orientation_from_vector(&vector.negated());
        }
    }

    pub fn interact_with(&mut self, other: &Entity, map: &map::Map) {
        let max_dist = 0.55;

        let dist_vect = self.location.dist_to(&other.location);
        let distance = dist_vect.length();

        // Storing closest seen enemy position
        if 
            other.team_id != self.team_id &&
            distance < 20.0
        {
            match &mut self.closest_seen_enemy_point {
                Some(point) => {
                    let currently_stored_points_distance = self.location.dist_to(&point).length();
                    if 
                        distance < currently_stored_points_distance  &&
                        map.line_of_sight(&self.location, &other.location)
                    {
                        self.closest_seen_enemy_point = Some(
                            point::Point::new(other.location.x, other.location.y)
                        );
                    }
                },
                _ => {
                    if map.line_of_sight(&self.location, &other.location) {
                        self.closest_seen_enemy_point = Some(
                            point::Point::new(other.location.x, other.location.y)
                        );
                    }
                }
            }
        }

        // Moving if another unit is too close
        if distance == 0.0 {
            // Two units at exact same location. Move by random value
            let mut randomizer = rand::thread_rng();
            let x_value: f32 = randomizer.gen_range(-0.1, 0.1);
            let y_value: f32 = randomizer.gen_range(-0.1, 0.1);
            self.move_vector(&point::Vector::new(x_value, y_value), false);
        } else if distance < max_dist {
            let move_vect = dist_vect.normalized().multiplied((max_dist - distance) * 0.3);
            self.move_vector(&move_vect, false);
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

