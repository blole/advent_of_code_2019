use std::io;
use std::io::prelude::*;
use std::str::FromStr;
use std::error;
use utils::dir::Dir;
use utils::point::Point;
use std::collections::{HashMap, HashSet};


#[derive(Debug, PartialEq, Eq)]
struct WireSeg {
    dir: Dir,
    len: u32,
}
impl FromStr for WireSeg {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Box<dyn error::Error>> {
        let (dir, len) = s.split_at(1);
        Ok(WireSeg {
            dir: dir.parse::<Dir>()?,
            len: len.parse::<u32>()?
        })
    }
}

fn wire(line: String) -> Vec<WireSeg> {
    line
        .split(',')
        .map(|di| di.parse::<WireSeg>())
        .filter_map(Result::ok)
        .collect()
}

trait VecWireSeg {
    fn points(&self) -> Vec<Point>;
}
impl VecWireSeg for Vec<WireSeg> {
    fn points(&self) -> Vec<Point> {
        let mut list: Vec<Point> = Vec::new();
        let mut p = Point::zero();
        for seg in self {
            for _ in 0..seg.len {
                p += &seg.dir;
                list.push(p);
            }
        }
        list
    }
}

fn part1(a: &Vec<Point>, b: &Vec<Point>) -> Option<i32> {
    let ah: HashSet<&Point> = a.iter().collect();
    let bh: HashSet<&Point> = b.iter().collect();
    
    ah.intersection(&bh).map(|&p| p.x.abs() + p.y.abs()).min()
}

fn part2(a: &Vec<Point>, b: &Vec<Point>) -> Option<usize> {
    let am: HashMap<&Point, usize> = a.iter().enumerate().map(|(i,p)| (p,i+1)).collect();
    let bm: HashMap<&Point, usize> = b.iter().enumerate().map(|(i,p)| (p,i+1)).collect();
    let ah: HashSet<&Point> = a.iter().collect();
    let bh: HashSet<&Point> = b.iter().collect();
    
    ah.intersection(&bh).map(|&p| am[p] + bm[p]).min()
}

fn main() {
    let stdin = io::stdin();
    let wires: Vec<_> = stdin
        .lock()
        .lines()
        .filter_map(Result::ok)
        .map(wire)
        .collect();

    let a = wires[0].points();
    let b = wires[1].points();
    println!("{}", part1(&a,&b).unwrap());
    println!("{}", part2(&a,&b).unwrap());
}





#[cfg(test)]
mod tests_day03 {
    use super::*;

    #[test]
    fn wire_seg() {
        assert_eq!("U0".parse::<WireSeg>().unwrap(), WireSeg { dir: Dir::U, len: 0 });
        assert_eq!("R1".parse::<WireSeg>().unwrap(), WireSeg { dir: Dir::R, len: 1 });
        assert_eq!("D2".parse::<WireSeg>().unwrap(), WireSeg { dir: Dir::D, len: 2 });
        assert_eq!("L3".parse::<WireSeg>().unwrap(), WireSeg { dir: Dir::L, len: 3 });
    }

    #[test]
    fn points() {
        assert_eq!(wire("U10".into()).points().len(), 10);
        assert_eq!(wire("U10,D10".into()).points().len(), 20);
    }

    #[test]
    fn p1() {
        let a = wire("R75,D30,R83,U83,L12,D49,R71,U7,L72".into()).points();
        let b = wire("U62,R66,U55,R34,D71,R55,D58,R83".into()).points();
        assert_eq!(part1(&a,&b), Some(159));
        let a = wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".into()).points();
        let b = wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".into()).points();
        assert_eq!(part1(&a,&b), Some(135));
    }

    #[test]
    fn p2() {
        let a = wire("R75,D30,R83,U83,L12,D49,R71,U7,L72".into()).points();
        let b = wire("U62,R66,U55,R34,D71,R55,D58,R83".into()).points();
        assert_eq!(part2(&a,&b), Some(610));
        let a = wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51".into()).points();
        let b = wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".into()).points();
        assert_eq!(part2(&a,&b), Some(410));
    }
}
