use std::collections::HashSet;

use advent_of_code::Day;

const SRC: &str = include_str!("../../input/day3.txt");

pub struct Three {}

impl Three {}

impl Day for Three {
    fn problem1() {
        const SYMBOLS: [char; 30] = [
            '-', '!', '$', '%', '^', '&', '*', '(', ')', '_', '+', '|', '~', '=', '`', '{', '}',
            '[', ']', ':', '"', ';', '\'', '<', '>', '?', ',', '/', '#', '@',
        ];

        let lines: Vec<&str> = SRC.lines().collect();
        let mut total = 0;
        for (i, line) in lines.iter().enumerate() {
            let mut num = 0;
            let mut valid = false;
            let row_start = (i as isize - 1).max(0) as usize;
            let row_end = (i + 2).min(lines.len());

            for (j, c) in line.bytes().enumerate() {
                if c.is_ascii_digit() {
                    num *= 10;
                    num += (c - b'0') as u32;

                    if valid {
                        continue;
                    }

                    let start = (j as isize - 1).max(0) as usize;
                    let end = (j + 2).min(line.len());
                    for k in &lines[row_start..row_end] {
                        valid |= SYMBOLS.iter().any(|&s| k[start..end].contains(s));
                    }
                } else {
                    if valid {
                        total += num;
                    }
                    num = 0;
                    valid = false;
                }
            }
            if valid {
                total += num;
            }
        }
        println!("{}", total);
    }

    fn problem2() {
        let lines: Vec<&str> = SRC.lines().collect();
        let mut total = 0;
        for (i, line) in lines.iter().enumerate() {
            let row_start = (i as isize - 1).max(0) as usize;
            let row_end = (i + 2).min(lines.len());
            for (j, c) in line.chars().enumerate() {
                if c != '*' {
                    continue;
                }

                // Find digits around star
                let start = (j as isize - 1).max(0) as usize;
                let end = (j + 2).min(line.len());

                let mut num_locations = HashSet::new();
                for row in lines[row_start..row_end].into_iter() {
                    let region = &row[start..end];

                    let s = region.find(|v: char| v.is_ascii_digit());

                    if s.is_none() {
                        continue;
                    }
                    let s = s.unwrap();
                    num_locations.insert((row, s + start));

                    let e = region.rfind(|v: char| v.is_ascii_digit());
                    if e.is_none() {
                        continue;
                    }
                    let e = e.unwrap();
                    let has_gap = region[s..e].contains(|c: char| !c.is_ascii_digit());
                    if has_gap {
                        num_locations.insert((row, e + start));
                    }
                }

                // Only run when there are two numbers
                if num_locations.len() != 2 {
                    continue;
                }

                // Expand the numbers
                total += num_locations
                    .iter()
                    .map(|&(row, mut s)| {
                        let mut e = s;
                        let chars: Vec<char> = row.chars().collect();
                        while s > 0 && chars[s - 1].is_ascii_digit() {
                            s -= 1;
                        }
                        while e < chars.len() && chars[e].is_ascii_digit() {
                            e += 1;
                        }
                        row[s..e].parse::<u32>().expect("Should be integer")
                    })
                    .product::<u32>();
            }
        }

        println!("{}", total);
    }
}
