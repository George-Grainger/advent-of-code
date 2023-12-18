use std::collections::HashMap;

use advent_of_code::Day;

const SRC: &str = include_str!("../../input/day14.txt");

pub struct Day14 {}

impl Day14 {
    fn parse() -> Vec<Vec<u8>> {
        SRC.lines().map(|line| line.as_bytes().to_vec()).collect()
    }

    fn calculate_load(grid: &Vec<Vec<u8>>) -> usize {
        let rows = grid.len();

        // Count number of rocks on each row
        grid.iter()
            .rev()
            // Multiply load based on height of row
            .zip(1..=rows)
            .map(|(row, mult)| mult * row.iter().filter(|&&c| c == b'O').count())
            .sum()
    }

    fn slide_north(grid: &mut Vec<Vec<u8>>) {
        let rows = grid.len();
        let cols = grid[0].len();

        for col in 0..cols {
            // Keep track of how many rocks have been seen
            let mut inc = 0;
            for row in 0..rows {
                match grid[row][col] {
                    b'O' => {
                        grid[row][col] = b'.';
                        grid[inc][col] = b'O';
                        inc += 1;
                    }
                    // Dont need to update grid value as already correct
                    b'#' => inc = row + 1,
                    b'.' => (),
                    _ => panic!("Should be either '#', 'O' or '.'"),
                }
            }
        }
    }

    fn slide_east(grid: &mut Vec<Vec<u8>>) {
        let rows = grid.len();
        let cols = grid[0].len();

        for row in 0..rows {
            // Keep track of how many rocks have been seen
            let mut inc = cols - 1;
            for col in (0..cols).rev() {
                match grid[row][col] {
                    b'O' => {
                        grid[row][col] = b'.';
                        grid[row][inc] = b'O';
                        if inc > 0 {
                            inc -= 1;
                        }
                    }
                    // Dont need to update grid value as already correct
                    b'#' if col > 0 => inc = col - 1,
                    b'#' | b'.' => (),
                    _ => panic!("Should be either '#', 'O' or '.'"),
                }
            }
        }
    }

    fn slide_south(grid: &mut Vec<Vec<u8>>) {
        let rows = grid.len();
        let cols = grid[0].len();

        for col in 0..cols {
            // Keep track of how many rocks have been seen
            let mut inc = rows - 1;
            for row in (0..rows).rev() {
                match grid[row][col] {
                    b'O' => {
                        grid[row][col] = b'.';
                        grid[inc][col] = b'O';
                        if inc > 0 {
                            inc -= 1;
                        }
                    }
                    // Dont need to update grid value as already correct
                    b'#' if row > 0 => inc = row - 1,
                    b'#' | b'.' => (),
                    _ => panic!("Should be either '#', 'O' or '.'"),
                }
            }
        }
    }

    fn slide_west(grid: &mut Vec<Vec<u8>>) {
        let rows = grid.len();
        let cols = grid[0].len();

        for row in 0..rows {
            // Keep track of how many rocks have been seen
            let mut inc = 0;
            for col in 0..cols {
                match grid[row][col] {
                    b'O' => {
                        grid[row][col] = b'.';
                        grid[row][inc] = b'O';
                        inc += 1;
                    }
                    // Dont need to update grid value as already correct
                    b'#' => inc = col + 1,
                    b'.' => (),
                    _ => panic!("Should be either '#', 'O' or '.'"),
                }
            }
        }
    }
}

impl Day for Day14 {
    fn problem1() {
        // Get grid and make mutable to work in place
        let mut grid = Self::parse();
        Self::slide_north(&mut grid);
        println!("{}", Self::calculate_load(&grid));
    }

    fn problem2() {
        let mut grid = Self::parse();
        // Store previous state
        let mut seen: HashMap<Vec<Vec<u8>>, usize> = HashMap::new();

        let mut cycles = 1000000000;
        while cycles > 0 {
            // Check if the output is looping and then find the number of cycles after n * the loop size
            if let Some(old) = seen.insert(grid.clone().into_iter().collect(), cycles) {
                cycles = cycles % (old - cycles);
                seen.clear();
            }
            Self::slide_north(&mut grid);
            Self::slide_west(&mut grid);
            Self::slide_south(&mut grid);
            Self::slide_east(&mut grid);
            cycles -= 1;
        }

        println!("{}", Self::calculate_load(&grid));
    }
}
