use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn read_input() -> Vec<char> {
    let path = Path::new("input/day9.txt");

    let file = File::open(&path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let line = reader.lines().next().expect("No lines found").expect("Failed to read line");
    line.chars().collect()
}

fn part1(input: Vec<char>) -> usize {
    let mut file_blocks = Vec::new();
    let mut file_id = 0;
    let mut free_space = false;

    for c in input {
        let cdigit = c.to_digit(10).unwrap_or(0) as usize;
        if free_space {
            free_space = false;
            file_blocks.extend(std::iter::repeat('.').take(cdigit));
        } else {
            free_space = true;
            file_blocks.extend(std::iter::repeat(char::from_digit(file_id, 10).unwrap()).take(cdigit));
            file_id += 1;
        }
    }

    let mut i = 0;
    let mut j = file_blocks.len() - 1;

    while i < j {
        while file_blocks[i] != '.' && i < j {
            i += 1;
        }
        while file_blocks[j] == '.' && i < j {
            j -= 1;
        }
        file_blocks.swap(i, j);
    }

    let mut checksum = 0;
    for (i, c) in file_blocks.iter().enumerate() {
        if *c != '.' {
            checksum += c.to_digit(10).unwrap() as usize * (i + 0);
        }
    }

    checksum
}

fn main() {
    println!("--- Day 9: Disk Fragmenter ---");
    let input = read_input();
    let result = part1(input);
    println!("Part 1 Solution: {}", result);
}
