use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// Read the input as 2d array of characters
fn read_input() -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let path = Path::new("input/day5.txt");

    // Open the file
    let file = File::open(&path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut rules = Vec::new();
    let mut page_lists = Vec::new();

    // First read all the lines
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .filter(|line| !line.trim().is_empty()) // Skip empty lines
        .collect();

    // Find the dividing line between rules and page lists
    let split_index = lines
        .iter()
        .position(|line| line.contains(","))
        .unwrap_or(lines.len());

    // Read the rules
    for line in &lines[..split_index] {
        let parts: Vec<&str> = line.split("|").collect();
        if parts.len() == 2 {
            let first = parts[0].parse::<usize>().unwrap();
            let second = parts[1].parse::<usize>().unwrap();
            rules.push((first, second));
        }
    }

    // Read the page lists
    for line in &lines[split_index..] {
        let pages: Vec<usize> = line
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        page_lists.push(pages);
    }

    (rules, page_lists)
}

fn check_rules(rules: &[(usize, usize)], position_map: &HashMap<usize, usize>) -> bool {
    for &(a, b) in rules {
        if let (Some(&pos_a), Some(&pos_b)) = (position_map.get(&a), position_map.get(&b)) {
            if pos_a > pos_b {
                return false; // Rule is violated
            }
        }
    }
    true // All rules are satisfied
}

fn get_midpag_pos(page_list: &[usize]) -> usize {
    if page_list.is_empty() {
        return 0; // Return 0 if the list is empty
    }
    let mid_index = page_list.len() / 2;
    page_list[mid_index] // Return the middle page position
}

fn part1(rules: &[(usize, usize)], page_lists: &[Vec<usize>]) {
    // Create a map to store the valid positions for each number
    let mut position_map: HashMap<usize, usize> = HashMap::new();

    let mut valid_middle_sum = 0;

    // Process each page list to build the position map
    for page_list in page_lists {
        for (index, &page) in page_list.iter().enumerate() {
            position_map.insert(page, index);
        }

        if check_rules(rules, &position_map) {
            valid_middle_sum += get_midpag_pos(page_list);
        }
        // Clear the position map for the next page list
        position_map.clear();
    }

    println!(
        "Part 1 - Solution: {}",
        valid_middle_sum
    );
}

fn main() {
    let (rules, page_lists) = read_input();

    // Call part1 function
    part1(&rules, &page_lists);
}
