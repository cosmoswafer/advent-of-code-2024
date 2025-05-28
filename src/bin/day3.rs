use std::path::Path;
use std::fs::File;
use std::io::Read;

fn read_input() -> String {
    let path = Path::new("input/day3.txt");
    
    // Open the file
    let file = File::open(&path).expect("Failed to open file");
    
    // Read the entire content as a string
    let mut content = String::new();
    file.take(u64::MAX).read_to_string(&mut content).expect("Failed to read file");

    content
}

fn part1() {
    let mut memories = Vec::new();
    let content = read_input();
    
    // Search for "mul(" in the content
    for i in 0..content.len() {
        if i + 4 <= content.len() && &content[i..i+4] == "mul(" {
            // Found "mul(" - now look for closing parenthesis
            for j in i+4..content.len() {
                if content.chars().nth(j) == Some(')') {
                    // Extract content between "mul(" and ")"
                    let content_str = &content[i+4..j];
                    let parts: Vec<&str> = content_str.split(',').collect();
                    
                    if parts.len() == 2 {
                        if let (Ok(a), Ok(b)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                            memories.push(vec![a, b]);
                        }
                    }
                    break;
                }
            }
        }
    }

    // Calculate the sum of products for each memory
    let sum: i32 = memories.iter()
        .map(|pair| pair[0] * pair[1])
        .sum();
    
    println!("Day 3 solution - Part 1: {}", sum);
}

fn main() {
    part1();
    // part2(&reports);
}
