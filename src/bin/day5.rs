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

fn build_position_map(page_list: &[usize]) -> HashMap<usize, usize> {
    let mut position_map = HashMap::new();
    for (index, &page) in page_list.iter().enumerate() {
        position_map.insert(page, index);
    }
    position_map
}

fn part1(rules: &[(usize, usize)], page_lists: &[Vec<usize>]) {
    let mut valid_middle_sum = 0;

    // Process each page list to build the position map
    for page_list in page_lists {
        let position_map = build_position_map(page_list);

        if check_rules(rules, &position_map) {
            valid_middle_sum += get_midpag_pos(page_list);
        }
    }

    println!("Part 1 - Solution: {}", valid_middle_sum);
}

fn fix_by_rules(rules: &[(usize, usize)], position_map: &mut HashMap<usize, usize>) -> bool {
    let mut changes_made = false;
    
    for &(a, b) in rules {
        if let (Some(&pos_a), Some(&pos_b)) = (position_map.get(&a), position_map.get(&b)) {
            if pos_a > pos_b {
                // Swap positions to fix the rule
                position_map.insert(a, pos_b);
                position_map.insert(b, pos_a);
                changes_made = true;
            }
        }
    }
    
    changes_made
}

fn compose_page_list(position_map: &HashMap<usize, usize>) -> Vec<usize> {
    let mut page_list: Vec<usize> = position_map.keys().cloned().collect();
    page_list.sort_by_key(|&k| position_map.get(&k).unwrap_or(&usize::MAX));
    page_list
}

fn part2(rules: &[(usize, usize)], page_lists: &[Vec<usize>]) {
    let mut fixed_middle_sum = 0;
    
    for page_list in page_lists {
        let mut position_map = build_position_map(page_list);
        let mut iteration_count = 0;
        let max_iterations = 100; // Safety limit to prevent infinite loops
        
        if check_rules(rules, &position_map) {
            continue;
        }

        // Keep applying rules until all violations are fixed
        while fix_by_rules(rules, &mut position_map) && iteration_count < max_iterations {
            iteration_count += 1;
        }
        
        if !check_rules(rules, &position_map) {
            // If rules are still not satisfied after fixing, skip this update
            continue;
        }
        
        fixed_middle_sum += get_midpag_pos(&compose_page_list(&position_map));
    }

    println!("Part 2 - Solution: {}", fixed_middle_sum);
}

fn main() {
    let (rules, page_lists) = read_input();

    part1(&rules, &page_lists);
    part2(&rules, &page_lists);
}
