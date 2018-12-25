use std::ops::{self};


#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }
}

impl Point {
    pub fn set(&mut self, x: f32, y: f32) -> &mut Point {
        self.x = x;
        self.y = y;
        self
    }
    
    pub fn floor(&self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }
}

impl ops::Add for &Point {
    type Output = Point;

    fn add(self, rhs: &Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }
}