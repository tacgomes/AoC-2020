use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const TARGET: i32 = 2020;

fn main() {
    let f = File::open("input.txt").unwrap();
    let lines = BufReader::new(f).lines();

    let mut vec = vec![];
    let mut set = HashSet::new();

    for line in lines {
        let n = line.unwrap().parse::<i32>().unwrap();
        vec.push(n);
        set.insert(n);
    }

    // Assumes non-repeated elements
    assert_eq!(set.len(), vec.len());

    'outer: for (a_i, a) in vec[..vec.len() - 2].iter().enumerate() {
        for b in vec[a_i + 1..].iter() {
            let diff = TARGET - a - b;
            if set.contains(&diff) {
                println!("Result: {}", a * b * diff);
                break 'outer;
            }
        }
    }
}
