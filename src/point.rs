

#[derive(Debug)]
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
        if length == 0.0 {
            return Vector::new(0.0, 0.0); // Todo is this best way?
        }
        Vector::new(
            self.x / length,
            self.y / length
        )
    }

    pub fn x_normalized(&self) -> Vector {
        if self.x == 0.0 {
            return Vector::new(0.0, 0.0)
        }
        Vector::new(self.x / self.x.abs(), self.y / self.x.abs())
    }

    pub fn y_normalized(&self) -> Vector {
        if self.y == 0.0 {
            return Vector::new(0.0, 0.0)
        }
        Vector::new(self.x / self.y.abs(), self.y / self.y.abs())
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

    pub fn negated(&self) -> Vector {
        Vector::new(
            -self.x,
            -self.y
        )
    }
}


#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point {x: x, y: y}
    }

    pub fn add(&mut self, vec: &Vector) {
        self.x += vec.x;
        self.y += vec.y;
    }

    pub fn added(&self, vec: &Vector) -> Point {
        Point::new(
            self.x + vec.x,
            self.y + vec.y
        )
    }

    pub fn dist_to(&self, other: &Point) -> Vector {
        Vector::new(
            self.x - other.x,
            self.y - other.y
        )
    }

    pub fn as_int(&self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }
}

