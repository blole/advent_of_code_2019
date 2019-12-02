use std::io::{self, Read};

fn fuel_required(mass: i32) -> i32 {
    mass/3-2
}

fn total_fuel_required(mass: i32) -> i32 {
    let mut total = 0;
    let mut fuel = fuel_required(mass);
    while fuel > 0 {
        total += fuel;
        fuel = fuel_required(fuel);
    }
    total
}

fn read_ints() -> Vec<i32> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("failed");
    buf.lines().map(|line| line.trim().parse::<i32>().unwrap()).collect()
}

fn main() {
    let module_masses = read_ints();
    let mut part1 = 0;
    for &module_mass in &module_masses {
        part1 += fuel_required(module_mass);
    }
    println!("{}", part1);

    let mut part2 = 0;
    for module_mass in module_masses {
        part2 += total_fuel_required(module_mass);
    }
    println!("{}", part2);
}





#[cfg(test)]
mod tests_day01 {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 654);
        assert_eq!(fuel_required(100756), 33583);
    }

    #[test]
    fn p2() {
        assert_eq!(total_fuel_required(14), 2);
        assert_eq!(total_fuel_required(1969), 966);
        assert_eq!(total_fuel_required(100756), 50346);
    }
}
