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

fn check_rules_against_subset(rules: &[(usize, usize)], position_map: &HashMap<usize, usize>) -> bool {
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

        if check_rules_against_subset(rules, &position_map) {
            valid_middle_sum += get_midpag_pos(page_list);
        }
    }

    println!("Part 1 - Solution: {}", valid_middle_sum);
}

fn topological_sort(rules: &[(usize, usize)], nodes: &[usize]) -> Option<Vec<usize>> {
    let node_set: std::collections::HashSet<_> = nodes.iter().cloned().collect();

    // Only consider rules that involve nodes in this page list
    let filtered_rules: Vec<_> = rules
        .iter()
        .filter(|&&(a, b)| node_set.contains(&a) && node_set.contains(&b))
        .cloned()
        .collect();

    // Build adjacency list and in-degree count
    let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut in_degree: HashMap<usize, usize> = HashMap::new();
    
    // Initialize all nodes
    for &node in nodes {
        adj.insert(node, Vec::new());
        in_degree.insert(node, 0);
    }
    
    // Add edges from filtered rules
    for &(a, b) in &filtered_rules {
        adj.entry(a).or_default().push(b);
        *in_degree.entry(b).or_default() += 1;
    }
    
    // Kahn's algorithm
    let mut queue: Vec<usize> = in_degree.iter()
        .filter(|&(_, &count)| count == 0)
        .map(|(&node, _)| node)
        .collect();
    
    let mut result = Vec::new();
    
    while !queue.is_empty() {
        let node = queue.pop()?;
        result.push(node);
        
        for neighbor in adj.get(&node).unwrap_or(&Vec::new()) {
            let degree = in_degree.get_mut(neighbor)?;
            *degree -= 1;
            if *degree == 0 {
                queue.push(*neighbor);
            }
        }
    }
    
    // Check if all nodes were visited (no cycles)
    if result.len() != nodes.len() {
        None // Cycle detected
    } else {
        Some(result)
    }
}

fn part2(rules: &[(usize, usize)], page_lists: &[Vec<usize>]) {
    let mut fixed_middle_sum = 0;
    
    for page_list in page_lists {
        // Skip if the list is already valid
        let position_map = build_position_map(page_list);
        if check_rules_against_subset(rules, &position_map) {
            continue;
        }
        
        // Extract unique nodes from this page list
        let nodes: Vec<usize> = page_list.iter().cloned().collect();
        
        // Perform topological sort using only relevant rules
        if let Some(sorted) = topological_sort(rules, &nodes) {
            // Create new position map based on topological sort
            let new_position_map = build_position_map(&sorted);
            
            // Only consider rules relevant to this page list
            let node_set: std::collections::HashSet<_> = nodes.iter().cloned().collect();
            let relevant_rules: Vec<_> = rules
                .iter()
                .filter(|&&(a, b)| node_set.contains(&a) && node_set.contains(&b))
                .cloned()
                .collect();

            // Check if the new ordering satisfies all relevant rules
            if check_rules_against_subset(&relevant_rules, &new_position_map) {
                fixed_middle_sum += get_midpag_pos(&sorted);
            }
        }
    }

    println!("Part 2 - Solution: {}", fixed_middle_sum);
}

fn main() {
    let (rules, page_lists) = read_input();

    part1(&rules, &page_lists);
    part2(&rules, &page_lists);
}
