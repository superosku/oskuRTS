
use super::entity;
use super::map;
use super::point;

pub struct EntityHolder {
    pub entities: Vec<entity::Entity>,
    pub id_counter: u32,
}

impl EntityHolder {
    pub fn new() -> EntityHolder {
        EntityHolder {
            entities: Vec::new(),
            id_counter: 0,
        }
    }

    pub fn add_new_entity(&mut self, x: f32, y: f32) {
        self.entities.push(entity::Entity::new(x, y, self.id_counter));
        self.id_counter += 1;
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
}

