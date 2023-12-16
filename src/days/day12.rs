use std::io::repeat;

use advent_of_code::Day;

const SRC: &str = include_str!("../../input/day12.txt");

pub struct Day12 {}

impl Day12 {
    fn parse_input() -> Vec<(&'static [u8], Vec<usize>)> {
        SRC.lines()
            .map(|line| {
                let (prefix, suffix) = line.split_once(' ').unwrap();

                let dist = prefix.as_bytes();
                let sizes = suffix
                    .split(',')
                    .map(|val| val.parse().expect("Should be parsable as usize"))
                    .collect();

                (dist, sizes)
            })
            .collect()
    }

    fn solve(repeats: usize) -> u64 {
        let input = Self::parse_input();
        let mut output = 0;

        for (base_pattern, base_springs) in input {
            let mut pattern = base_pattern.to_vec();
            let mut springs = base_springs.clone();
            for _ in 1..repeats {
                pattern.push(b'?');
                pattern.extend_from_slice(base_pattern);
                springs.extend_from_slice(&base_springs);
            }
            pattern.push(b'.');

            let mut broken = vec![0];
            broken.extend(pattern.iter().scan(0, |state, &c| {
                if c == b'?' || c == b'#' {
                    *state += 1;
                }
                Some(*state)
            }));

            let num_springs: usize = springs.iter().sum();
            let wiggle = broken.len() - num_springs - springs.len();
            let mut table = vec![vec![0; pattern.len()]; springs.len()];

            let size = springs[0];
            let mut sum = 0;
            let mut valid = true;
            for i in 0..wiggle {
                if pattern[i + size] == b'#' {
                    sum = 0;
                } else if valid && broken[i + size] - broken[i] == size {
                    sum += 1;
                }
                table[0][i + size] = sum;

                // The first pattern can't have any '#' characters anywhere to its left
                // otherwise it wouldn't be the first pattern.
                valid &= pattern[i] != b'#';
            }

            let mut start = size + 1;
            for (row, &size) in springs.iter().enumerate().skip(1) {
                // Reset the running sum.
                sum = 0;

                for i in start..start + wiggle {
                    // As a minor optimisation only check the pattern if the previous row
                    // will contribute a non-zero value.
                    if pattern[i + size] == b'#' {
                        sum = 0;
                    } else if table[row - 1][i - 1] > 0
                        && pattern[i - 1] != b'#'
                        && broken[i + size] - broken[i] == size
                    {
                        sum += table[row - 1][i - 1];
                    }

                    table[row][i + size] = sum;
                }

                start += size + 1;
            }

            output += sum;
        }

        output
    }
}

impl Day for Day12 {
    fn problem1() {
        println!("{}", Self::solve(1));
    }

    fn problem2() {
        println!("{}", Self::solve(5));
    }
}
