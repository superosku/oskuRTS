use super::point;
use super::map;
use super::projectile::Projectile;

use super::binary_helpers::{Binaryable, u32_as_bytes, i32_as_bytes, f32_as_bytes};
use super::binary_helpers;


// #[derive(Clone)]
#[derive(Copy, Clone, PartialEq)]
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


impl Binaryable for Task {
    fn as_binary(&self) -> Vec<u8> {
        let mut binary_data: Vec<u8> = Vec::new();
        match self {
            Task::Idle => {
                binary_data.push(0u8);
            },
            Task::Move { point } => {
                binary_data.push(1u8);
                binary_data.extend(f32_as_bytes(point.x));
                binary_data.extend(f32_as_bytes(point.y));
            },
            Task::AttackMove { point } => {
                binary_data.push(2u8);
                binary_data.extend(f32_as_bytes(point.x));
                binary_data.extend(f32_as_bytes(point.y));
            },
            Task::Gather { point, resource_type } => {
                binary_data.push(3u8);
                binary_data.extend(f32_as_bytes(point.x));
                binary_data.extend(f32_as_bytes(point.y));
                binary_data.push(*resource_type as u8);
            }
        }
        binary_data
    }

    fn from_binary(binary_data: Vec<u8>) -> Task {
        let (task_type, binary_data) = binary_helpers::pop_u8(binary_data);
        match task_type {
            0 => Task::Idle,
            1 => {
                let (point_x, binary_data) = binary_helpers::pop_f32(binary_data);
                let (point_y, binary_data) = binary_helpers::pop_f32(binary_data);
                Task::Move {
                    point: point::Point::new(point_x, point_y)
                }
            },
            2 => {
                let (point_x, binary_data) = binary_helpers::pop_f32(binary_data);
                let (point_y, binary_data) = binary_helpers::pop_f32(binary_data);
                Task::AttackMove {
                    point: point::Point::new(point_x, point_y)
                }
            },
            3 => {
                let (point_x, binary_data) = binary_helpers::pop_f32(binary_data);
                let (point_y, binary_data) = binary_helpers::pop_f32(binary_data);
                let (resource_type, binary_data) = binary_helpers::pop_u8(binary_data);
                Task::Gather {
                    point: point::Point::new(point_x, point_y),
                    resource_type: ResourceType::Wood,
                }
            },
            _ => {
                println!("This should not happen, unknown Task id");
                Task::Idle
            }
        }
    }
}


#[derive(Clone)]
pub enum EntityType {
    Peasant,
    Ranged,
    Meelee
}


pub struct Entity {
    location: point::Point,
    id: u32,

    entity_type: EntityType,

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
    closest_seen_enemy_id: Option<u32>,

    // For ai handling
    task: Task,
}


impl Binaryable for Entity {
    fn as_binary(&self) -> Vec<u8> {
        let mut binary_data: Vec<u8> = Vec::new();

        binary_data.extend(f32_as_bytes(self.location.x));
        binary_data.extend(f32_as_bytes(self.location.y));
        binary_data.extend(u32_as_bytes(self.id));
        binary_data.push(self.entity_type.clone() as u8);
        binary_data.extend(u32_as_bytes(self.waypoint_index));
        binary_data.extend(u32_as_bytes(self.orientation));
        binary_data.extend(u32_as_bytes(self.team_id));
        binary_data.extend(i32_as_bytes(self.hp));
        binary_data.extend(u32_as_bytes(self.cooldown));

        let mut path_binary: Vec<u8> = Vec::new();
        for path_point in self.path.iter() {
            path_binary.extend(f32_as_bytes(path_point.x));
            path_binary.extend(f32_as_bytes(path_point.y));
        }
        let path_binary_length: u32 = path_binary.len() as u32;
        binary_data.extend(u32_as_bytes(path_binary_length));
        binary_data.extend(path_binary);

        match self.closest_seen_enemy_point {
            Some(point) => {
                binary_data.push(0u8);
                binary_data.extend(f32_as_bytes(point.x));
                binary_data.extend(f32_as_bytes(point.y));
            },
            None => {
                binary_data.push(1u8);
                binary_data.extend(u32_as_bytes(0u32));
                binary_data.extend(u32_as_bytes(0u32));
            }
        }
        binary_data.extend(self.task.as_padded_binary());

        binary_data
    }

