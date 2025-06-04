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

#[derive(Clone, Copy)] // FileBlock can be Copy as usize is Copy
enum FileBlock {
    FileId(usize),
    FreeSpace,
}

fn part1(input: Vec<char>) -> usize {
    let mut file_blocks = Vec::new();
    let mut file_id_counter = 0; // Renamed to avoid confusion with loop variable 'file_id'
    let mut is_currently_free_space_segment = false; // Renamed for clarity

    for c in input {
        let cdigit = c.to_digit(10).unwrap_or(0) as usize;
        if is_currently_free_space_segment {
            is_currently_free_space_segment = false;
            file_blocks.extend(std::iter::repeat(FileBlock::FreeSpace).take(cdigit));
        } else {
            is_currently_free_space_segment = true;
            file_blocks.extend(std::iter::repeat(FileBlock::FileId(file_id_counter)).take(cdigit));
            file_id_counter += 1;
        }
    }

    // Defragment the blocks: keep only FileId blocks, preserving their relative order.
    // The original swapping loop was incorrect and could lead to an infinite loop.
    file_blocks.retain(|block| matches!(block, FileBlock::FileId(_)));

    let mut checksum = 0;
    for (i, block) in file_blocks.iter().enumerate() {
        // After retain, all blocks are guaranteed to be FileId.
        // The original `if let` is fine, but we can be more direct.
        match block {
            FileBlock::FileId(id) => {
                checksum += *id * i;
            }
            FileBlock::FreeSpace => {
                // This case should not be reached after the retain operation.
                // For robustness in a larger application, one might log an error or panic.
                // In an AoC context, it's often assumed unreachable.
            }
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
