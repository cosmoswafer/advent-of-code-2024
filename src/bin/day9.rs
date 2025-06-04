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

#[derive(Clone)]
enum FileBlock {
    FileId(usize),
    FreeSpace,
}

fn part1(input: Vec<char>) -> usize {
    let mut file_blocks = Vec::new();
    let mut file_id = 0;
    let mut free_space = false;

    for c in input {
        let cdigit = c.to_digit(10).unwrap_or(0) as usize;
        if free_space {
            free_space = false;
            file_blocks.extend(std::iter::repeat(FileBlock::FreeSpace).take(cdigit));
        } else {
            free_space = true;
            file_blocks.extend(std::iter::repeat(FileBlock::FileId(file_id)).take(cdigit));
            file_id += 1;
        }
    }

    let mut i = 0;
    let mut j = file_blocks.len() - 1;

    while i < j {
        // Find next file block from left
        while i < j && !matches!(file_blocks[i], FileBlock::FreeSpace) {
            i += 1;
        }
        // Find next free space from right
        while i < j && matches!(file_blocks[j], FileBlock::FreeSpace) {
            j -= 1;
        }
        // Swap and move pointers
        if i < j {
            file_blocks.swap(i, j);
            i += 1;
            j -= 1;
        }
    }

    let mut checksum = 0;
    for (idx, block) in file_blocks.iter().enumerate() {
        if let FileBlock::FileId(id) = block {
            checksum += idx * *id;
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
