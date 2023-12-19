use std::collections::HashMap;

use advent_of_code::Day;

const SRC: &str = include_str!("../../input/day15.txt");

pub struct Day15 {}

impl Day15 {
    fn hash(s: &str) -> u8 {
        s.bytes().fold(0u8, |acc, c| {
            // Use overflow to prevent requiring mod
            acc.wrapping_add(c).wrapping_mul(17)
        })
    }
}

impl Day for Day15 {
    fn problem1() {
        let output: u32 = SRC.split(',').map(|s| Self::hash(s) as u32).sum();
        println!("{:?}", output);
    }

    fn problem2() {
        const N_BOXES: usize = 256;
        let strings = SRC.split(',');

        let mut boxes: Vec<Vec<&str>> = vec![Vec::new(); N_BOXES];
        let mut box_map: HashMap<&str, &str> = HashMap::new();

        for s in strings {
            let (k, v) = s
                .split_once(|c| c == '=' || c == '-')
                .expect("Every line will contain either '-' or '='");
            let hash = Self::hash(k) as usize;

            // If it's empty we know it's '-'
            if v.is_empty() {
                if box_map.remove(k).is_some() {
                    let idx = boxes[hash]
                        .iter()
                        .position(|&s| s == k)
                        .expect("List will contain value when in map");
                    boxes[hash].remove(idx);
                }
            } else {
                if box_map.insert(k, v).is_none() {
                    boxes[hash].push(k);
                }
            }
        }

        let mut total = 0;
        for (box_mul, l_box) in (1..).zip(boxes) {
            for (pos_mul, k) in (1..).zip(l_box) {
                let focal_strength = box_map
                    .get(k)
                    .expect("Map keys should align with boxes array");
                let focal_strength = focal_strength
                    .parse::<u32>()
                    .expect("Focal strength should be number between 1 and 9");
                total += focal_strength * pos_mul * box_mul;
            }
        }

        println!("{}", total);
    }
}
