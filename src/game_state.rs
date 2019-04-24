

use std::fs;
use std::fs::File;
use std::io::Read;


use std::collections::HashMap;


use super::point::Point;
use super::map::{Map, GroundType};
use super::entity_holder::{EntityHolder};
use super::entity::{EntityType, Task};

use super::binary_helpers::Binaryable;
use super::binary_helpers;


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


impl binary_helpers::Binaryable for GameState {
    fn as_binary(&self) -> Vec<u8> {
        let mut binary_data: Vec<u8> = Vec::new();

        binary_data.extend(binary_helpers::u32_as_bytes(self.tick));
        binary_data.extend(self.map.as_padded_binary());
        binary_data.extend(self.entity_holder.as_padded_binary());

        binary_data
    }

    fn from_binary(binary_data: Vec<u8>) -> GameState {
        println!("Loading GameState from binary");

        let (tick ,binary_data) = binary_helpers::pop_u32(binary_data);
        let (map_data, binary_data) = binary_helpers::pop_padded(binary_data);
        let (entity_data, binary_data) = binary_helpers::pop_padded(binary_data);

        println!("Map data: {}", map_data.len());
        println!("Entity data: {}", entity_data.len());
        println!("Rest of the data: {}", binary_data.len());

        GameState {
            tick: tick,
            map: Map::from_binary(map_data),
            entity_holder: EntityHolder::from_binary(entity_data),
            event_log: Vec::new(),
        }
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

    pub fn from_file_name(file_name: String) -> GameState {
        let mut file_object = File::open(file_name).expect("File name not found");
        let mut binary_data: Vec<u8> = Vec::new();
        file_object.read_to_end(&mut binary_data).expect("File reading failed");
        GameState::from_binary(binary_data)
    }

    pub fn map(&self) -> &Map { &self.map }
    pub fn entity_holder(&self) -> &EntityHolder { &self.entity_holder }

    pub fn tick(&self) -> u32 { self.tick }
    pub fn do_tick(&mut self) {
        self.tick += 1;
        self.entity_holder.entity_ai(&self.map, self.tick);
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

