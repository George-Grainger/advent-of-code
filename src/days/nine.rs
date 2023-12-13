use advent_of_code::Day;

const SRC: &str = include_str!("../../input/day9.txt");

pub struct Nine {}

impl Nine {
    fn get_parsed_input() -> impl Iterator<Item = Vec<Vec<i32>>> {
        SRC.lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<i32>().expect("Each reading should be an integer"))
                    .collect::<Vec<i32>>()
            })
            .map(Self::forward_pass)
    }

    fn forward_pass(readings: Vec<i32>) -> Vec<Vec<i32>> {
        let mut pattern = Vec::new();
        let mut working = readings;
        let mut finished = false;

        while !working.is_empty() && !finished {
            // Requires check before update to get final line of 0s
            finished = working.iter().all(|&n| n == 0);

            // Update combined array
            pattern.push(working.clone());
            working = working.windows(2).map(|win| win[1] - win[0]).collect();
        }

        pattern
    }

    fn fill_forward(mut pattern: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // Update the first row
        let n = pattern.len();
        let last_row = &mut pattern[n - 1];
        let mut val_right = last_row[last_row.len() - 1];
        last_row.push(val_right);

        // Now update every other row
        for i in (1..n).rev() {
            let upper = &mut pattern[i - 1];
            let val_left = upper[upper.len() - 1];
            val_right = val_left + val_right;

            upper.push(val_right);
        }

        pattern
    }

    fn fill_backward(mut pattern: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // Update the first row
        let n = pattern.len();
        let last_row = &mut pattern[n - 1];
        let mut val_right = last_row[0];
        last_row.insert(0, val_right);

        // Now update every other row
        for i in (1..n).rev() {
            let upper = &mut pattern[i - 1];
            let val_left = upper[0];
            val_right = val_left - val_right;

            upper.insert(0, val_right);
        }

        pattern
    }
}

impl Day for Nine {
    fn problem1() {
        let lines = Self::get_parsed_input();

        let output: i32 = lines
            .map(Self::fill_forward)
            .map(|row| {
                row.first()
                    .expect("Pattern should contain at least one row")
                    .last()
                    .expect("Row should contain at least one element")
                    .to_owned()
            })
            .sum();

        println!("{:?}", output);
    }

    fn problem2() {
        let lines = Self::get_parsed_input();

        let output: i32 = lines
            .map(Self::fill_backward)
            .map(|row| {
                row.first()
                    .expect("Pattern should contain at least one row")
                    .first()
                    .expect("Row should contain at least one element")
                    .to_owned()
            })
            .sum();

        println!("{:?}", output);
    }
}
