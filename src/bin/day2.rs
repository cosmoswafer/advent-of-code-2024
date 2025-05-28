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
        if diff < 1 || diff > 3 {
            is_increasing = false;
        }
        if diff > -1 || diff < -3 {
            is_decreasing = false;
        }
    }
    
    is_increasing || is_decreasing
}

fn part1(reports: &[Vec<i32>]) {
    let mut safe_count = 0;
    
    for report in reports {
        if is_safe(&report) {
            safe_count += 1;
            println!("Report {:?} is safe", report);
        } else {
            println!("Report {:?} is NOT safe!", report);
        }
    }

    println!("Day 2 solution:");
    println!("Number of safe reports: {}", safe_count);
}

fn main() {
    println!("Reading input for Day 2...");
    let reports = read_input();

    part1(&reports);
}
