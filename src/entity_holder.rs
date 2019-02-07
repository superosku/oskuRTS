
use super::entity;
use super::map;

pub struct EntityHolder {
    pub entities: Vec<entity::Entity>,
}

impl EntityHolder {
    pub fn new() -> EntityHolder {
        EntityHolder {
            // entities: Vec<entity::Entity>::new()
            entities: Vec::new()
        }
    }

    pub fn add_new_entity(&mut self, x: f32, y: f32) {
        self.entities.push(entity::Entity::new(x, y));
        println!("Entity added at {}, {}", x, y);
    }

    pub fn get_entity_refs(&self) -> &Vec<entity::Entity> {
        return &self.entities
    }

    fn entity_at(&self, i: usize) -> entity::Entity {
        /// Returns copy of entity
        self.entities[i]
    }

    pub fn entities_interact_with_each_other(&mut self) {
        let mut entity1: entity::Entity;
        // TOOD: Is this copy really required? Cant borrow two mutables
        // Ok for now since entity only holds x,y but how about future?
        let vec_len = self.entities.len(); 
        for i_2 in 0..vec_len {
            let entity_2 = self.entity_at(i_2);
            for (i_1, entity_1) in self.entities.iter_mut().enumerate() {
                if i_1 != i_2 {
                    entity_1.interact_with(entity_2);
                }
            }
        }
    }

    pub fn entities_interact_with_map(&mut self, map: &map::Map) {
        for entity in self.entities.iter_mut() {
            entity.interact_with_map(&map);
        }
    }
}

