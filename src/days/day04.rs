use std::collections::HashSet;

use advent_of_code::Day;

const SRC: &str = include_str!("../../input/day04.txt");

pub struct Day04 {}

impl Day04 {
    fn scores() -> impl Iterator<Item = u32> {
        SRC.lines()
            .map(|line| {
                let start = line.find(':').expect("Card should be 'Card <num>: ...'") + 1;
                line[start..]
                    .split_once('|')
                    .expect("Every card should have a veritcal bar")
            })
            .map(|(winning, got)| {
                let got: HashSet<u32> = got
                    .split_whitespace()
                    .filter_map(|c| c.parse::<u32>().ok())
                    .collect();
                (winning, got)
            })
            .map(|(winning, got)| {
                winning
                    .split_whitespace()
                    .filter_map(|c| c.parse::<u32>().ok())
                    .fold(0, |won, num| if got.contains(&num) { won + 1 } else { won })
            })
    }
}

impl Day for Day04 {
    fn problem1() {
        let total: u32 = Self::scores()
            .filter(|&c| c != 0)
            .map(|score| u32::pow(2, score - 1))
            .sum();
        println!("{:?}", total);
    }

    fn problem2() {
        let scores: Vec<u32> = Self::scores().collect();
        let mut mults = vec![1; scores.len()];
        let mut total = 0;
        for (i, &score) in scores.iter().enumerate() {
            for j in i..i + score as usize {
                mults[j + 1] += mults[i];
            }
            total += mults[i];
        }
        println!("{}", total);
    }
}
