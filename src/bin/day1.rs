use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
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
    
    // Sort both lists from smallest to largest
    list_a.sort();
    list_b.sort();
    
    // Calculate the total difference
    let mut total_difference = 0;
    
    // Go through both lists, compute the differences for each pair, e.g. A[i] - B[i], then sum them up
    for i in 0..list_a.len() {
        if i < list_b.len() {
            total_difference += (list_a[i] - list_b[i]).abs();
        }
    }
    
    // Print the result
    println!("Total difference: {}", total_difference);
}
