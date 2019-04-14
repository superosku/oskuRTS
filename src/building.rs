
use super::binary_helpers::Binaryable;
use super::binary_helpers;


pub struct Building {
    x: i32,
    y: i32,
}

impl Binaryable for Building {
    fn as_binary(&self) -> Vec<u8> {
        let mut binary_data: Vec<u8> = Vec::new();
        binary_data.extend(binary_helpers::i32_as_bytes(self.x));
        binary_data.extend(binary_helpers::i32_as_bytes(self.y));
        binary_data
    }

    fn from_binary(binary_data: Vec<u8>) -> Building{
        let (x, binary_data) = binary_helpers::pop_i32(binary_data);
        let (y, binary_data) = binary_helpers::pop_i32(binary_data);
        Building::new((x, y))
    }
}

impl Building {
    pub fn new(location: (i32, i32)) -> Building {
        Building {
            x: location.0,
            y: location.1,
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn height(&self) -> i32 {
        3
    }

    pub fn width(&self) -> i32 {
        3
    }
}



