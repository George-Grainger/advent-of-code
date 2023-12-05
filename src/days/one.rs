use advent_of_code::Day;

const SRC: &str = include_str!("../../static/day1input.txt");

pub struct One {}

impl One {
    fn get_calibration(line: &str) -> u32 {
        let first = line
            .bytes()
            .find(|&b| 48 <= b && b <= 57)
            .expect("Should be at least one number?");

        let last = line
            .bytes()
            .rfind(|&b| 48 <= b && b <= 57)
            .expect("Should be at least one number?");

        (10 * (first - 48) + (last - 48)) as u32
    }

    fn get_calibration_p2(line: &str) -> u32 {
        let first = Self::get_num(line, true);

        let rev_line: String = line.chars().rev().collect();
        let last = Self::get_num(&rev_line, false);

        (10 * first + last) as u32
    }

    fn get_num(line: &str, forward: bool) -> u8 {
        let mut seen = ['\0'; 5];

        for c in line.chars() {
            seen.rotate_left(1);
            seen[4] = c;

            if forward {
                return match seen {
                    [_, _, _, _, '0'..='9'] => c as u8 - b'0',
                    [_, _, 'o', 'n', 'e'] => 1,
                    [_, _, 't', 'w', 'o'] => 2,
                    ['t', 'h', 'r', 'e', 'e'] => 3,
                    [_, 'f', 'o', 'u', 'r'] => 4,
                    [_, 'f', 'i', 'v', 'e'] => 5,
                    [_, _, 's', 'i', 'x'] => 6,
                    ['s', 'e', 'v', 'e', 'n'] => 7,
                    ['e', 'i', 'g', 'h', 't'] => 8,
                    [_, 'n', 'i', 'n', 'e'] => 9,
                    _ => continue,
                };
            } else {
                // Might be a nicer way then reversing here - could
                return match seen {
                    [_, _, _, _, '0'..='9'] => c as u8 - b'0',
                    [_, _, 'e', 'n', 'o'] => 1,
                    [_, _, 'o', 'w', 't'] => 2,
                    ['e', 'e', 'r', 'h', 't'] => 3,
                    [_, 'r', 'u', 'o', 'f'] => 4,
                    [_, 'e', 'v', 'i', 'f'] => 5,
                    [_, _, 'x', 'i', 's'] => 6,
                    ['n', 'e', 'v', 'e', 's'] => 7,
                    ['t', 'h', 'g', 'i', 'e'] => 8,
                    [_, 'e', 'n', 'i', 'n'] => 9,
                    _ => continue,
                };
            }
        }

        unreachable!("Every line should contain a number!")
    }
}

impl Day for One {
    fn problem1() {
        let output: u32 = SRC.lines().map(Self::get_calibration).sum();
        println!("{:?}", output);
    }

    fn problem2() {
        let output: u32 = SRC.lines().map(Self::get_calibration_p2).sum();
        println!("{:?}", output);
    }
}
