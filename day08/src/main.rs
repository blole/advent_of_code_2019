use std::io;
use std::io::prelude::*;

fn count(layer: &str, chr: char) -> usize {
    layer.as_bytes().iter()
        .filter(|c| **c == chr as u8)
        .count()
}

fn overlay(a: &String, b: &String) -> String {
    assert_eq!(a.len(), b.len());
    let bytes: Vec<u8> = a.as_bytes().iter()
        .zip(b.as_bytes().iter())
        .map(|(&x,&y)| if x!='2' as u8 {x} else {y})
        .collect();
    std::str::from_utf8(bytes.as_ref()).unwrap().to_string()
}

fn main() {
    let stdin = io::stdin();
    let input: String = stdin.lock().lines().filter_map(Result::ok).next().unwrap();

    let layers: Vec<String> = input.as_bytes()
        .chunks(25*6)
        .map(|s| unsafe { ::std::str::from_utf8_unchecked(s) }.to_string())
        .collect();
    
    let best = layers.iter()
        .min_by_key(|layer| count(layer,'0'))
        .unwrap();
        
    let part1 = count(best, '1') * count(best, '2');
    println!("{}", part1);


    let first = layers.first().unwrap().to_string();
    let overlaid = layers.iter().skip(1).fold(first, |a,b| overlay(&a,&b));
    //let part2 = layers.iter().reduce(|a,b| overlay(&a, &b)).unwrap();

    let part2: Vec<String> = overlaid.as_bytes()
        .chunks(25)
        .map(|s| unsafe { ::std::str::from_utf8_unchecked(s) }.to_string())
        .collect();
    

    for line in part2 {
        println!("{}", line);
    }
}
