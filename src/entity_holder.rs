use std::collections::HashMap;

use super::entity;
use super::map;
use super::point;
use super::path_finder;
use super::projectile::Projectile;

pub struct EntityHolder {
    pub entities: Vec<entity::Entity>,
    pub projectiles: Vec<Projectile>,

    pub selected_entity_ids: HashMap<u32, bool>,
    pub id_counter: u32,
    pub debug_search_tree: HashMap<(i32, i32), Option<(i32, i32)>> // For debug drawings of the search tree
}

impl EntityHolder {
    pub fn new() -> EntityHolder {
        EntityHolder {
            entities: Vec::new(),
            projectiles: Vec::new(),

            selected_entity_ids: HashMap::new(),
            id_counter: 0,
            debug_search_tree: HashMap::new(),
        }
    }

    pub fn set_selection(&mut self, pos1: (f32, f32), pos2: (f32, f32)) {
        self.selected_entity_ids.clear();
        for entity in self.entities.iter() {
            if entity.is_inside(pos1, pos2) && entity.team_id() == 0 {
                self.selected_entity_ids.insert(entity.id(), true);
            }
        }
    }

    pub fn order_selected_units_to(&mut self, map: &map::Map, end_point: (f32, f32), task: entity::Task) {
        // TODO: Clean up this whole mess of a function....

        // Find out distinct goal points
        let mut goal_points: HashMap<(i32, i32), bool> = HashMap::new();
        for entity in self.entities.iter() {
            if self.entity_selected(entity) {
                let key = (entity.location().x as i32, entity.location().y as i32);
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
            path_finder::build_search_tree(map, (end_point.0 as i32, end_point.1 as i32), &distinct_points);

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
            // println!("Point {:?} {} {:?}", point, search_tree.contains_key(point), path);

            for entity in self.entities.iter_mut() {
                if self.selected_entity_ids.contains_key(&entity.id()) &&
                    entity.location().x as i32 == point.0 &&
                    entity.location().y as i32 == point.1
                {
                    let mut path_queue = Vec::new();

                    for p in path.iter() {
                        // path_queue.push_back(point::Point::new(p.0 as f32 + 0.5, p.1 as f32 + 0.5));
                        path_queue.push(point::Point::new(p.0 as f32 + 0.5, p.1 as f32 + 0.5));
                    }
                    path_queue.push(point::Point::new(end_point.0 as f32, end_point.1 as f32));

                    entity.set_path(path_queue, task.clone());
                }
            }
        }

        self.debug_search_tree = search_tree;
    }

    pub fn entity_selected(&self, entity: &entity::Entity) -> bool {
        self.selected_entity_ids.contains_key(&entity.id())
    }

    pub fn add_new_entity(&mut self, x: f32, y: f32, team_id: u32) {
        self.entities.push(entity::Entity::new(x, y, self.id_counter, team_id));
        self.id_counter += 1;
    }

    pub fn sort_entities(&mut self) {
        // TODO: This is not so nice way to do this
        if self.entities.len() < 2 {
            return
        }
        // Some bubblesort :)
        for i in 0..(self.entities.len() - 2) {
            if self.entities[i].location().y > self.entities[i + 1].location().y {
                self.entities.swap(i, i+1);
            }
        }
    }

    pub fn get_entity_refs(&self) -> &Vec<entity::Entity> {
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

    pub fn order_stop_selection(&mut self) {
        for entity in self.entities.iter_mut() {
            if self.selected_entity_ids.contains_key(&entity.id()) {
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

