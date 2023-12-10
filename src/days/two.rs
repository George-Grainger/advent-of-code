use advent_of_code::Day;
use std::collections::HashMap;

const SRC: &str = include_str!("../../static/day2input.txt");

pub struct Two {}

impl Two {
    fn process_games() -> impl Iterator<Item = HashMap<&'static str, i32>> {
        SRC.lines()
            .map(|l| l.split(|c| c == ':' || c == ';').skip(1))
            .map(|game| {
                game.fold(HashMap::new(), |mut acc, round| {
                    round.split(",").for_each(|draw| {
                        let (count, color) = draw[1..]
                            .split_once(" ")
                            .expect("Should be '<count> <color>'");
                        let count: i32 = count.parse().expect("Count should be an integer");
                        let cc = acc.entry(color).or_insert(0);
                        *cc = i32::max(*cc, count);
                    });
                    acc
                })
            })
    }
}

impl Day for Two {
    fn problem1() {
        let mut maxes = HashMap::with_capacity(3);
        maxes.insert("red", 12);
        maxes.insert("green", 13);
        maxes.insert("blue", 14);

        let total: usize = Self::process_games()
            .enumerate()
            .filter(|(_, counts)| {
                counts.iter().all(|(k, v)| {
                    v <= maxes
                        .get(k)
                        .expect("Key should be one of 'red', 'green' or 'blue'")
                })
            })
            .map(|(i, _)| i + 1)
            .sum();

        println!("{:?}", total);
    }

    fn problem2() {
        let total: i32 = Self::process_games()
            .map(|counts| counts.values().product::<i32>())
            .sum();

        println!("{}", total);
    }
}
