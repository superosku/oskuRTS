

use std::fs;


use std::collections::HashMap;


use super::point::Point;
use super::map::{Map, GroundType};
use super::entity_holder::{EntityHolder};
use super::entity::{EntityType, Task};

use super::binary_helpers::Binaryable;


pub enum GameEvent {
    OrderUnits {
        task: Task,
        unit_ids: HashMap<u32, bool>,
    },
    InsertUnit {
        location: Point,
        team_id: u32,
        unit_type: EntityType,
    },
    SetMapPoint {
        location: (i32, i32),
        ground_type: GroundType,
    },
    AddBuilding {
        location: (i32, i32),
    }
}


pub struct GameState {
    tick: u32,

    map: Map,
    entity_holder: EntityHolder,

    event_log: Vec<GameEvent>,
}


impl Binaryable for GameState {
    fn as_binary(&self) -> Vec<u8> {
        let mut binary_data: Vec<u8> = Vec::new();

        binary_data.extend(self.map.as_padded_binary());
        binary_data.extend(self.entity_holder.as_padded_binary());

        binary_data
    }
}


impl GameState {
    pub fn new() -> GameState {
        GameState {
            tick: 0,
            map: Map::new_random(100, 50),
            entity_holder: EntityHolder::new(),
            event_log: Vec::new(),
        }
    }

    pub fn map(&self) -> &Map { &self.map }
    pub fn entity_holder(&self) -> &EntityHolder { &self.entity_holder }

    pub fn tick(&self) -> u32 { self.tick }
    pub fn do_tick(&mut self) {
        self.tick += 1;
        self.entity_holder.entity_ai(&self.map);
        while let Some(game_event) = self.event_log.pop() {
            match game_event {
                GameEvent::OrderUnits { task, unit_ids } => {
                    self.entity_holder.order_entities(
                        &self.map,
                        task,
                        unit_ids,
                    );
                },
                GameEvent::InsertUnit { location, team_id, unit_type } => {
                    self.entity_holder.add_new_entity(
                        location.x, location.y, team_id
                    );
                },
                GameEvent::SetMapPoint { location, ground_type } => {
                    self.map.set(location.0, location.1, ground_type);
                },
                GameEvent::AddBuilding { location } => {
                    self.entity_holder.add_new_building(
                        &mut self.map, location, 0
                    );
                }
            }
        }
    }

    pub fn dispatch_event(&mut self, game_event: GameEvent) {
        self.event_log.push(game_event);
    }

    pub fn save_to_file(&self) {
        println!("GameState saved to file");
        let binary_data = self.as_binary();

        fs::write("saved_game.dat", binary_data).expect("Unable to write file");
    }
}
