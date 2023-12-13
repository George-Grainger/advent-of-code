use std::{cmp::Ordering, collections::HashMap};

use advent_of_code::Day;

const SRC: &str = include_str!("../../input/day07.txt");

#[derive(Debug, PartialEq, PartialOrd)]
enum Rank {
    FiveOfKind = 6,
    FourOfKind = 5,
    FullHouse = 4,
    ThreeOfKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl Rank {
    fn get(hand: [u8; 5], is_p2: bool) -> Self {
        let mut occurances = hand.iter().fold(HashMap::new(), |mut counts, &item| {
            *counts.entry(item).or_insert(0u32) += 1;
            counts
        });

        let wildcards = if is_p2 {
            occurances.remove(&1).unwrap_or_default()
        } else {
            0
        };

        let mut occurances: Vec<_> = occurances.into_values().collect();

        // Reorder the array
        occurances.sort();
        let first = occurances.pop().unwrap_or_default();
        let second = occurances.pop().unwrap_or_default();

        match (first + wildcards, second) {
            (5, _) => Rank::FiveOfKind,
            (4, _) => Rank::FourOfKind,
            (3, 2) => Rank::FullHouse,
            (3, _) => Rank::ThreeOfKind,
            (2, 2) => Rank::TwoPair,
            (2, _) => Rank::OnePair,
            _ => Rank::HighCard,
        }
    }
}

pub struct Day07 {}

impl Day07 {
    fn map_hand(hand: &str, is_p2: bool) -> [u8; 5] {
        hand.chars()
            .map(|c| match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => {
                    if !is_p2 {
                        11
                    } else {
                        1
                    }
                }
                'T' => 10,
                '2'..='9' => (c as u8) - b'0',
                _ => panic!("Hand should only consist of characters AKQJT9-2"),
            })
            .collect::<Vec<_>>()
            .try_into()
            .expect("Hand should contain 5 cards")
    }

    fn solve_problem(is_p2: bool) -> u32 {
        let mut hands: Vec<_> = SRC
            .lines()
            .map(|line| {
                line.split_once(' ')
                    .expect("Each line should be: '<CCCCC> <bet>'")
            })
            .map(|(hand, bet)| {
                let hand = Self::map_hand(hand, is_p2);
                let bet = bet
                    .parse::<u32>()
                    .ok()
                    .expect("Bet should be parsable as number");
                (hand, bet)
            })
            .map(|(hand, bet)| (Rank::get(hand, is_p2), hand, bet))
            .collect();

        hands.sort_by(|a, b| {
            if a.0 == b.0 {
                let mut i = 0;
                while i < 5 && a.1[i] == b.1[i] {
                    i += 1;
                }
                if i == 5 {
                    Ordering::Equal
                } else if a.1[i] < b.1[i] {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            } else if a.0 < b.0 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });

        hands
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, (_, _, bet))| (i + 1) as u32 * bet)
            .sum()
    }
}

impl Day for Day07 {
    fn problem1() {
        let output = Self::solve_problem(false);
        println!("{:?}", output);
    }

    fn problem2() {
        let output = Self::solve_problem(true);
        println!("{:?}", output);
    }
}
