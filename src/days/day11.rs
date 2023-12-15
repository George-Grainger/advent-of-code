use std::collections::HashSet;

use advent_of_code::Day;

const SRC: &str = include_str!("../../input/day11.txt");

pub struct Day11 {}

impl Day11 {
    fn abs_diff(a: usize, b: usize) -> usize {
        if a > b {
            a - b
        } else {
            b - a
        }
    }

    fn coord_diff(p1: &(usize, usize), p2: &(usize, usize)) -> usize {
        Self::abs_diff(p1.0, p2.0) + Self::abs_diff(p1.1, p2.1)
    }

    fn get_total_distance(dilation: usize) -> usize {
        let map: Vec<Vec<_>> = SRC.lines().map(|line| line.chars().collect()).collect();

        let empty_rows: HashSet<_> = map
            .iter()
            .enumerate()
            .filter(|(_, row)| !row.contains(&'#'))
            .map(|(i, _)| i)
            .collect();

        let empty_cols: HashSet<_> = (0..map.len())
            .filter(|&i| map.iter().all(|row| row[i] == '.'))
            .collect();

        let mut galaxy_coords: HashSet<(usize, usize)> = HashSet::new();
        let mut y_dilation = 0;
        for (y, row) in map.into_iter().enumerate() {
            let mut x_dilation = 0;
            for (x, val) in row.into_iter().enumerate() {
                if val == '#' {
                    let coord = (x + x_dilation, y + y_dilation);
                    galaxy_coords.insert(coord);
                } else if empty_cols.contains(&x) {
                    // subtract one to account for x increment
                    x_dilation += dilation - 1;
                }
            }
            if empty_rows.contains(&y) {
                // subtract one to account for y increment
                y_dilation += dilation - 1;
            }
        }

        let mut distance = 0;
        for (i, galaxy1) in galaxy_coords.iter().enumerate() {
            for galaxy2 in galaxy_coords.iter().skip(i) {
                distance += Self::coord_diff(galaxy1, galaxy2);
            }
        }

        distance
    }
}

impl Day for Day11 {
    fn problem1() {
        println!("{}", Self::get_total_distance(2));
    }

    fn problem2() {
        println!("{}", Self::get_total_distance(1_000_000));
    }
}
