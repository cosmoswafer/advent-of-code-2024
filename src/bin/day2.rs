use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Read input and convert each line to a vector of integers
fn read_input() -> Vec<Vec<i32>> {
    // Read input from `day2.txt` file
    let path = Path::new("input/day2.txt");
    
    // Open the file
    let file = File::open(&path).expect("Failed to open file");
    
    // Create a buffered reader
    let reader = io::BufReader::new(file);

    // Read all lines, parse each line into integers
    reader
        .lines()
        .map(|line_result| {
            line_result
                .expect("Failed to read line")
                .split_whitespace()
                .map(|s| s.parse::<i32>().expect("Failed to parse integer"))
                .collect()
        })
        .collect()
}

fn is_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
        return true; // A report with 0 or 1 level is trivially safe
    }
    
    // Check if all differences are between 1 and 3 (inclusive)
    let mut is_increasing = true;
    let mut is_decreasing = true;
    
    for i in 1..report.len() {
        let diff = report[i] - report[i-1];
        
        // Check for increasing pattern (1-3)
        if diff >= 1 && diff <= 3 {
            is_decreasing = false; // Can't be both increasing and decreasing
        } 
        // Check for decreasing pattern (-3 to -1)
        else if diff >= -3 && diff <= -1 {
            is_increasing = false; // Can't be both increasing and decreasing
        }
        else {
            // Not safe in either pattern
            return false;
        }
    }
    
    is_increasing || is_decreasing
}

fn part1(reports: &[Vec<i32>]) {
    let mut safe_count = 0;
    
    for report in reports {
        if is_safe(report) {
            safe_count += 1;
        }
    }

    println!("Day 2 solution - Part 1:");
    println!("Number of safe reports: {}", safe_count);
}

fn part2(reports: &[Vec<i32>]) {
    let mut safe_count = 0;
    
    for report in reports {
        // Check if report is already safe
        if is_safe(report) {
            safe_count += 1;
            continue;
        }
        
        // Try removing each element one by one and check if the resulting report is safe
        for i in 0..report.len() {
            // Create a new report without the i-th element
            let mut modified_report = Vec::with_capacity(report.len() - 1);
            for (idx, &val) in report.iter().enumerate() {
                if idx != i {
                    modified_report.push(val);
                }
            }
            
            if is_safe(&modified_report) {
                safe_count += 1;
                break; // No need to try other removals
            }
        }
    }
    
    println!("Day 2 solution - Part 2:");
    println!("Number of safe reports with problem dampener: {}", safe_count);
}

fn main() {
    println!("Reading input for Day 2...");
    let reports = read_input();

    part1(&reports);
    part2(&reports);
}
