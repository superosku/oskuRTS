use std::collections::HashMap;

use super::entity::{Entity, Task, EntityType};
use super::map;
use super::point;
use super::path_finder;
use super::projectile::Projectile;
use super::building::Building;
use super::binary_helpers::{Binaryable, u32_as_bytes, vec_as_bytes};

pub struct EntityHolder {
    pub entities: Vec<Entity>,
    pub projectiles: Vec<Projectile>,
    pub buildings: Vec<Building>,

    pub id_counter: u32,
    pub debug_search_tree: HashMap<(i32, i32), Option<(i32, i32)>> // For debug drawings of the search tree
}


impl Binaryable for EntityHolder {
    fn as_binary(&self) -> Vec<u8> {
        let mut binary_data: Vec<u8> = Vec::new();
        
        binary_data.extend(u32_as_bytes(self.id_counter));
        binary_data.extend(vec_as_bytes(&self.entities));
        binary_data.extend(vec_as_bytes(&self.projectiles));
        binary_data.extend(vec_as_bytes(&self.buildings));

        binary_data
    }
}


impl EntityHolder {
    pub fn new() -> EntityHolder {
        EntityHolder {
            entities: Vec::new(),
            projectiles: Vec::new(),
            buildings: Vec::new(),

            id_counter: 0,
            debug_search_tree: HashMap::new(),
        }
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
        for entity in self.entities.iter() {
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

            for entity in self.entities.iter_mut() {
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

        self.entities.push(Entity::new(x, y, self.id_counter, team_id, entity_type));
        self.id_counter += 1;
    }

    pub fn get_entity_refs(&self) -> &Vec<Entity> {
        return &self.entities
    }

    pub fn entities_interact_with_each_other(&mut self, map: &map::Map) {
        for entity in self.entities.iter_mut() {
            entity.clear_interaction_data();
        }
        let entity_count = self.entities.len();
        for entity_id_1 in 0..entity_count {
            let (a, b) = self.entities.split_at_mut(entity_id_1 + 1);
            for entity_id_2 in (entity_id_1 + 1)..entity_count {
                match a.get_mut(entity_id_1) {
                    Some(entity_1) => {
                        match b.get_mut(entity_id_2 - entity_id_1 - 1) {
                            Some(entity_2) => {
                                entity_1.interact_with(entity_2, map);
                                entity_2.interact_with(entity_1, map);
                            },
                            _ => {println!("This should not happen");}
                        };
                    },
                    _ => {println!("This should not happen");}
                };
            }
        }
    }

    pub fn order_stop(&mut self, entity_ids: HashMap<u32, bool>) {
        for entity in self.entities.iter_mut() {
            if entity_ids.contains_key(&entity.id()) {
                entity.order_stop();
            }
        }
    }

    pub fn entities_interact_with_map(&mut self, map: &map::Map) {
        for entity in self.entities.iter_mut() {
            entity.interact_with_map(&map);
        }
    }

    pub fn entities_ai_stuff(&mut self, map: &map::Map) {
        for entity in self.entities.iter_mut() {
            match entity.ai_stuff(map) { Some(projectile) => {
                self.projectiles.push(projectile);
            }, _ => {} }
        }
    }

    pub fn increment_projectiles(&mut self) {
        for projectile in self.projectiles.iter_mut() {
            projectile.increment();
            if projectile.at_location() {
                'inner: for entity in self.entities.iter_mut() {
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

    pub fn entity_ai(&mut self, map: &map::Map) {
        self.entities_ai_stuff(&map);
        self.entities_interact_with_each_other(&map);
        self.entities_interact_with_map(&map);

        self.increment_projectiles();

        self.entities.retain(|entity| {entity.alive()});
    }
}

