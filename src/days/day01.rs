use advent_of_code::Day;

const SRC: &str = include_str!("../../input/day01.txt");

pub struct Day01 {}

impl Day01 {
    fn get_calibration(line: &str) -> u32 {
        let first = line
            .bytes()
            .find(|&b| b.is_ascii_digit())
            .expect("Should be at least one number?");

        let last = line
            .bytes()
            .rfind(|&b| b.is_ascii_digit())
            .expect("Should be at least one number?");

        (10 * (first - 48) + (last - 48)) as u32
    }

    fn get_calibration_p2(line: &str) -> u32 {
        const DIGITS: [&str; 9] = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        let mut first = None;
        let mut last = 0;

        // Simplify digit update
        let mut digit = |b| {
            first = first.or(Some(b));
            last = b;
        };

        let chars = line.as_bytes();
        for (i, b) in chars.iter().enumerate() {
            if b.is_ascii_digit() {
                digit(b - b'0');
            } else {
                for (j, d) in DIGITS.iter().enumerate() {
                    if chars[i..].starts_with(d.as_bytes()) {
                        digit(j as u8 + 1);
                    }
                }
            }
        }

        let first = first.expect("Should be at least one digit");
        (10 * first + last) as u32
    }
}

impl Day for Day01 {
    fn problem1() {
        let output: u32 = SRC.lines().map(Self::get_calibration).sum();
        println!("{:?}", output);
    }

    fn problem2() {
        let output: u32 = SRC.lines().map(Self::get_calibration_p2).sum();
        println!("{:?}", output);
    }
}
