use std::collections::{HashMap};
use std::collections::hash_map::{Values, ValuesMut};
use std::iter::Iterator;

use multi_mut::{HashMapMultiMut, BTreeMapMultiMut};

use super::entity::{Entity, Task, EntityType};
use super::map;
use super::point;
use super::path_finder;
use super::projectile::Projectile;
use super::building::Building;
use super::binary_helpers::Binaryable;
use super::binary_helpers;

pub struct EntityHolder {
    pub entities: HashMap<u32, Entity>,
    pub projectiles: Vec<Projectile>,
    pub buildings: Vec<Building>,
    pub id_counter: u32,

    pub entity_location_map: HashMap<(i32, i32), Vec<u32>>,

    // For debug drawings of the search tree
    pub debug_search_tree:
        HashMap<(i32, i32), Option<(i32, i32)>>,
    pub debug_entity_interaction_count: u32,
}


impl Binaryable for EntityHolder {
    fn as_binary(&self) -> Vec<u8> {
        let mut binary_data: Vec<u8> = Vec::new();
        
        binary_data.extend(binary_helpers::u32_as_bytes(self.id_counter));
        binary_data.extend(binary_helpers::iter_as_bytes(self.entities_iter()));
        binary_data.extend(binary_helpers::iter_as_bytes(self.projectiles.iter()));
        binary_data.extend(binary_helpers::iter_as_bytes(self.buildings.iter()));

        binary_data
    }

    fn from_binary(binary_data: Vec<u8>) -> EntityHolder {
        let (id_counter, binary_data) = binary_helpers::pop_u32(binary_data);
        let (mut units_data, binary_data) = binary_helpers::pop_padded(binary_data);
        let (mut projectiles_data, binary_data) = binary_helpers::pop_padded(binary_data);
        let (mut buildings_data, binary_data) = binary_helpers::pop_padded(binary_data);

        let mut new_entity_holder = EntityHolder::new();

        new_entity_holder.id_counter = id_counter;

        while buildings_data.len() > 0 {
            let (building_data, tmp) = binary_helpers::pop_padded(buildings_data);
            buildings_data = tmp;
            new_entity_holder.buildings.push(Building::from_binary(building_data));
        }
        while projectiles_data.len() > 0 {
            let (projectile_data, tmp) = binary_helpers::pop_padded(projectiles_data);
            projectiles_data = tmp;
            new_entity_holder.projectiles.push(Projectile::from_binary(projectile_data));
        }
        while units_data.len() > 0 {
            let (unit_data, tmp) = binary_helpers::pop_padded(units_data);
            units_data = tmp;
            let mut entity = Entity::from_binary(unit_data);
            new_entity_holder.entities.insert(entity.id(), entity);
        }

        new_entity_holder
    }
}


impl EntityHolder {
    pub fn new() -> EntityHolder {
        EntityHolder {
            entities: HashMap::new(),
            projectiles: Vec::new(),
            buildings: Vec::new(),
            id_counter: 0,

            entity_location_map: HashMap::new(),

            debug_search_tree: HashMap::new(),
            debug_entity_interaction_count: 0,
        }
    }

    pub fn entities_iter_mut(&mut self) -> ValuesMut<u32, Entity> { // Iterator<Item=&Entity> {
        self.entities.values_mut()
        // self.entities.iter_mut().map(|(k, v)| v)
    }

    pub fn entities_iter(&self) -> Values<u32, Entity> { // Iterator<Item=&Entity> {
        self.entities.values()
        // self.entities.iter().map(|(k, v)| v)
    }

    pub fn order_entities(
        &mut self,
        map: &map::Map,
        task: Task,
        entity_ids: HashMap<u32, bool>
    ) {
        // TODO: Clean up this whole mess of a function....

        // Get the end_point from task or return
        let end_point = match task {
            Task::Move {point} |
            Task::AttackMove {point} => point,
            Task::Idle => {
                self.order_stop(entity_ids);
                return
            },
            _ => {
                println!("Warning: Unknown order");
                return
            }
        };

        // Find out distinct goal points
        let mut goal_points: HashMap<(i32, i32), bool> = HashMap::new();
        for entity in self.entities_iter() {
            if entity_ids.contains_key(&entity.id()) {
                let key = entity.location().as_i();
                if !goal_points.contains_key(&key) {
                    goal_points.insert(key, true);
                }
            }
        }
        let mut distinct_points: Vec<(i32, i32)> = Vec::new(); // TODO: Cleaner way to do this mess...
        for point in goal_points.keys() {
            distinct_points.push(*point);
        }

        let search_tree: HashMap<(i32, i32), Option<(i32, i32)>> =
            path_finder::build_search_tree(map, end_point.as_i(), &distinct_points);

        for point in distinct_points.iter() {
            let mut path: Vec<(i32, i32)> = Vec::new();
            let mut old_point: &(i32, i32) = point;
            path.push(*old_point);
            loop {
                let next_point_option: &Option<(i32, i32)> = match search_tree.get(old_point) {
                    Some(x) => x,
                    _ => {
                        break;
                    }
                };

                let next_point = match next_point_option {
                    Some(x) => x,
                    _ => break
                };

                path.push(*next_point);
                old_point = next_point;
            }

            for entity in self.entities_iter_mut() {
                if entity_ids.contains_key(&entity.id()) &&
                    entity.location().x as i32 == point.0 &&
                    entity.location().y as i32 == point.1
                {
                    let mut path_queue = Vec::new();

                    for p in path.iter() {
                        path_queue.push(point::Point::new(p.0 as f32 + 0.5, p.1 as f32 + 0.5));
                    }
                    path_queue.push(end_point.clone());

                    entity.set_path(path_queue, task.clone());
                }
            }
        }

        self.debug_search_tree = search_tree;
    }

