use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone)]
struct Marker {
    direction: Direction,
    row: usize,
    col: usize,
}

impl Marker {
    fn new(direction: Direction, row: usize, col: usize) -> Self {
        Self { direction, row, col }
    }

    fn from_char(c: char, row: usize, col: usize) -> Option<Self> {
        match c {
            '^' => Some(Self::new(Direction::Up, row, col)),
            '>' => Some(Self::new(Direction::Right, row, col)),
            'v' => Some(Self::new(Direction::Down, row, col)),
            '<' => Some(Self::new(Direction::Left, row, col)),
            _ => None,
        }
    }

    fn is_marker(c: char) -> bool {
        Self::from_char(c, 0, 0).is_some()
    }

    fn rotate(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }

    fn to_char(&self) -> char {
        match self.direction {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        }
    }
}

fn read_input() -> Vec<Vec<char>> {
    let path = Path::new("input/day6s.txt");

    // Open the file
    let file = File::open(&path).expect("Failed to open file");
    let reader = BufReader::new(file);

    // Read the file line by line and convert to array of arrays of characters
    reader.lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().collect())
        .collect()
}

fn find_the_marker(input: &[Vec<char>]) -> Option<(usize, usize)> {
    for (row_idx, row) in input.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if let Some(marker) = Marker::from_char(cell, row_idx, col_idx) {
                return Some((marker.row, marker.col));
            }
        }
    }
    None
}

fn part1(input: &[Vec<char>]) -> usize {
    find_the_marker(input).expect("No marker found").0 + 1 // Return the row index + 1
}

fn main() {
    let input = read_input();

    let result = part1(&input);
    println!("Part 1 result: {}", result);
}
