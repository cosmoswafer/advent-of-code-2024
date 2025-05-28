# Advent of Code 2024 - Rust Solutions

This repository contains my Rust solutions for the Advent of Code 2024 challenges. Each day's solution is organized in its own binary crate within the `src/bin` directory.

## Project Structure

- `src/bin/dayX.rs`: Solution for Day X (where X is the day number)
- `input/dayX.txt`: Input file for Day X (where X is the day number)

Each day follows a consistent pattern:
- The solution code is in a separate binary crate under `src/bin`
- Input files are stored in the `input` directory with matching day numbers
- Each solution can be run independently using Cargo

## Running the Solutions

To run a specific day's solution, use the following command:

```bash
cargo run --bin dayX
```

Replace `X` with the day number you want to run.
