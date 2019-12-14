use std::io;
use std::io::prelude::*;
use std::collections::{HashMap};
use std::fmt;

#[derive(Debug)]
struct Orbit {
    center: String,
    satellite: String,
}

struct Planet<'a> {
    name: String,
    parent: Option<&'a Planet<'a>>,
    satellites: Vec<&'a Planet<'a>>,
}

impl<'a> fmt::Debug for Planet<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}){}){:?}",
            self.parent.map(|p| p.name.to_string()).unwrap_or("root".to_string()),
            self.name.to_string(),
            self.satellites.iter().map(|s| s.name.to_string()).collect::<Vec<_>>()
        )
    }
}

impl<'a> Planet<'a> {
    fn new(name: String) -> Planet<'a> {
        Planet { name, parent: None, satellites: Vec::new() }
    }

    fn p1(&self, depth: i32) -> i32 {
        self.satellites.iter().map(|s| s.p1(depth+1)).sum::<i32>() + depth
    }

    fn add_ancestors(&'a self, v: &mut Vec<&Planet<'a>>) {
        v.push(self);
        self.parent.map(|p| p.add_ancestors(v));
    }

    fn ancestors(&self) -> Vec<&Planet> {
        let mut v = Vec::new();
        self.add_ancestors(&mut v);
        v.reverse();
        v
    }
}

fn orbit(line: String) -> Option<Orbit> {
    let mut iter = line.split(')');
    Some(Orbit { center: iter.next()?.into(), satellite: iter.next()?.into() })
}

fn orbits<I: Iterator<Item=S>, S: Into<String>>(iter: I) -> Vec<Orbit> {
    iter
        .map(|s| orbit(s.into()))
        .filter_map(|x| x)
        .collect()
}

fn p1(p: &Planet) -> i32 {
    p.p1(0)
}

fn p2(a: &Planet, b: &Planet) -> usize {
    let aa = a.ancestors();
    let ba = b.ancestors();
    let common_ancestors = aa.iter().zip(ba.iter()).take_while(|(x,y)| x.name==y.name).count();
    aa.len() + ba.len() - common_ancestors*2 - 2
}

fn read_satellites<'a>(orbits: &Vec<Orbit>) -> HashMap::<String, Planet<'a>> {
    let mut satellites = HashMap::<String, Planet<'a>>::with_capacity(orbits.len()*2);
    for Orbit { center: c, satellite: s } in orbits.iter() {
        unsafe {
            let center    = satellites.entry(c.to_string()).or_insert_with(|| Planet::new(c.to_string())) as *mut Planet;
            let satellite = satellites.entry(s.to_string()).or_insert_with(|| Planet::new(s.to_string())) as *mut Planet;
            (*center).satellites.push(&*satellite);
            (*satellite).parent = Some(&*center);
        }
    }
    satellites
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().filter_map(Result::ok);
    let orbits: Vec<_> = orbits(lines);
    let satellites = read_satellites(&orbits);

    let part1 = p1(&satellites["COM"]);
    let part2 = p2(&satellites["YOU"], &satellites["SAN"]);
    println!("{}", part1);
    println!("{}", part2);
}





#[cfg(test)]
mod tests_day06 {
    use super::*;

    #[test]
    fn p1_0() {
        let orbits = orbits("COM)B B)C C)D D)E E)F B)G G)H D)I E)J J)K K)L".split(" "));
        let satellites = read_satellites(&orbits);
        assert_eq!(42, p1(&satellites["COM"]));
    }

    #[test]
    fn p2_0() {
        let orbits = orbits("COM)B B)C C)D D)E E)F B)G G)H D)I E)J J)K K)L K)YOU I)SAN".split(" "));
        let satellites = read_satellites(&orbits);
        assert_eq!(4, p2(&satellites["YOU"], &satellites["SAN"]));
    }
}
