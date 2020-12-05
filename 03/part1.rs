use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::process;

#[derive(PartialEq, Eq, Hash)]
struct MapPosition {
    r: usize,
    c: usize,
}

impl MapPosition {
    fn new(r: usize, c: usize) -> MapPosition {
        MapPosition { r, c }
    }
}

struct Map {
    num_rows: usize,
    num_cols: usize,
    trees: HashSet<MapPosition>,
}

impl Map {
    fn new(num_rows: usize, num_cols: usize) -> Map {
        Map {
            num_rows,
            num_cols,
            trees: HashSet::new(),
        }
    }

    fn add_tree(&mut self, point: MapPosition) {
        self.trees.insert(point);
    }

    fn navigate_toboggan(&self) -> usize {
        let mut num_trees = 0;
        let mut current_pos = MapPosition::new(0, 0);

        while current_pos.r != self.num_rows {
            current_pos.r += 1;
            current_pos.c = (current_pos.c + 3) % self.num_cols;

            if self.trees.contains(&current_pos) {
                num_trees += 1;
            }
        }

        num_trees
    }
}

fn encountered_trees_count(file_name: impl AsRef<Path>) -> usize {
    let file = File::open(file_name).unwrap();
    let lines = BufReader::new(file).lines();
    let lines: Vec<_> = lines.map(|x| x.unwrap()).collect();

    let mut map = Map::new(lines.len(), lines[0].chars().count());

    for (r, line) in lines.iter().enumerate() {
        for (c, character) in line.chars().enumerate() {
            if character == '#' {
                map.add_tree(MapPosition::new(r, c));
            }
        }
    }
    map.navigate_toboggan()
}

fn main() {
    if env::args().count() != 2 {
        eprintln!("USAGE: {} FILE", env::args().next().unwrap());
        process::exit(1);
    }

    let count = encountered_trees_count(env::args().nth(1).unwrap());
    println!("Result: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        assert_eq!(encountered_trees_count("example.txt"), 7);
    }

    #[test]
    fn test_puzzle_input() {
        assert_eq!(encountered_trees_count("input.txt"), 207);
    }
}
