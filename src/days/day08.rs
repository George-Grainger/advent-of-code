use core::panic;
use std::collections::HashMap;

use advent_of_code::Day;

const SRC: &str = include_str!("../../input/day08.txt");

#[derive(Debug)]
struct MapEntry<'a> {
    left: &'a str,
    right: &'a str,
}

impl<'a> MapEntry<'a> {
    fn new(left: &'a str, right: &'a str) -> Self {
        Self {
            left: left.trim().trim_start_matches('('),
            right: right.trim().trim_end_matches(')'),
        }
    }
}

pub struct Day08 {}

impl Day08 {
    fn parse_input() -> (&'static str, HashMap<&'static str, MapEntry<'static>>) {
        let mut lines = SRC.lines();
        let directions = lines
            .next()
            .expect("Should contains directions as first line");

        let map = lines
            .skip(1)
            .map(|line| {
                let (k, v) = line
                    .split_once('=')
                    .expect("Map should be in form <key> = (<left>, <right>)");

                let (l, r) = v
                    .split_once(',')
                    .expect("Value should be in form (<left>, <right>)");
                let v = MapEntry::new(l, r);

                (k.trim(), v)
            })
            .collect();

        (directions, map)
    }
}

impl Day for Day08 {
    fn problem1() {
        let (directions, map) = Self::parse_input();

        let mut i = 0;
        let mut location = "AAA";
        let mut directions = directions.chars().cycle();
        while location != "ZZZ" {
            let direction = directions.next().expect("Should cycle through directions");
            let possibilities = map.get(location).expect("Location should be in map");

            location = match direction {
                'L' => possibilities.left,
                'R' => possibilities.right,
                _ => panic!("Direction should either be left or right"),
            };
            i += 1;
        }

        println!("{}", i);
    }

    fn problem2() {
        let (directions, map) = Self::parse_input();
        let locations: Vec<&str> = map
            .keys()
            .filter(|key| key.ends_with("A"))
            .map(|location| *location)
            .collect();

        // Track seen and index into array
        let mut pres = Vec::with_capacity(locations.len());
        let mut posts = Vec::with_capacity(locations.len());
        let mut loop_sizes = Vec::with_capacity(locations.len());

        for start in locations.into_iter() {
            let mut location = start;
            let mut seen: HashMap<(&str, usize), usize> = HashMap::new();
            let mut seen_at: Vec<usize> = Vec::new();
            for (total, (pos, direction)) in directions.chars().enumerate().cycle().enumerate() {
                let possibilities = map.get(location).expect("Location should be in map");

                location = match direction {
                    'L' => possibilities.left,
                    'R' => possibilities.right,
                    _ => panic!("Direction should either be left or right"),
                };

                if location.ends_with("Z") {
                    if let Some(&split) = seen.get(&(location, pos)) {
                        pres.push(seen_at[..split].to_vec());
                        posts.push(seen_at[split..].to_vec());
                        loop_sizes.push((total - seen_at[split]) + 1);
                        break;
                    }
                    seen.insert((location, pos), seen_at.len());
                    seen_at.push(total + 1);
                }
            }
        }

        let pres = pres
            .into_iter()
            .reduce(|acc, curr| {
                let joint = acc.iter().filter(|val| curr.contains(val)).map(|val| *val);
                joint.collect()
            })
            .unwrap_or_default();

        if !pres.is_empty() {
            let min = pres.iter().min().expect("Should be a minimum value");
            println!("{:?}", min);
            return;
        }

        let posts = posts
            .into_iter()
            .zip(loop_sizes)
            .reduce(|(acc, _), (curr, len)| {
                let joint = acc
                    .into_iter()
                    .flat_map(|val| {
                        let x = curr.iter().filter_map(move |offset| {
                            (0..val).map(|n| len * n + offset).find(|n| n % val == 0)
                        });
                        x
                    })
                    .collect();
                (joint, usize::default())
            });
        let posts = posts.unwrap_or_default().0;
        let min = posts.iter().min().expect("Should be a minimum value");
        println!("{:?}", min);
        // println!("{:?}", (0..2).map(|n| 6 * n + 6).find(|n| n % 2 == 0));
    }
}
