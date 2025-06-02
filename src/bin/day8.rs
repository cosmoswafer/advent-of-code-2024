use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn read_input() -> Vec<Vec<char>> {
    let path = Path::new("input/day8s.txt");

    // Open the file
    let file = File::open(&path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut output = Vec::new();
    // Read the input file into a vector of vectors of characters
    for line_result in reader.lines() {
        let line = line_result.expect("Failed to read line");
        output.push(line.chars().collect());
    }

    output
}

fn is_antenna(c: char) -> bool {
    // Return true if the character is digit/uppercase/lowercase
    c.is_ascii_alphanumeric()
}

fn part1(input: &Vec<Vec<char>>) -> usize {
    let mut count = 0;

    // Go through each character in the input
    for (row_idx, row) in input.iter().enumerate() {
        for (col_idx, &c) in row.iter().enumerate() {
            if is_antenna(c) {
                // Check horizontal (left and right)
                for x in 0..col_idx {
                    if input[row_idx][x] == c {
                        count += 1;
                    }
                }
                for x in col_idx + 1..input[row_idx].len() {
                    if input[row_idx][x] == c {
                        count += 1;
                    }
                }

                // Check vertical (up and down)
                for y in 0..row_idx {
                    if input[y][col_idx] == c {
                        count += 1;
                    }
                }
                for y in row_idx + 1..input.len() {
                    if input[y][col_idx] == c {
                        count += 1;
                    }
                }

                // Check diagonal (top-left to bottom-right)
                let mut y = row_idx as i32 - 1;
                let mut x = col_idx as i32 - 1;
                while y >= 0 && x >= 0 {
                    if input[y as usize][x as usize] == c {
                        count += 1;
                    }
                    y -= 1;
                    x -= 1;
                }
                y = row_idx as i32 + 1;
                x = col_idx as i32 + 1;
                while y < input.len() as i32 && x < input[y as usize].len() as i32 {
                    if input[y as usize][x as usize] == c {
                        count += 1;
                    }
                    y += 1;
                    x += 1;
                }

                // Check diagonal (top-right to bottom-left)
                y = row_idx as i32 - 1;
                x = col_idx as i32 + 1;
                while y >= 0 && x < input[y as usize].len() as i32 {
                    if input[y as usize][x as usize] == c {
                        count += 1;
                    }
                    y -= 1;
                    x += 1;
                }
                y = row_idx as i32 + 1;
                x = col_idx as i32 - 1;
                while y < input.len() as i32 && x >= 0 {
                    if input[y as usize][x as usize] == c {
                        count += 1;
                    }
                    y += 1;
                    x -= 1;
                }
            }
        }
    }

    count
}

fn main() {
    println!("--- Day 8: Resonant Collinearity ---");
    let input = read_input();

    let result = part1(&input);
    println!("Part 1 Solution: {}", result);
}
