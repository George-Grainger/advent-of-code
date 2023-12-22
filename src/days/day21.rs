use advent_of_code::Day;
use std::collections::HashSet;

const SRC: &str = include_str!("../../input/day21.txt");

type Coord = (usize, usize);

pub struct Day21 {}

impl Day21 {
    fn parse_input() -> (Vec<Vec<char>>, Coord) {
        let mut start = (0, 0);
        let map = SRC
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, char)| {
                        if char == 'S' {
                            start = (y, x);
                            '.'
                        } else {
                            char
                        }
                    })
                    .collect()
            })
            .collect();

        (map, start)
    }

    fn fill(map: &Vec<Vec<char>>, start: Coord, steps: usize) -> usize {
        let mut positions: HashSet<Coord> = HashSet::new();
        positions.insert(start);

        for _ in 0..steps {
            let mut new_positions: HashSet<Coord> = HashSet::new();
            for position in positions {
                let (y, x) = position;
                if y > 0 && map[y - 1][x] == '.' {
                    new_positions.insert((y - 1, x));
                }
                if y < map.len() - 1 && map[y + 1][x] == '.' {
                    new_positions.insert((y + 1, x));
                }
                if x > 0 && map[y][x - 1] == '.' {
                    new_positions.insert((y, x - 1));
                }
                if x < map[y].len() - 1 && map[y][x + 1] == '.' {
                    new_positions.insert((y, x + 1));
                }
            }
            positions = new_positions;
        }
        positions.len()
    }
}

impl Day for Day21 {
    fn problem1() {
        const STEPS: usize = 64;
        let (map, start) = Self::parse_input();
        println!("{:?}", Self::fill(&map, start, STEPS));
    }

    fn problem2() {
        const STEPS: usize = 26501365;

        let (map, start) = Self::parse_input();
        let len = map.len();

        // Get number of odd and even squares
        let grid_width = STEPS / len - 1;
        let mut odd_squares = ((grid_width + 1) / 2 * 2).pow(2);
        let mut even_squares = (grid_width / 2 * 2 + 1).pow(2);

        // Determine points that can be reached
        odd_squares *= Self::fill(&map, start, len * 2);
        even_squares *= Self::fill(&map, start, len * 2 + 1);

        // Dertime corners - top, bottom, right, left
        let mut corners = Self::fill(&map, (len - 1, start.1), len - 1);
        corners += Self::fill(&map, (0, start.1), len - 1);
        corners += Self::fill(&map, (start.0, len - 1), len - 1);
        corners += Self::fill(&map, (start.0, 0), len - 1);

        // Calculate other paritally filled segments segments
        let mut sm_segments = Self::fill(&map, (len - 1, 0), len / 2 - 1);
        sm_segments += Self::fill(&map, (len - 1, len - 1), len / 2 - 1);
        sm_segments += Self::fill(&map, (0, 0), len / 2 - 1);
        sm_segments += Self::fill(&map, (0, len - 1), len / 2 - 1);

        // Compute the large segments
        let mut lg_segments = Self::fill(&map, (len - 1, 0), 3 * len / 2 - 1);
        lg_segments += Self::fill(&map, (len - 1, len - 1), 3 * len / 2 - 1);
        lg_segments += Self::fill(&map, (0, 0), 3 * len / 2 - 1);
        lg_segments += Self::fill(&map, (0, len - 1), 3 * len / 2 - 1);

        // See the segments multiple times
        sm_segments *= grid_width + 1;
        lg_segments *= grid_width;
        let segments = sm_segments + lg_segments;

        let total = odd_squares + even_squares + corners + segments;
        println!("{}", total)
    }
}
