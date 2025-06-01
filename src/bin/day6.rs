use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr; // Added to make `from_str()` work with `strum::EnumString`
use strum::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoStaticStr, Display, EnumString)]
enum Direction {
    #[strum(serialize = "^")]
    Up,
    #[strum(serialize = ">")]
    Right,
    #[strum(serialize = "v")]
    Down,
    #[strum(serialize = "<")]
    Left,
}

#[derive(Debug, Clone)]
struct Guard {
    direction: Direction,
    row: i32,
    col: i32,
}

impl Guard {
    fn new(direction: Direction, row: i32, col: i32) -> Self {
        Self {
            direction,
            row,
            col,
        }
    }

    fn from_char(c: char, row: usize, col: usize) -> Option<Self> {
        Direction::from_str(&c.to_string())
            .ok()
            .map(|d| Self::new(d, row as i32, col as i32))
    }

    fn rotate(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }

    fn walk(&mut self) {
        // Move to its front position
        let (new_row, new_col) = self.front_position();
        self.row = new_row;
        self.col = new_col;
    }

    #[allow(dead_code)]
    fn to_char(&self) -> char {
        self.direction.to_string().chars().next().unwrap()
    }

    fn front_position(&self) -> (i32, i32) {
        // Calculate the position in front of the guard based on its direction
        match self.direction {
            Direction::Up => (self.row - 1, self.col),
            Direction::Right => (self.row, self.col + 1),
            Direction::Down => (self.row + 1, self.col),
            Direction::Left => (self.row, self.col - 1),
        }
    }
}

struct Map {
    grid: Vec<Vec<char>>,
    guard: Guard,
}

impl Map {
    /// Places an obstacle at the given grid position.
    /// If the position is out of bounds, does nothing.
    fn place_obstacle(&mut self, row: usize, col: usize) {
        if row < self.grid.len() && col < self.grid[row].len() {
            self.grid[row][col] = '#';
        }
    }

    /// Removes an obstacle at the given grid position, replacing it with a space.
    /// If the position is out of bounds or not an obstacle, does nothing.
    fn remove_obstacle(&mut self, row: usize, col: usize) {
        if row < self.grid.len() && col < self.grid[row].len() {
            if self.grid[row][col] == '#' {
                self.grid[row][col] = ' ';
            }
        }
    }

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

    fn is_obstacle(&self, row: i32, col: i32) -> bool {
        if row >= 0 && col >= 0 {
            let row = row as usize;
            let col = col as usize;
            if row < self.grid.len() && col < self.grid[row].len() {
                self.grid[row][col] == '#'
            } else {
                false
            }
        } else {
            false
        }
    }

    fn is_within_map(&self, row: i32, col: i32) -> bool {
        if row >= 0 && col >= 0 {
            let row = row as usize;
            let col = col as usize;
            row < self.grid.len() && col < self.grid[row].len()
        } else {
            false
        }
    }

    fn get_guard(&self) -> Guard {
        self.guard.clone()
    }
}

fn read_input() -> Vec<Vec<char>> {
    let path = Path::new("input/day6.txt");

    // Open the file
    let file = File::open(&path).expect("Failed to open file");
    let reader = BufReader::new(file);

    // Read the file line by line and convert to array of arrays of characters
    reader
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.chars().collect())
        .collect()
}

fn part1(input: &[Vec<char>]) -> usize {
    let map = Map::new(input.to_vec());
    let mut guard = map.get_guard();

    // Track visited positions
    let mut visited = std::collections::HashSet::<(i32, i32)>::new();

    // Continue moving the guard until it leaves the map or we detect a loop
    while map.is_within_map(guard.row, guard.col) {
        visited.insert((guard.row, guard.col));

        // Try to move forward
        let (new_row, new_col) = guard.front_position();

        if map.is_obstacle(new_row, new_col) {
            // Obstacle ahead, rotate and don't move
            guard.rotate();
        } else {
            guard.walk();
        }
    }

    // Return the number of unique positions visited
    visited.len()
}

fn part2(input: &[Vec<char>]) -> usize {
    let mut map = Map::new(input.to_vec());
    let mut count = 0;
    let guard_start_pos = (map.guard.row as usize, map.guard.col as usize);

    for row in 0..map.grid.len() {
        for col in 0..map.grid[row].len() {
            // Skip guard start position and existing obstacles
            if (row, col) == guard_start_pos || map.grid[row][col] == '#' {
                continue;
            }

            map.place_obstacle(row, col);
            
            let mut sim_guard = map.get_guard();
            let mut visited_states = std::collections::HashSet::new();

            while map.is_within_map(sim_guard.row, sim_guard.col) {
                let state = (sim_guard.row, sim_guard.col, sim_guard.direction);
                
                // Detect loop if we've seen this exact state before
                if visited_states.contains(&state) {
                    break;
                }
                visited_states.insert(state);

                let (new_row, new_col) = sim_guard.front_position();
                if map.is_obstacle(new_row, new_col) {
                    sim_guard.rotate();
                } else {
                    sim_guard.walk();
                }
            }

            // Count if guard is still in map (loop detected) or exited
            if map.is_within_map(sim_guard.row, sim_guard.col) {
                count += 1;
            }

            map.remove_obstacle(row, col);
        }
    }

    count
}

fn main() {
    let input = read_input();

    let result = part1(&input);
    println!("Part 1 result: {}", result);
    let result2 = part2(&input);
    println!("Part 2 result: {}", result2);
}
