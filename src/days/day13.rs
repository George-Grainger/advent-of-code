use advent_of_code::Day;

const SRC: &str = include_str!("../../input/day13.txt");

pub struct Day13 {}

impl Day13 {
    fn transpose2d<T: Copy>(matrix: Vec<Vec<T>>) -> Vec<Vec<T>> {
        if matrix.is_empty() || matrix[0].is_empty() {
            return Vec::new();
        }

        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut transposed = vec![vec![matrix[0][0]; rows]; cols];

        for i in 0..rows {
            for j in 0..cols {
                transposed[j][i] = matrix[i][j];
            }
        }

        transposed
    }

    fn smudge_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>, smudge_available: &mut bool) -> bool {
        if *smudge_available {
            let diff = a.iter().zip(b.iter()).filter(|(a, b)| a != b).count();
            if diff == 0 {
                true
            } else if diff == 1 {
                *smudge_available = false;
                true
            } else {
                false
            }
        } else {
            a == b
        }
    }

    fn get_n_above<T: PartialEq>(pattern: &Vec<Vec<T>>, is_p2: bool) -> Option<usize> {
        let n = pattern.len() - 1;

        for reflection in 1..=n {
            let mut low = reflection - 1;
            let mut high = reflection;
            let mut can_smudge = is_p2;
            let mut valid = Self::smudge_match(&pattern[low], &pattern[high], &mut can_smudge);
            while low > 0 && high < n && valid {
                low -= 1;
                high += 1;
                valid = Self::smudge_match(&pattern[low], &pattern[high], &mut can_smudge);
            }
            if valid && !can_smudge {
                return Some(reflection);
            }
        }

        None
    }

    fn solve(is_p2: bool) {
        let patterns: Vec<Vec<Vec<u8>>> = SRC
            .split("\r\n\r\n")
            .map(|pattern| {
                pattern
                    .lines()
                    .map(|line| line.as_bytes().to_vec())
                    .collect()
            })
            .collect();

        // Transpose array so can reuse row method
        let patterns_t: Vec<Vec<Vec<u8>>> = patterns
            .iter()
            .map(|pattern| Self::transpose2d(pattern.to_vec()))
            .collect();

        let mut sum = 0;
        // Sum from rows
        for (pattern, pattern_t) in patterns.iter().zip(patterns_t.iter()) {
            if let Some(above) = Self::get_n_above(pattern, is_p2) {
                sum += 100 * above;
            } else if let Some(left) = Self::get_n_above(pattern_t, is_p2) {
                sum += left;
            } else {
                panic!("Shouldn't get here?");
            }
        }

        println!("{}", sum);
    }
}

impl Day for Day13 {
    fn problem1() {
        Self::solve(false);
    }

    fn problem2() {
        Self::solve(true);
    }
}