    pub fn add_new_building(&mut self, map: &mut map::Map, location: (i32, i32), team_id: u32) {
        let building = Building::new(location);

        for x in building.x()..(building.x() + building.width()) {
            for y in building.y()..(building.y() + building.height()) {
                map.set_second_layer(x, y, map::SecondLevelType::Building)
            }
        }

        self.buildings.push(building);
    }

    pub fn add_new_entity(&mut self, x: f32, y: f32, team_id: u32) {
        let mut entity_type: EntityType = EntityType::Meelee;
        if self.id_counter % 3 == 0 { entity_type = EntityType::Ranged }
        if self.id_counter % 3 == 1 { entity_type = EntityType::Peasant }

        let mut new_entity = Entity::new(x, y, self.id_counter, team_id, entity_type);
        let entity_id = new_entity.id();
        self.entities.insert(entity_id, new_entity);

        self.id_counter += 1;
    }

    pub fn update_entity_location_map(&mut self) {
        let mut entity_location_map = HashMap::new();

        for entity in self.entities_iter() {
            let key = entity.location().as_i();

            if !entity_location_map.contains_key(&key) {
                let new_vec: Vec<u32> = Vec::new();
                entity_location_map.insert(key, new_vec);
            }

            match entity_location_map.get_mut(&key) {
                Some(vector) => {
                    vector.push(entity.id());
                }, None => {
                    println!("This should not happen");
                }
            }
        }

        self.entity_location_map = entity_location_map;
    }

    pub fn get_close_entity_ids(&self, location: &point::Point, radius: f32) -> Vec<u32> {
        let mut close_entity_ids: Vec<u32> = Vec::new();

        // println!("Get close entity_ids, {},{} {}", location.x, location.y, radius);
        for x in ((location.x as f32 - radius) as i32)..((location.x as f32 + radius) as i32 + 1) {
            for y in ((location.y as f32 - radius) as i32)..((location.y as f32 + radius) as i32 + 1) {
                // println!("X {} Y{}", x, y);
                match self.entity_location_map.get(&(x, y)) {
                    Some(ids) => {
                        for id in ids.iter() {
                            close_entity_ids.push(*id);
                        }
                    }, None => {}
                }
            }
        }

        close_entity_ids
    }

    pub fn entities_interact_with_each_other(&mut self, map: &map::Map, tick: u32) {
        for entity in self.entities_iter_mut() {
            entity.clear_interaction_data();
        }

        let entity_keys: Vec<u32> = self.entities.keys().map(|k| k.clone()).collect();

        self.update_entity_location_map();

        self.debug_entity_interaction_count = 0;

        for entity_key_1 in entity_keys.iter() {
            let entity_1_location = match self.entities.get(entity_key_1) {
                Some(entity) => entity.location().clone(),
                None => {println!("This should not happen"); point::Point::new(0.0, 0.0)}
            };
            for entity_key_2 in self.get_close_entity_ids(&entity_1_location, 5.0).iter() {
            // for entity_key_2 in entity_keys.iter() {
                if entity_key_1 != entity_key_2 {
                    let (entity_1, entity_2) = self.entities.get_pair_mut(entity_key_1, entity_key_2).unwrap();

                    entity_1.interact_with(entity_2, map);
                    // entity_2.interact_with(entity_1, map);
                    self.debug_entity_interaction_count += 1;
                }
            }
        }
    }

    pub fn order_stop(&mut self, entity_ids: HashMap<u32, bool>) {
        for entity in self.entities_iter_mut() {
            if entity_ids.contains_key(&entity.id()) {
                entity.order_stop();
            }
        }
    }

    pub fn entities_interact_with_map(&mut self, map: &map::Map) {
        for entity in self.entities_iter_mut() {
            entity.interact_with_map(&map);
        }
    }

    pub fn entities_ai_stuff(&mut self, map: &map::Map) {
        for entity in self.entities.values_mut() {
            match entity.ai_stuff(map) { Some(projectile) => {
                self.projectiles.push(projectile);
            }, _ => {} }
        }
    }

    pub fn increment_projectiles(&mut self) {
        for projectile in self.projectiles.iter_mut() {
            projectile.increment();
            if projectile.at_location() {
                'inner: for entity in self.entities.values_mut() {
                    if entity.location().dist_to(projectile.location()).length() < 0.5 {
                        entity.take_hit(12);
                        break 'inner;
                    }
                }
                // TODO: Harm units that were hit
            }
        }
        self.projectiles.retain(|projectile| {
            return !projectile.at_location()
        });
    }

    pub fn entity_ai(&mut self, map: &map::Map, tick: u32) {
        self.entities_ai_stuff(&map);
        self.entities_interact_with_each_other(&map, tick);
        self.entities_interact_with_map(&map);

        self.increment_projectiles();

        self.entities.retain(|_, entity| {entity.alive()});
    }
}