    fn from_binary(binary_data: Vec<u8>) -> Entity {
        let (location_x, binary_data) = binary_helpers::pop_f32(binary_data);
        let (location_y, binary_data) = binary_helpers::pop_f32(binary_data);
        let (id, binary_data) = binary_helpers::pop_u32(binary_data);
        let (entity_type, binary_data) = binary_helpers::pop_u8(binary_data);
        let (waypoint_index, binary_data) = binary_helpers::pop_u32(binary_data);
        let (orientation, binary_data) = binary_helpers::pop_u32(binary_data);
        let (team_id, binary_data) = binary_helpers::pop_u32(binary_data);
        let (hp, binary_data) = binary_helpers::pop_i32(binary_data);
        let (cooldown, binary_data) = binary_helpers::pop_u32(binary_data);
        let (mut path_binary_data, binary_data) = binary_helpers::pop_padded(binary_data);
        let (closest_seen_enemy_point_exists, binary_data) = binary_helpers::pop_u8(binary_data);
        let (closest_seen_enemy_point_x, binary_data) = binary_helpers::pop_f32(binary_data);
        let (closest_seen_enemy_point_y, binary_data) = binary_helpers::pop_f32(binary_data);
        let (task_binary_data, binary_data) = binary_helpers::pop_padded(binary_data);

        let mut path: Vec<point::Point> = Vec::new();
        while path_binary_data.len() > 0 {
            let mut point_x: f32 = 0.0;
            let mut point_y: f32 = 0.0;
            let (point_x, tmp1) = binary_helpers::pop_f32(path_binary_data);
            let (point_y, tmp2) = binary_helpers::pop_f32(tmp1);
            path_binary_data = tmp2;
            path.push(point::Point::new(point_x, point_y));
        }

        Entity {
            location: point::Point::new(location_x, location_y),
            id: id,
            entity_type: match entity_type {
                0 => EntityType::Peasant,
                1 => EntityType::Ranged,
                2 => EntityType::Meelee,
                _ => {println!("This should not happen"); EntityType::Peasant}
            },
            waypoint_index: waypoint_index,
            path: path,
            orientation: orientation,
            team_id: team_id,
            hp: hp,
            cooldown: cooldown,
            closest_seen_enemy_point: if closest_seen_enemy_point_exists == 0
                {None} else
                {Some(point::Point::new(
                    closest_seen_enemy_point_x,
                    closest_seen_enemy_point_y,
                ))},
            closest_seen_enemy_id: None,
            task: Task::from_binary(task_binary_data),
        }
    }
}


impl Entity {
    pub fn new(x: f32, y: f32, id: u32, team_id: u32, entity_type: EntityType) -> Entity {
        Entity {
            location: point::Point::new(x, y),
            id: id,
            entity_type: entity_type,

            waypoint_index: 0,
            path: Vec::new(),

            orientation: id % 8,

            team_id: team_id,
            hp: 200,
            cooldown: 0,
            closest_seen_enemy_point: None,
            closest_seen_enemy_id: None,

            task: Task::Idle,
        }
    }

    // Getter methods
    pub fn entity_type(&self) -> &EntityType { &self.entity_type }
    pub fn orientation(&self) -> u32 { self.orientation }
    pub fn path(&self) -> &Vec<point::Point> { &self.path }
    pub fn closest_seen_enemy_point(&self) -> &Option<point::Point> { &self.closest_seen_enemy_point }
    pub fn closest_seen_enemy_id(&self) -> &Option<u32> { &self.closest_seen_enemy_id}
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

    pub fn is_ranged(&self) -> bool {
        match self.entity_type {
            EntityType::Peasant => false,
            EntityType::Meelee => false,
            _ => true
        }
    }

    pub fn seeing_distance(&self) -> f32 {
        15.0
    }

