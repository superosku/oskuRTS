


pub struct Building {
    x: i32,
    y: i32,
}

impl Building {
    pub fn new(x: i32, y: i32) -> Building {
        Building {
            x: x,
            y: y,
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



