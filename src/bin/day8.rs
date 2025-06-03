use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

struct AntennaMap {
    nrows: usize,
    ncols: usize,
    positions: HashMap<char, Vec<(usize, usize)>>,
}

fn read_input() -> AntennaMap {
    let path = Path::new("input/day8.txt");

    let file = File::open(&path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("Failed to read line")
    ).collect();

    let nrows = lines.len();
    let ncols = if nrows > 0 { lines[0].len() } else { 0 };

    let mut positions = HashMap::new();

    for (row_idx, line) in lines.iter().enumerate() {
        for (col_idx, c) in line.chars().enumerate() {
            if is_antenna(c) {
                positions
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push((row_idx, col_idx));
            }
        }
    }

    AntennaMap {
        nrows,
        ncols,
        positions,
    }
}

fn is_antenna(c: char) -> bool {
    c.is_ascii_alphanumeric()
}

fn part1(input: &AntennaMap) -> usize {
    let mut counted = HashSet::new();

    for (_c, positions) in &input.positions {
        let len = positions.len();
        for i in 0..len {
            for j in 0..len {
                if i == j {
                    continue;
                }
                let a = positions[i];
                let b = positions[j];
                // Compute the third point that is collinear and at twice the distance
                let x = 2 * b.0 as i32 - a.0 as i32;
                let y = 2 * b.1 as i32 - a.1 as i32;
                let c_point = (x as usize, y as usize);

                // Check if the point is within bounds and exists in the current positions
                if c_point.0 < input.nrows && c_point.1 < input.ncols {
                    counted.insert(c_point);
                }
            }
        }
    }

    counted.len()
}

fn main() {
    println!("--- Day 8: Resonant Collinearity ---");
    let input = read_input();

    let result = part1(&input);
    println!("Part 1 Solution: {}", result);
}
