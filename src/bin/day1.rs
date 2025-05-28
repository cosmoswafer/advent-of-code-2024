use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn read_input() -> (Vec<i32>, Vec<i32>) {
    // Read input from `day1/input` file on the current path
    let path = Path::new("input/day1.txt");
    
    // Open the file
    let file = File::open(&path).expect("Failed to open file");
    
    // Create a buffered reader
    let reader = io::BufReader::new(file);
    
    // Vectors to store the numbers from each column
    let mut list_a: Vec<i32> = Vec::new();
    let mut list_b: Vec<i32> = Vec::new();
    
    // Read each line from the file
    for line in reader.lines() {
        if let Ok(ip) = line {
            // Parse the line into integers
            let numbers: Vec<i32> = ip
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            
            // Ensure we have exactly two numbers
            if numbers.len() == 2 {
                list_a.push(numbers[0]);
                list_b.push(numbers[1]);
            }
        }
    }
    
    // Return the two lists instead of the computed result
    (list_a, list_b)
}

fn part1(list_a: &Vec<i32>, list_b: &Vec<i32>) {
    // Create copies of the lists so we don't modify the original ones
    let mut list_a_copy = list_a.clone();
    let mut list_b_copy = list_b.clone();
    
    // Sort both lists from smallest to largest
    list_a_copy.sort();
    list_b_copy.sort();

    // Calculate the total difference using the passed lists
    let mut total_difference = 0;
    
    // Go through both lists, compute the differences for each pair, e.g. A[i] - B[i], then sum them up
    for i in 0..list_a_copy.len() {
        if i < list_b_copy.len() {
            total_difference += (list_a_copy[i] - list_b_copy[i]).abs();
        }
    }
    
    // Print the result
    println!("Part 1 answer: {}", total_difference);
}

fn part2(list_a: &Vec<i32>, list_b: &Vec<i32>) {
    // Create a frequency map for list B
    let mut b_freq = HashMap::new();
    
    // Count occurrences of each number in list B
    for &num in list_b {
        *b_freq.entry(num).or_insert(0) += 1;
    }
    
    // Calculate the similarity score
    let mut similarity_score = 0;
    
    // For each number in list A, multiply its count in B by the number itself
    for &num in list_a {
        if let Some(&count) = b_freq.get(&num) {
            similarity_score += num * count;
        }
    }
    
    // Print the result
    println!("Part 2 answer: {}", similarity_score);
}

fn main() {
    println!("Reading input for Day 1...");
    let (list_a, list_b) = read_input();
    println!("Day 1 solution:");
    part1(&list_a, &list_b);
    
    part2(&list_a, &list_b);
}
