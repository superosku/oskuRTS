

pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Vector {
        Vector {x: x, y: y}
    }

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()

    }

    pub fn normalized(&self) -> Vector {
        let length = self.length();
        Vector::new(
            self.x / length,
            self.y / length
        )
    }

    pub fn add(&mut self, other: &Vector) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn multiplied(&self, times: f32) -> Vector {
        Vector::new(
            self.x * times,
            self.y * times
        )
    }
}


pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point {x: x, y: y}
    }

    /*
    pub fn distance(&self, other: &Point) -> f32 {
        (
            (self.x - other.x).powi(2) +
            (self.y - other.y).powi(2)
        ).sqrt()
    }
    */

    pub fn add(&mut self, vec: &Vector) {
        self.x += vec.x;
        self.y += vec.y;
    }

    pub fn add_vect(&self, vec: &Vector) -> Point {
        Point {
            x: self.x + vec.x,
            y: self.y + vec.y
        }
    }

    pub fn dist_vect(&self, other: &Point) -> Vector {
        Vector::new(
            self.x - other.x,
            self.y - other.y
        )
    }

    pub fn as_int(&self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }
}

