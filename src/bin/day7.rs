use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn read_input() -> Vec<(i64, Vec<i64>)> {
    let path = Path::new("input/day7.txt");

    // Open the file
    let file = File::open(&path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut result = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() != 2 {
            continue;
        }

        let first_part = parts[0].parse::<i64>().ok();
        let second_part: Vec<i64> = parts[1]
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();

        if let Some(first) = first_part {
            result.push((first, second_part));
        }
    }

    result
}

fn part1(input: Vec<(i64, Vec<i64>)>) -> i64 {
    let mut count: i64 = 0;

    for (first, numbers) in input {
        let m = numbers.len();
        if m == 0 {
            continue;
        }

        if m == 1 {
            if numbers[0] == first {
                count += 1;
            }
            continue;
        }

        let mut valid = false;
        let num_ops = m - 1;

        for mask in 0..(1 << num_ops) {
            let mut ops = Vec::new();
            for i in 0..num_ops {
                if mask & (1 << i) != 0 {
                    ops.push('*');
                } else {
                    ops.push('+');
                }
            }

            let mut result = numbers[0];
            for i in 0..num_ops {
                let next = numbers[i + 1];
                match ops[i] {
                    '+' => result += next,
                    '*' => result *= next,
                    _ => panic!("Invalid operator"),
                }
            }

            if result == first {
                valid = true;
                break;
            }
        }

        if valid {
            count += first;
        }
    }

    count
}

fn main() {
    let input = read_input();

    let result = part1(input);
    println!("Part 1 result: {}", result);
}
