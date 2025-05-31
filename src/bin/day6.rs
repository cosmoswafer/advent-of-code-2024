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
struct Guard {
    direction: Direction,
    row: usize,
    col: usize,
}

impl Guard {
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

    fn is_guard(c: char) -> bool {
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

struct Map {
    grid: Vec<Vec<char>>,
    guard: Guard,
}

impl Map {
    fn new(grid: Vec<Vec<char>>) -> Self {
        // Find the guard in the grid
        let mut guard = None;
        
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                if let Some(g) = Guard::from_char(cell, row_idx, col_idx) {
                    guard = Some(g);
                    break;
                }
            }
            if guard.is_some() {
                break;
            }
        }
        
        Self {
            grid,
            guard: guard.expect("No guard found in the map"),
        }
    }
    
    fn is_obstacle(&self, row: usize, col: usize) -> bool {
        if row < self.grid.len() && col < self.grid[row].len() {
            self.grid[row][col] == '#'
        } else {
            false
        }
    }
    
    fn guard_is_within_map(&self) -> bool {
        self.guard.row < self.grid.len() && self.guard.col < self.grid[self.guard.row].len()
    }
    
    fn get_guard(&self) -> Guard {
        self.guard.clone()
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

fn part1(input: &[Vec<char>]) -> usize {
    let map = Map::new(input.to_vec());
    let guard = map.get_guard();
    
    // Just returning the guard's row for demonstration
    guard.row + 1 // Adding 1 because the problem likely expects 1-based indexing
}

fn main() {
    let input = read_input();

    let result = part1(&input);
    println!("Part 1 result: {}", result);
}
