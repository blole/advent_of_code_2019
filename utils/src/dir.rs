use crate::point::Point;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Dir { U,R,D,L }

impl Dir {
    pub fn point(&self) -> Point {
        match *self {
            Dir::U => Point { x: -1, y:  0 },
            Dir::R => Point { x:  0, y:  1 },
            Dir::D => Point { x:  1, y:  0 },
            Dir::L => Point { x:  0, y: -1 },
        }
    }
}

impl FromStr for Dir {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().chars().next() {
            Some('u') => Ok(Dir::U),
            Some('r') => Ok(Dir::R),
            Some('d') => Ok(Dir::D),
            Some('l') => Ok(Dir::L),
            _   => Err(format!("bad dir '{}'", s))
        }
    }
}
