use self::Direction::*;
use advent_of_code::Day;
use std::{
    fmt::Display,
    fmt::{Error, Formatter},
};

const SRC: &str = include_str!("../../input/day18.txt");

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

pub struct Day18 {}

impl Day18 {
    fn calculate_area(instructions: impl Iterator<Item = (Direction, i64)>) -> i64 {
        // Track perimeter - extra for first and last step
        let mut perim = 2;
        let mut total = 0;

        // Track total area using shoelace formula
        let mut x = 0;
        let mut y = 0;
        for (direction, steps) in instructions {
            let (next_x, next_y) = match direction {
                North => (x, y - steps),
                East => (x + steps, y),
                South => (x, y + steps),
                West => (x - steps, y),
            };

            // Handle increments
            total += x * next_y;
            total -= y * next_x;
            perim += steps;

            // Update variables
            x = next_x;
            y = next_y;
        }

        (perim + total.abs()) / 2
    }
}

impl Day for Day18 {
    fn problem1() {
        let instructions = SRC.lines().map(|line| {
            let mut iter = line.split_ascii_whitespace();

            // Set direction be enum
            let direction = iter.next().expect("Should be direction before first space");
            let direction: Direction = direction.try_into().unwrap();

            // Cast number of steps to usize for list indexing later
            let steps = iter.next().expect("Should be steps after first space");
            let steps = steps.parse::<i64>().unwrap();

            (direction, steps)
        });

        println!("{:?}", Self::calculate_area(instructions));
    }

    fn problem2() {
        let instructions = SRC.lines().map(|line| {
            let mut hex = line
                .split_ascii_whitespace()
                .next_back()
                .expect("Should contain hex code as final value");

            // Remove (# and )
            hex = &hex[2..8];

            // Parse the hex value to u32
            let (steps, direction) = hex.split_at(5);
            let steps = i64::from_str_radix(steps, 16).unwrap();

            // Get direction as Direction Type
            let directions = [West, South, East, North];
            let direction_idx = (direction.as_bytes()[0] - b'0') as usize;
            let direction = directions[direction_idx];

            // Return in same format as p1
            (direction, steps)
        });

        println!("{:?}", Self::calculate_area(instructions));
    }
}
