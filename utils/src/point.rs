use crate::dir::Dir;
use std::ops;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}


impl Point {
    pub fn zero() -> Point {
        Point { x: 0, y: 0 }
    }
}

impl ops::Add<Dir> for Point {
    type Output = Point;
    fn add(self, dir: Dir) -> Point {
        self + &(dir.point())
    }
}

impl ops::Add<&Point> for Point {
    type Output = Point;
    fn add(self, p: &Point) -> Point {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

impl ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl ops::AddAssign<&Dir> for Point {
    fn add_assign(&mut self, other: &Dir) {
        *self += other.point();
    }
}
