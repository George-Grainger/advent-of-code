use advent_of_code::Day;
use std::collections::{BTreeSet, HashSet};

const SRC: &str = include_str!("../../input/day21.txt");

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    x: usize,
    y: usize,
    steps: usize,
}

impl State {
    fn new(x: usize, y: usize, steps: usize) -> Self {
        Self { x, y, steps }
    }
}

pub struct Day21 {}

impl Day21 {
    fn get_neighbours(x: usize, y: usize, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
        let mut neighbours: Vec<_> = Vec::with_capacity(4);
        if y > 0 {
            neighbours.push((x, y - 1))
        }
        if x > 0 {
            neighbours.push((x - 1, y))
        }
        if y < max_x {
            neighbours.push((x, y + 1))
        }
        if x < max_y {
            neighbours.push((x + 1, y))
        }
        neighbours
    }

    fn print(map: &Vec<Vec<char>>) {
        for line in map.iter() {
            for &c in line.iter() {
                print!("{:} ", c as char);
            }
            println!();
        }
    }
}

impl Day for Day21 {
    fn problem1() {
        let map: Vec<Vec<char>> = SRC.lines().map(|s| s.chars().collect()).collect();
        let height = map.len();
        let width = map.get(0).expect("Should be at least one row").len();
        let max_x = width - 1;
        let max_y = height - 1;

        // Find the 'S' position
        let start = map
            .iter()
            .flatten()
            .position(|&c| c == 'S')
            .expect("Should be 'S' in map");
        let start = State::new(start % height, start / height, 0);

        // Create a set for holding active positions
        let mut active_positions: BTreeSet<State> = BTreeSet::new();
        active_positions.insert(start);

        // Create list of final positions
        const MAX_STEPS: usize = 12;
        let mut final_positions = HashSet::new();

        while let Some(pos) = active_positions.pop_first() {
            if pos.steps == MAX_STEPS {
                final_positions.insert(pos);
            } else {
                // Get neighbours that aren't rocks and insert into the active_positions
                Self::get_neighbours(pos.x, pos.y, max_x, max_y)
                    .into_iter()
                    .filter(|&(x, y)| map[y][x] != '#')
                    .map(|(x, y)| State::new(x, y, pos.steps + 1))
                    .for_each(|neighbour| {
                        active_positions.insert(neighbour);
                    });
            }
        }

        println!("{:?}", final_positions.len());
    }

    fn problem2() {}
}
