use std::fs::File;
use std::io::Read;
use std::path::Path;

// Read the input as 2d array of characters
fn read_input() -> Vec<Vec<char>> {
    let path = Path::new("input/day4.txt");

    // Open the file
    let file = File::open(&path).expect("Failed to open file");

    // Read the entire content as a string
    let mut content = String::new();
    file.take(u64::MAX)
        .read_to_string(&mut content)
        .expect("Failed to read file");

    // Convert to 2D array of characters
    content.lines().map(|line| line.chars().collect()).collect()
}

fn is_xmas(words: Vec<char>) -> bool {
    if words.is_empty() {
        return false;
    }
    if words.len() < 3 {
        return false;
    }
    // Check if the word is a valid XMAS word
    let word = words.iter().collect::<String>();
    if word == "XMAS" {
        return true;
    } else {
        return false;
    }
}

fn part1(content: &Vec<Vec<char>>) {
    let mut sum = 0;
    let mut xmas_coords: Vec<(usize, usize)> = Vec::new();

    for i in 0..content.len() {
        for j in 0..content[i].len() {
            // Check horizontal lines (left to right)
            if j + 4 <= content[i].len() && is_xmas(content[i][j..j + 4].to_vec()) {
                for k in 0..4 {
                    xmas_coords.push((i, j + k));
                }
                sum += 1;
            }
            // Check vertical lines (top to bottom)
            if i + 3 < content.len() {
                let mut column_chars = Vec::new();
                for k in 0..4 {
                    column_chars.push(content[i + k][j]);
                }
                if is_xmas(column_chars) {
                    for k in 0..4 {
                        xmas_coords.push((i + k, j));
                    }
                    sum += 1;
                }
            }
            // Check horizontal lines (right to left)
            if j >= 3 {
                // Reverse the characters before checking
                let mut reversed = content[i][j - 3..j + 1].to_vec();
                reversed.reverse();
                if is_xmas(reversed) {
                    for k in 0..4 {
                        xmas_coords.push((i, j - k));
                    }
                    sum += 1;
                }
            }
            // Check vertical lines (bottom to top)
            if i >= 3 {
                // Extract characters from bottom to top
                let mut column_chars = Vec::new();
                for k in 0..4 {
                    column_chars.push(content[i - k][j]);
                }

                // Check if we found XMAS
                if is_xmas(column_chars) {
                    for k in 0..4 {
                        xmas_coords.push((i - k, j));
                    }
                    sum += 1;
                }
            }
            // Check diagonal lines (top-left to bottom-right)
            if i + 3 < content.len() && j + 3 < content[i].len() {
                let mut diagonal_chars = Vec::new();
                for k in 0..4 {
                    diagonal_chars.push(content[i + k][j + k]);
                }
                if is_xmas(diagonal_chars) {
                    for k in 0..4 {
                        xmas_coords.push((i + k, j + k));
                    }
                    sum += 1;
                }
            }
            // Check diagonal lines (top-right to bottom-left)
            if i + 3 < content.len() && j >= 3 {
                let mut diagonal_chars = Vec::new();
                for k in 0..4 {
                    diagonal_chars.push(content[i + k][j - k]);
                }
                if is_xmas(diagonal_chars) {
                    for k in 0..4 {
                        xmas_coords.push((i + k, j - k));
                    }
                    sum += 1;
                }
            }
            // Check diagonal lines (bottom-right to top-left)
            if i >= 3 && j >= 3 {
                let mut diagonal_chars = Vec::new();
                for k in 0..4 {
                    diagonal_chars.push(content[i - k][j - k]);
                }
                if is_xmas(diagonal_chars) {
                    for k in 0..4 {
                        xmas_coords.push((i - k, j - k));
                    }
                    sum += 1;
                }
            }
            // Check diagonal lines (bottom-left to top-right)
            if i >= 3 && j + 3 < content[i].len() {
                let mut diagonal_chars = Vec::new();
                for k in 0..4 {
                    diagonal_chars.push(content[i - k][j + k]);
                }
                if is_xmas(diagonal_chars) {
                    for k in 0..4 {
                        xmas_coords.push((i - k, j + k));
                    }
                    sum += 1;
                }
            }
        }
    }

    // Create a grid with only the XMAS characters visible
    let mut display_grid: Vec<Vec<char>> = vec![vec!['.'; content[0].len()]; content.len()];

    for (i, j) in xmas_coords {
        display_grid[i][j] = content[i][j];
    }

    // Print the display grid
    /*
    println!("\nXMAS locations:");
    for row in display_grid {
        println!("{}", row.iter().collect::<String>());
    }
    */
    println!("Day 4 solution - Part 1: {}", sum);
}

fn is_cross_max(lt: char, rt: char, lb: char, rb: char) -> bool {
    match (lt, rt, lb, rb) {
        ('M', 'M', 'S', 'S') => true,
        ('S', 'S', 'M', 'M') => true,
        ('M', 'S', 'M', 'S') => true,
        ('S', 'M', 'S', 'M') => true,
        _ => false,
    }
}

fn part2(content: &Vec<Vec<char>>) {
    let mut sum = 0;

    for i in 1..content.len() - 1 {
        for j in 1..content[i].len() - 1 {
            if content[i][j] == 'A' {
                // Check if the current character is 'A'
                // and if it forms a cross with 'M' and 'S'
                if is_cross_max(
                    content[i - 1][j - 1],
                    content[i - 1][j + 1],
                    content[i + 1][j - 1],
                    content[i + 1][j + 1],
                ) {
                    sum += 1;
                }
            }
        }
    }

    println!("Day 4 solution - Part 2: {}", sum);
}

fn main() {
    let content = read_input();

    // Call part 1
    part1(&content);

    // Call part 2 (&content);
    part2(&content);
}
