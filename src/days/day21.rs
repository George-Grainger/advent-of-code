use advent_of_code::Day;
use std::collections::HashSet;

const SRC: &str = include_str!("../../input/day21.txt");

type Coord = (usize, usize);

pub struct Day21 {}

impl Day21 {
    fn parse_input() -> (Vec<Vec<char>>, Coord, Coord) {
        let map: Vec<Vec<char>> = SRC.lines().map(|s| s.chars().collect()).collect();
        let height = map.len();
        let width = map.get(0).expect("Should be at least one row").len();
        let lim = (width - 1, height - 1);

        // Find the 'S' position
        let start = map
            .iter()
            .flatten()
            .position(|&c| c == 'S')
            .expect("Should be 'S' in map");
        let start = (start / height, start % height);

        (map, start, lim)
    }

    fn get_neighbours((x, y): Coord, (max_x, max_y): Coord) -> Vec<Coord> {
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
}

impl Day for Day21 {
    fn problem1() {
        const MAX_STEPS: usize = 64;
        let (map, start, lim) = Self::parse_input();

        // Create a set for holding active positions
        let mut active_positions: HashSet<Coord> = HashSet::new();
        active_positions.insert(start);

        // Iterate over each step to find active positions
        for _ in 0..MAX_STEPS {
            let mut new_active_positions = HashSet::new();

            for position in active_positions {
                let next_poisitions = Self::get_neighbours(position, lim)
                    .into_iter()
                    .filter(|&(x, y)| map[y][x] != '#');

                for np in next_poisitions {
                    new_active_positions.insert(np);
                }
            }

            active_positions = new_active_positions;
        }

        println!("{:?}", active_positions.len());
    }

    fn problem2() {}
}
