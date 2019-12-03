use std::io;
use std::io::prelude::*;


fn run(mut v: Vec<usize>) -> Result<Vec<usize>, String> {
    let mut pc = 0;
    while v[pc] != 99 {
        match (v[pc], v[pc+1], v[pc+2], v[pc+3]) {
            (1,a,b,c) => v[c] = v[a] + v[b], //add
            (2,a,b,c) => v[c] = v[a] * v[b], //mul
            _ => return Err(format!("unknown opcode v[{}]={}", pc, v[pc])),
        }
        pc += 4;
    }
    Ok(v)
}

fn run2(mut v: Vec<usize>, input: usize) -> Result<usize, String> {
    v[1] = input/100;
    v[2] = input%100;
    Ok(run(v)?[0])
}

fn find_noun_verb(v: Vec<usize>, output: usize) -> Result<usize, String> {
    for input in 0..9999 {
        if run2(v.clone(), input)? == output {
            return Ok(input);
        }
    }
    return Err(format!("no (noun,verb) found for {}", output));
}

fn main() {
    let stdin = io::stdin();
    let input: Vec<usize> = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|word| word.parse::<usize>())
        .filter_map(Result::ok)
        .collect();
    
    let part1 = run2(input.clone(), 1202).unwrap();
    println!("{}", part1);

    let part2 = find_noun_verb(input, 19690720).unwrap();
    println!("{}", part2);
}





#[cfg(test)]
mod tests_day02 {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(run(vec![1,0,0,0,99]), Ok(vec![2,0,0,0,99]));
        assert_eq!(run(vec![2,3,0,3,99]), Ok(vec![2,3,0,6,99]));
        assert_eq!(run(vec![2,4,4,5,99,0]), Ok(vec![2,4,4,5,99,9801]));
        assert_eq!(run(vec![1,1,1,4,99,5,6,0,99]), Ok(vec![30,1,1,4,2,5,6,0,99]));
    }
}
