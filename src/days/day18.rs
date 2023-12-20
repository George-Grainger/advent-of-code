use self::Direction::*;
use advent_of_code::Day;
use std::{
    fmt::Display,
    fmt::{Error, Formatter},
};

const SRC: &str = include_str!("../../input/test.txt");

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct InvalidDirectionError;

impl Display for InvalidDirectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "Invalid direction, should be one of ['U', 'R', 'D', 'L']"
        )
    }
}

impl TryFrom<&str> for Direction {
    type Error = InvalidDirectionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "U" => Ok(North),
            "R" => Ok(East),
            "D" => Ok(South),
            "L" => Ok(West),
            _ => Err(InvalidDirectionError),
        }
    }
}

struct GridState {
    height: usize,
    width: usize,
    start_x: usize,
    start_y: usize,
}

pub struct Day18 {}

impl Day18 {
    fn calculate_grid_size(instructions: &Vec<(Direction, usize)>) -> GridState {
        // Track the extreme points on the grid
        let mut max_w = 0;
        let mut max_h = 0;
        let mut min_w = 0;
        let mut min_h = 0;
        let mut x = 0;
        let mut y = 0;
        for (direction, steps) in instructions.iter() {
            let steps = *steps as i32;
            match direction {
                North => {
                    y -= steps;
                    min_h = min_h.min(y)
                }
                East => {
                    x += steps;
                    max_w = max_w.max(x)
                }
                South => {
                    y += steps;
                    max_h = max_h.max(y)
                }
                West => {
                    x -= steps;
                    min_w = min_w.min(x)
                }
            }
        }

        // Create suitably sized grid
        GridState {
            height: (max_h - min_h) as usize,
            width: (max_w - min_w) as usize,
            start_x: -min_w as usize,
            start_y: -min_h as usize,
        }
    }

    fn calculate_outline(
        mut x: usize,
        mut y: usize,
        instructions: Vec<(Direction, usize)>,
        grid: &mut Vec<Vec<char>>,
    ) {
        // Create grid outline
        for (direction, steps) in instructions {
            match direction {
                North => {
                    for i in y - steps..=y {
                        grid[i][x] = '^';
                    }
                    y -= steps;
                }
                East => {
                    for j in x + 1..x + steps {
                        grid[y][j] = '-';
                    }
                    x += steps;
                }
                South => {
                    for i in y..=y + steps {
                        grid[i][x] = 'v';
                    }
                    y += steps;
                }
                West => {
                    for j in x - steps + 1..x {
                        grid[y][j] = '-';
                    }
                    x -= steps;
                }
            }
        }
    }

    fn calculate_area(grid: &Vec<Vec<char>>) -> u32 {
        // Caclulate number inside using Pick's algorithm
        let mut total = 0;
        let mut inside = false;
        for line in grid {
            for c in line {
                match c {
                    '^' => {
                        inside = true;
                        total += 1;
                    }
                    'v' => {
                        inside = false;
                        total += 1;
                    }
                    '-' => total += 1,
                    '.' if inside => total += 1,
                    _ => (),
                }
            }
        }
        total
    }
}

impl Day for Day18 {
    fn problem1() {
        let instructions: Vec<_> = SRC
            .lines()
            .map(|line| {
                let mut iter = line.split_ascii_whitespace();

                // Set direction be enum
                let direction = iter.next().expect("Should be direction before first space");
                let direction: Direction = direction.try_into().unwrap();

                // Cast number of steps to usize for list indexing later
                let steps = iter.next().expect("Should be steps after first space");
                let steps = steps.parse::<usize>().unwrap();

                (direction, steps)
            })
            .collect();

        // Calculate the grid based on the instructions
        let GridState {
            height,
            width,
            start_x,
            start_y,
        } = Self::calculate_grid_size(&instructions);
        let mut grid = vec![vec!['.'; width + 1]; height + 1];

        // Add the outline to the grid (in place)
        Self::calculate_outline(start_x, start_y, instructions, &mut grid);

        // Calculate the total area of the outline
        let total = Self::calculate_area(&grid);
        println!("{}", total);
    }

    fn problem2() {
        let instructions: Vec<_> = SRC
            .lines()
            .map(|line| {
                let mut hex = line
                    .split_ascii_whitespace()
                    .next_back()
                    .expect("Should contain hex code as final value");

                // Remove (# and )
                hex = &hex[2..8];

                // Parse the hex value to u32
                let (steps, direction) = hex.split_at(5);
                let steps = usize::from_str_radix(steps, 16).unwrap();

                // Get direction as Direction Type
                let directions = [West, South, East, North];
                let direction_idx = (direction.as_bytes()[0] - b'0') as usize;
                let direction = directions[direction_idx];

                // Return in same format as p1
                (direction, steps)
            })
            .collect();

        // Calculate the grid based on the instructions
        let GridState {
            height,
            width,
            start_x,
            start_y,
        } = Self::calculate_grid_size(&instructions);
        let mut grid = vec![vec!['.'; width + 1]; height + 1];

        // Add the outline to the grid (in place)
        Self::calculate_outline(start_x, start_y, instructions, &mut grid);

        // Calculate the total area of the outline
        let total = Self::calculate_area(&grid);
        println!("{}", total);
    }
}
