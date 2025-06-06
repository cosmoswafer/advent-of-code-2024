use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn read_input() -> Vec<char> {
    let path = Path::new("input/day9s.txt");

    let file = File::open(&path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let line = reader
        .lines()
        .next()
        .expect("No lines found")
        .expect("Failed to read line");
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
        // Find next free space from left
        while i < j && !matches!(file_blocks[i], FileBlock::FreeSpace) {
            i += 1;
        }
        // Find next file block from right
        while i < j && matches!(file_blocks[j], FileBlock::FreeSpace) {
            j -= 1;
        }
        // Swap and move pointers to void infinite loop
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
    
    // Print the initial file blocks for debugging
    println!("Initial file blocks:");
    for block in &file_blocks {
        match block {
            FileBlock::FileId(id) => print!("{}", id),
            FileBlock::FreeSpace => print!("."),
        }
    }
    println!("\n");
    
    // Start from the end of the array and move backwards
    let mut pos = file_blocks.len();
    
    // Process each file block from right to left
    while pos > 0 {
        // Find the position of the rightmost file block
        // Decrement pos first to point to the current element
        pos -= 1; 
        while pos > 0 && matches!(file_blocks[pos], FileBlock::FreeSpace) {
            pos -= 1;
        }
        
        if matches!(file_blocks[pos], FileBlock::FreeSpace) { // If we landed on a free space at index 0
            break;
        }
        
        // Find the start position of the current file block
        let file_end = pos; // pos is now the end of the current file block
        let mut file_start = file_end;
        while file_start > 0 {
            if let FileBlock::FileId(current_id) = file_blocks[file_start - 1] {
                if let FileBlock::FileId(prev_id) = file_blocks[file_start] {
                    if current_id == prev_id { // Check if it's the same file ID
                        file_start -= 1;
                    } else {
                        break; // Different file ID, so this is the start of the current file
                    }
                } else {
                    break; // Previous block is FreeSpace, so this is the start of the current file
                }
            } else {
                break; // Previous block is FreeSpace, so this is the start of the current file
            }
        }
        
        // Get the file ID and calculate its length
        let current_file_id = match file_blocks[file_start] {
            FileBlock::FileId(id) => id,
            FileBlock::FreeSpace => unreachable!(), // Should not happen if logic is correct
        };
        let file_length = file_end - file_start + 1;
        
        // Find the leftmost contiguous free space that can fit the file
        let mut free_start = 0;
        let mut found_suitable_spot = false;
        while free_start < file_start {
            // Find the start of a free space region
            while free_start < file_start && !matches!(file_blocks[free_start], FileBlock::FreeSpace) {
                free_start += 1;
            }
            
            if free_start >= file_start {
                break;
            }
            
            // Calculate the length of this free space region
            let mut free_end = free_start;
            while free_end < file_start && matches!(file_blocks[free_end], FileBlock::FreeSpace) {
                free_end += 1;
            }
            
            // Check if this free space can fit our file
            if free_end - free_start >= file_length {
                // We found a suitable spot - move the file there
                for i in 0..file_length {
                    file_blocks[free_start + i] = FileBlock::FileId(current_file_id);
                }
                
                // Mark the old positions as free space
                for i in file_start..=file_end {
                    file_blocks[i] = FileBlock::FreeSpace;
                }
                
                found_suitable_spot = true;
                // Break out of the free space search
                break;
            }
            
            // Move to the next potential free space region
            free_start = free_end;
        }
        
        // Print the updated file blocks for debugging
        println!("After moving file {}:", current_file_id);
        for block in &file_blocks {
            match block {
                FileBlock::FileId(id) => print!("{}", id),
                FileBlock::FreeSpace => print!("."),
            }
        }
        println!("\n");
        
        // Move left to the next file
        pos = file_start;
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
