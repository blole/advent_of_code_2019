extern crate regex;

use std::io;
use std::io::prelude::*;
use regex::Regex;
use std::ops;

#[derive(Debug)]
struct Triple(i32, i32, i32);

impl Triple {
    fn signum(&self) -> Triple {
        Triple(self.0.signum(), self.1.signum(), self.2.signum())
    }
}

impl ops::Add for &Triple {
    type Output = Triple;
    fn add(self, o: &Triple) -> Triple {
        Triple(self.0+o.0, self.1+o.1, self.2+o.2)
    }
}
impl ops::Sub<&Triple> for &Triple {
    type Output = Triple;
    fn sub(self, o: &Triple) -> Triple {
        Triple(self.0-o.0, self.1-o.1, self.2-o.2)
    }
}
impl ops::AddAssign<&Triple> for &mut Triple {
    fn add_assign(&mut self, o: &Triple) {
        **self = &**self+o;
    }
}
impl std::iter::Sum<Triple> for Triple {
    fn sum<I>(iter: I) -> Triple
    where
        I: Iterator<Item = Triple>,
    {
        iter.fold(Triple(0,0,0), |a,b| &a+&b)
    }
}

#[derive(Debug)]
struct Moon {
    pos: Triple,
    vel: Triple,
}
impl Moon {
    fn energy(&self) -> i32 {
        let Triple(a,b,c) = self.pos;
        let Triple(x,y,z) = self.vel;
        (a.abs() + b.abs() + c.abs()) * (x.abs() + y.abs() + z.abs())
    }
}

fn read_moons<I: Iterator<Item=S>, S: Into<String>>(iter: I) -> Vec<Moon> {
    let re = Regex::new(r"<x=(-?[0-9]+), y=(-?[0-9]+), z=(-?[0-9]+)>").unwrap();
    iter.map(|line| line.into())
        .map(|line| {
            let caps = re.captures(&line).unwrap();
            let x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let y = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let z = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
            Moon { pos: Triple(x,y,z), vel: Triple(0,0,0) }
        })
        .collect::<Vec<Moon>>()
}

fn step(moons: &mut Vec<Moon>) {
    let vel_updates: Vec<Triple> = moons.iter()
        .map(|moon| moons.iter().map(|o| (&o.pos-&moon.pos).signum()).sum())
        .collect();
    for (moon, update) in moons.iter_mut().zip(vel_updates) {
        let mut vel: &mut Triple = & mut moon.vel; //?!?
        vel += &update;
    }
    for moon in moons.iter_mut() {
        moon.pos = &moon.pos + &moon.vel;
    }
}

fn p1(moons: &mut Vec<Moon>, steps: i32) -> i32 {
    for _ in 0..steps {
        step(moons);
    }
    moons.iter().map(|moon| moon.energy()).sum::<i32>()
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().filter_map(Result::ok);
    let mut moons = read_moons(lines);

    let part1 = p1(&mut moons, 1000);

    println!("{}", part1)
}





#[cfg(test)]
mod tests_day06 {
    use super::*;

    #[test]
    fn p1_0() {
        let mut moons = read_moons("<x=-1, y=0, z=2>;<x=2, y=-10, z=-7>;<x=4, y=-8, z=8>;<x=3, y=5, z=-1>".split(";"));
        assert_eq!(179, p1(&mut moons, 10));
    }

    #[test]
    fn p1_1() {
        let mut moons = read_moons("<x=-8, y=-10, z=0>;<x=5, y=5, z=10>;<x=2, y=-7, z=3>;<x=9, y=-8, z=-3>".split(";"));
        assert_eq!(1940, p1(&mut moons, 100));
    }
}
