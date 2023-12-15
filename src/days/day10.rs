use std::collections::HashSet;

use self::Direction::*;
use advent_of_code::Day;

const SRC: &str = include_str!("../../input/day10.txt");

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn iterator() -> impl Iterator<Item = Direction> {
        [North, East, South, West].iter().copied()
    }
}

pub struct Day10 {}

impl Day10 {
    fn get_grid() -> Vec<Vec<char>> {
        SRC.lines().map(|line| line.chars().collect()).collect()
    }

    fn get_init_xy(grid: &Vec<Vec<char>>) -> (usize, usize) {
        let start = grid
            .iter()
            .flatten()
            .position(|&c| c == 'S')
            .expect("Should be a starting position");
        let width = grid[0].len();
        let x = start % width;
        let y = start / width;

        (x, y)
    }

    fn get_starting_direction(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Direction {
        Direction::iterator()
            .find(|&direction| {
                if let Some(pipe) = Self::get_next_pipe(&grid, x, y, direction) {
                    Self::valid_neighbour(pipe, direction)
                } else {
                    false
                }
            })
            .expect("Should be a valid neighbour to start")
    }

    fn get_pipe_north(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Option<char> {
        if y > 0 {
            Some(grid[y - 1][x])
        } else {
            None
        }
    }

    fn get_pipe_east(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Option<char> {
        if x < grid[y].len() - 1 {
            Some(grid[y][x + 1])
        } else {
            None
        }
    }

    fn get_pipe_south(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Option<char> {
        if y < grid.len() - 1 {
            Some(grid[y + 1][x])
        } else {
            None
        }
    }

    fn get_pipe_west(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Option<char> {
        if x > 0 {
            Some(grid[y][x - 1])
        } else {
            None
        }
    }

    fn get_next_direction(pipe: char, direction: Direction) -> Direction {
        match (pipe, direction) {
            ('|', North) | ('|', South) | ('-', East) | ('-', West) | ('S', _) | ('X', _) => {
                direction
            }
            ('L', West) | ('J', East) => North,
            ('7', North) | ('J', South) => West,
            ('7', East) | ('F', West) => South,
            ('L', South) | ('F', North) => East,
            _ => panic!(
                "Invalid combo of moving into pipe {} from direction {:?}",
                pipe, direction
            ),
        }
    }

    fn valid_neighbour(pipe: char, direction: Direction) -> bool {
        match (pipe, direction) {
            ('|', North) | ('|', South) | ('-', East) | ('-', West) | ('S', _) | ('X', _) => true,
            ('L', West) | ('J', East) => true,
            ('7', North) | ('J', South) => true,
            ('7', East) | ('F', West) => true,
            ('L', South) | ('F', North) => true,
            _ => false,
        }
    }

    fn get_next_pipe(
        grid: &Vec<Vec<char>>,
        x: usize,
        y: usize,
        direction: Direction,
    ) -> Option<char> {
        match direction {
            North => Self::get_pipe_north(grid, x, y),
            East => Self::get_pipe_east(grid, x, y),
            South => Self::get_pipe_south(grid, x, y),
            West => Self::get_pipe_west(grid, x, y),
        }
    }

    fn get_next_xy(x: usize, y: usize, direction: Direction) -> (usize, usize) {
        match direction {
            North => (x, y - 1),
            East => (x + 1, y),
            South => (x, y + 1),
            West => (x - 1, y),
        }
    }
}

impl Day for Day10 {
    fn problem1() {
        // Parse the grid from the input
        let grid = Self::get_grid();
        let (init_x, init_y) = Self::get_init_xy(&grid);
        let mut direction = Self::get_starting_direction(&grid, init_x, init_y);

        // Init state
        let mut dist = 0;
        let mut x = init_x;
        let mut y = init_y;
        let mut pipe;

        // Iterate around the pipe
        loop {
            dist += 1;
            pipe = Self::get_next_pipe(&grid, x, y, direction).expect("Should be a next neighbour");
            (x, y) = Self::get_next_xy(x, y, direction);
            direction = Self::get_next_direction(pipe, direction);

            if x == init_x && y == init_y {
                break;
            }
        }

        // Half the distance
        println!("{}", dist / 2);
    }

    fn problem2() {
        // Parse the grid from the input
        let mut grid = Self::get_grid();
        let (init_x, init_y) = Self::get_init_xy(&grid);
        let init_direction = Self::get_starting_direction(&grid, init_x, init_y);

        // Init state
        let mut direction = init_direction;
        let mut x = init_x;
        let mut y = init_y;
        let mut pipe;

        // Iterate around the pipe
        let mut grid_coords: HashSet<(usize, usize)> = HashSet::new();
        loop {
            pipe = Self::get_next_pipe(&grid, x, y, direction).expect("Should be a next neighbour");
            (x, y) = Self::get_next_xy(x, y, direction);
            direction = Self::get_next_direction(pipe, direction);
            grid_coords.insert((x, y));

            if x == init_x && y == init_y {
                break;
            }
        }

        // Map S to the correct pipe
        grid[init_y][init_x] = match (init_direction, direction) {
            (North, North) | (South, South) => '|',
            (East, East) | (West, West) => '-',
            (East, North) | (South, West) => 'F',
            (West, North) | (South, East) => '7',
            (East, South) | (North, West) => 'L',
            (West, South) | (North, East) => 'J',
            _ => unreachable!("Shouldn't be able to come in from same pipe?"),
        };

        // Use Pick's theorem to find inside and outside points
        let mut n_inside = 0;
        for (y, row) in grid.into_iter().enumerate() {
            let mut down = false;
            let mut up = false;
            for (x, pipe) in row.into_iter().enumerate() {
                if grid_coords.contains(&(x, y)) {
                    match pipe {
                        '|' => {
                            down = !down;
                            up = !up;
                        }
                        'F' | '7' => down = !down,
                        'L' | 'J' => up = !up,
                        _ => (),
                    }
                } else if up || down {
                    n_inside += 1;
                }
            }
        }

        println!("{}", n_inside);
    }
}