    pub fn attack_distance(&self) -> f32 {
        match self.entity_type {
            EntityType::Ranged => 8.0,
            // EntityType::Meelee => 1.0,
            EntityType::Meelee => 0.6,
            _ => 0.0
        }
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

    pub fn can_attack(&self) -> bool {
        match self.entity_type {
            EntityType::Peasant => false,
            _ => true
        }
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
        if 
            // self.can_attack() && 
            seeing_enemy &&
            !moving
        {
            if self.can_attack() {
                return self.attack_enemy()
            } else {
                self.run_from_enemy();
                return None
            }
        }

        // Move
        if moving || attack_moving {
            self.follow_path_finding(map);
            return None
        }

        None
    }

    fn run_from_enemy(&mut self) {
        match &self.closest_seen_enemy_point {
            Some(point) => {
                let vector_to_enemy = self.location.dist_to(point);
                self.move_vector(&vector_to_enemy.normalized().multiplied(0.04), true);
            }, _ => {}
        }
    }

    fn attack_enemy(&mut self) -> Option<Projectile> {
        match &self.closest_seen_enemy_point {
            Some(point) => {
                let vector_to_enemy = self.location.dist_to(point);
                // Move towards
                if vector_to_enemy.length() > self.attack_distance() {
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

    /*
    pub fn clear_interaction_data(&mut self) {
        self.closest_seen_enemy_point = None;
    }
    */

    pub fn move_vector(&mut self, vector: &point::Vector, update_orientation: bool) {
        self.location.x += vector.x;
        self.location.y += vector.y;
        if update_orientation {
            self.set_orientation_from_vector(&vector.negated());
        }
    }

    pub fn can_reach(&self, distance: f32, point: &point::Point, map: &map::Map) -> bool {
        if distance > self.seeing_distance() as f32 {
            return false
        }

        if self.is_ranged() {
            if distance > self.attack_distance() as f32 {
                let walking_distance = distance - self.attack_distance() as f32 - 0.5;
                let walking_vector = self.location.dist_to(point).multiplied(walking_distance);
                let walking_point = self.location.added(&walking_vector);
                return map.line_of_sight_fat(self.location(), &walking_point, 0.25);
            } else {
                return true;
            }
        } else {
            return map.line_of_sight_fat(&self.location(), point, 0.25);
        }
    }

    pub fn reset_closest_seen_enemy_position(&mut self) {
        self.closest_seen_enemy_point = None;
        self.closest_seen_enemy_id= None;
    }

    pub fn update_closest_seen_enemy_point(&mut self, other_entity: &Entity) {
        self.closest_seen_enemy_point = Some(other_entity.location().clone());
    }

    pub fn update_closest_seen_enemy(&mut self, other_entity: &Entity) {
        if other_entity.team_id() != self.team_id() {
            let distance = self.location().dist_to(&other_entity.location()).length();
            match self.closest_seen_enemy_point {
                Some(point) => {
                    let current_distance = self.location().dist_to(&point).length();
                    if distance < current_distance {
                        self.closest_seen_enemy_point = Some(other_entity.location().clone());
                        self.closest_seen_enemy_id= Some(other_entity.id());
                    }
                },
                None => {
                    self.closest_seen_enemy_point = Some(other_entity.location().clone());
                    self.closest_seen_enemy_id= Some(other_entity.id());
                }
            }
        }
    }

    pub fn interact_with(&mut self, other: &Entity, map: &map::Map) {
        let max_dist = 0.55;

        let dist_vect = self.location.dist_to(&other.location);
        let distance = dist_vect.length();

        // Moving if another unit is too close
        if distance == 0.0 {
            // Two units at exact same location. Move one of them a bit.
            if self.id < other.id {
                self.move_vector(&point::Vector::new(0.1, 0.0), false);
            }
        } else if distance < max_dist {
            let move_vect = dist_vect.normalized().multiplied((max_dist - distance) * 0.3);
            self.move_vector(&move_vect, false);
        }
    }

    pub fn interact_with_map(&mut self, map: &map::Map) {
        let treshold = 0.25;

        let int_loc = self.location.as_i();
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

