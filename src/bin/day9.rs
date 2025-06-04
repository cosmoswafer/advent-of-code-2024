use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn read_input() -> Vec<char> {
    let path = Path::new("input/day9s.txt");

    let file = File::open(&path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let line = reader.lines().next().expect("No lines found").expect("Failed to read line");
    line.chars().collect()
}

#[derive(Clone, Debug)]
enum FileBlock {
    FileId(usize),
    FreeSpace,
}

fn part1(input: &Vec<char>) -> usize {
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

fn part2(input: &Vec<char>) -> usize {
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
    println!("Initial file blocks: {:?}", file_blocks);
    
    let mut free_pos = 0;
    while free_pos < file_blocks.len() {
        // Find next free space
        while free_pos < file_blocks.len() && !matches!(file_blocks[free_pos], FileBlock::FreeSpace) {
            free_pos += 1;
        }
        if free_pos >= file_blocks.len() {
            break;
        }
        
        // Find the length of this free space
        let mut free_len = 0;
        while free_pos + free_len < file_blocks.len() && matches!(file_blocks[free_pos + free_len], FileBlock::FreeSpace) {
            free_len += 1;
        }
        
        // Find the rightmost file block that can fit in this free space
        let mut found = false;
        for block_pos in (free_pos + free_len..file_blocks.len()).rev() {
            // Check if this is the start of a file block
            if matches!(file_blocks[block_pos], FileBlock::FileId(_)) {
                // Count the length of this file block
                let mut block_len = 1;
                let file_id = match file_blocks[block_pos] {
                    FileBlock::FileId(id) => id,
                    _ => panic!("Shouldn't happen"),
                };
                while block_pos + block_len < file_blocks.len() && 
                      matches!(file_blocks[block_pos + block_len], FileBlock::FileId(id) if id == file_id) {
                    block_len += 1;
                }
                
                // If this block can fit in our free space
                if block_len <= free_len {
                    // Swap the free space with this block
                    for i in 0..block_len {
                        file_blocks.swap(free_pos + i, block_pos + i);
                    }
                    println!("Swapped file block: {:?}", file_blocks);
                    // Update our position to after this block
                    free_pos += block_len;
                    found = true;
                    break;
                }
            }
        }
        
        // If no suitable block was found for this free space,
        // move past this free space
        if !found {
            free_pos += free_len;
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
    let result = part1(&input);
    println!("Part 1 Solution: {}", result);
    let result = part2(&input);
    println!("Part 2 Solution: {}", result);
}
