use std::collections::HashMap;
use std::collections::VecDeque;

use super::entity;
use super::map;
use super::point;
use super::path_finder;

pub struct EntityHolder {
    pub entities: Vec<entity::Entity>,
    pub selected_entity_ids: HashMap<u32, bool>,
    pub id_counter: u32,
}

impl EntityHolder {
    pub fn new() -> EntityHolder {
        EntityHolder {
            entities: Vec::new(),
            selected_entity_ids: HashMap::new(),
            id_counter: 0,
        }
    }

    pub fn set_selection(&mut self, pos1: (f32, f32), pos2: (f32, f32)) {
        self.selected_entity_ids.clear();
        for entity in self.entities.iter() {
            if entity.is_inside(pos1, pos2) {
                self.selected_entity_ids.insert(entity.id, true);
            }
        }
    }

    pub fn order_selected_units_to(&mut self, map: &map::Map, end_point: (f32, f32)) {
        // TODO: Clean up this whole mess of a function....

        // Find out distinct goal points
        let mut goal_points: HashMap<(i32, i32), bool> = HashMap::new();
        for entity in self.entities.iter() {
            if self.entity_selected(entity) {
                let key = (entity.location.x as i32, entity.location.y as i32);
                if !goal_points.contains_key(&key) {
                    goal_points.insert(key, true);
                }
            }
        }
        let mut distinct_points: Vec<(i32, i32)> = Vec::new(); // TODO: Cleaner way to do this mess...
        for point in goal_points.keys() {
            distinct_points.push(*point);
        }

        let search_tree = path_finder::build_search_three(map, (end_point.0 as i32, end_point.1 as i32), &distinct_points);
        // println!("Finding paths for points");
        for point in distinct_points.iter() {
            let mut path: Vec<(i32, i32)> = Vec::new();
            let mut old_point: &(i32, i32) = point;
            path.push(*old_point);
            loop {
                let next_point: &(i32, i32) = match search_tree.get(old_point) {
                    Some(x) => x,
                    _ => {
                        break;
                    }
                };

                path.push(*next_point);

                if old_point == next_point {
                    break;
                }

                old_point = next_point;
            }
            // println!("Point {:?} {} {:?}", point, search_tree.contains_key(point), path);

            for entity in self.entities.iter_mut() {
                if self.selected_entity_ids.contains_key(&entity.id) &&
                    entity.location.x as i32 == point.0 &&
                    entity.location.y as i32 == point.1
                {
                    let mut path_queue = VecDeque::new();

                    for p in path.iter() {
                        path_queue.push_back(point::Point::new(p.0 as f32 + 0.5, p.1 as f32 + 0.5));
                    }
                    path_queue.push_back(point::Point::new(end_point.0 as f32, end_point.1 as f32));

                    entity.set_path(path_queue)
                }
            }
        }
    }

    pub fn entity_selected(&self, entity: &entity::Entity) -> bool {
        self.selected_entity_ids.contains_key(&entity.id)
    }

    pub fn add_new_entity(&mut self, x: f32, y: f32) {
        self.entities.push(entity::Entity::new(x, y, self.id_counter));
        self.id_counter += 1;
    }

    pub fn sort_entities(&mut self) {
        if self.entities.len() < 2 {
            return
        }
        for i in 0..(self.entities.len() - 2) {
            if self.entities[i].location.y > self.entities[i + 1].location.y {
                self.entities.swap(i, i+1);
            }
        }
    }

    pub fn get_entity_refs(&self) -> &Vec<entity::Entity> {
        return &self.entities
    }

    pub fn entities_interact_with_each_other(&mut self) {
        let mut force_vecs: Vec<point::Vector> = Vec::new();

        for (i_1, entity_1) in self.entities.iter().enumerate() {
            let mut force_vec_for_1 = point::Vector::new(0.0, 0.0);
            for (i_2, entity_2) in self.entities.iter().enumerate() {
                if i_1 != i_2 {
                    let vector_some = entity_1.interact_with(entity_2);
                    match vector_some {
                        Some(vector) => {force_vec_for_1.add(&vector)},
                        _ => {}
                    }
                }
            }
            force_vecs.push(force_vec_for_1);
        }

        for (entity, force_vect) in self.entities.iter_mut().zip(force_vecs) {
            entity.add_force_vect(&force_vect);
        }
    }

    pub fn entities_interact_with_map(&mut self, map: &map::Map) {
        for entity in self.entities.iter_mut() {
            entity.interact_with_map(&map);
        }
    }

    pub fn entities_ai_stuff(&mut self, map: &map::Map) {
        for entity in self.entities.iter_mut() {
            entity.ai_stuff(map);
        }
    }
}

