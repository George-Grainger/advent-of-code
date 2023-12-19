use std::collections::HashSet;

use self::Direction::*;
use advent_of_code::Day;

const SRC: &str = include_str!("../../input/day16.txt");

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Day16 {}

impl Day16 {
    /// Generates a function for incrementing the coordinates based on a grid of fixed size
    fn create_inc_coords(
        max_x: usize,
        max_y: usize,
    ) -> impl Fn(usize, usize, Direction) -> Option<(usize, usize)> {
        move |x, y, direction| match direction {
            Direction::North if y > 0 => Some((x, y - 1)),
            Direction::West if x > 0 => Some((x - 1, y)),
            Direction::South if y < max_y => Some((x, y + 1)),
            Direction::East if x < max_x => Some((x + 1, y)),
            _ => None,
        }
    }

    fn get_next_directions(symbol: u8, direction: Direction) -> Vec<Direction> {
        match (symbol, direction) {
            (b'-', North) | (b'-', South) => vec![East, West],
            (b'|', East) | (b'|', West) => vec![North, South],
            (b'/', North) | (b'\\', South) => vec![East],
            (b'/', East) | (b'\\', West) => vec![North],
            (b'/', South) | (b'\\', North) => vec![West],
            (b'/', West) | (b'\\', East) => vec![South],
            _ => vec![direction],
        }
    }

    fn count_energised(
        grid: &Vec<&[u8]>,
        start: (usize, usize, Direction),
        inc_coords: impl Fn(usize, usize, Direction) -> Option<(usize, usize)>,
    ) -> usize {
        // Create a stack of beam states (x, y, direction)
        let mut seen = HashSet::new();
        let mut beams = vec![start];

        // Loop while there's still a previously unseen beam moving through the grid
        while !beams.is_empty() {
            let mut next_beams = Vec::new();
            for &(x, y, direction) in beams.iter() {
                // Keep track of previous values to prevent infinite loop
                if seen.insert((x, y, direction)) {
                    for new_direction in Self::get_next_directions(grid[y][x], direction) {
                        // Use option to finish when beam exits grid
                        if let Some((new_x, new_y)) = inc_coords(x, y, new_direction) {
                            next_beams.push((new_x, new_y, new_direction));
                        }
                    }
                }
            }
            beams = next_beams;
        }

        // Count the number of unique squares moved across
        let seen: HashSet<(usize, usize)> = seen.into_iter().map(|(x, y, _)| (x, y)).collect();
        seen.len()
    }
}

impl Day for Day16 {
    fn problem1() {
        let grid: Vec<&[u8]> = SRC.lines().map(|line| line.as_bytes()).collect();

        // Get function for incrementing coordinates
        let max_x = grid.get(0).expect("Should be at least one row").len() - 1;
        let max_y = grid.len() - 1;
        let inc_coords = Self::create_inc_coords(max_x, max_y);

        // Get the number of energised tiles
        let energised = Self::count_energised(&grid, (0, 0, East), &inc_coords);
        println!("{}", energised);
    }

    fn problem2() {
        let grid: Vec<&[u8]> = SRC.lines().map(|line| line.as_bytes()).collect();

        // Get function for incrementing coordinates
        let max_x = grid.get(0).expect("Should be at least one row").len() - 1;
        let max_y = grid.len() - 1;
        let inc_coords = Self::create_inc_coords(max_x, max_y);

        let mut max_energised = 0;

        // Iterate vertical directions
        for x in 0..=max_x {
            // Iterate over beams coming from top of grid
            let mut energised = Self::count_energised(&grid, (x, 0, South), &inc_coords);
            max_energised = max_energised.max(energised);

            // Iterate over beams coming from bottom of grid
            energised = Self::count_energised(&grid, (x, max_y, North), &inc_coords);
            max_energised = max_energised.max(energised);
        }

        // Iterate horizontal directions
        for y in 0..=max_y {
            // Iterate over beams coming from left of grid
            let mut energised = Self::count_energised(&grid, (0, y, East), &inc_coords);
            max_energised = max_energised.max(energised);

            // Iterate over beams coming from right of grid
            energised = Self::count_energised(&grid, (max_x, y, West), &inc_coords);
            max_energised = max_energised.max(energised);
        }

        println!("{}", max_energised);
    }
}
