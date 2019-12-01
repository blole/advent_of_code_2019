use std::io::{self, Read};

fn read_ints() -> Vec<i32> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).expect("failed");
    buf.lines().map(|line| line.trim().parse::<i32>().unwrap()).collect()
}

fn main() {
    let module_masses = read_ints();
    let mut part1 = 0;
    for module_mass in &module_masses {
        part1 += module_mass/3-2;
    }
    println!("{}", part1);

    let mut part2 = 0;
    for module_mass in module_masses {
        let mut fuel = module_mass/3-2;
        while fuel > 0 {
            part2 += fuel;
            fuel = fuel/3-2;
        }
    }
    println!("{}", part2);
}
