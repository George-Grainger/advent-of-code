use advent_of_code::Day;

const SRC: &str = include_str!("../../input/day5.txt");

pub struct Five {}

impl Five {
    fn get_maps() -> Vec<Vec<u64>> {
        SRC.split("\r\n\r\n")
            .map(|section| {
                let i = section
                    .find(':')
                    .expect("Should contain header followed by ':'");
                &section[i + 1..]
            })
            .map(|section| {
                section
                    .split_whitespace()
                    .filter_map(|val| val.parse::<u64>().ok())
                    .collect()
            })
            .collect()
    }
}

impl Day for Five {
    fn problem1() {
        let maps = Self::get_maps();
        let seeds = &maps[0];
        let closest = seeds
            .iter()
            .map(|&seed| {
                let mut next = seed;
                for map in &maps[1..] {
                    next = match map
                        .chunks(3)
                        .find(|win| win[1] <= next && next < win[1] + win[2])
                    {
                        Some(win) => (win[0] + next) - win[1],
                        None => next,
                    };
                }
                next
            })
            .min()
            .expect("Should be minimum value");
        println!("{}", closest);
    }

    fn problem2() {
        let maps = Self::get_maps();
        let mut seeds = maps[0].to_vec();

        for map in &maps[1..] {
            let mut working: Vec<u64> = Vec::new();
            for seed in seeds.chunks(2) {
                let mut start = seed[0];
                let mut rem = seed[1];

                while rem > 0 {
                    let (mapped_start, capcity) = match map
                        .chunks(3)
                        .find(|win| win[1] <= start && start < win[1] + win[2])
                    {
                        Some(win) => {
                            let diff = start - win[1];
                            let capacity = rem.min(win[2] - diff);
                            let mapped_start = (win[0] + start) - win[1];
                            (mapped_start, capacity)
                        }
                        None => {
                            let diff = map
                                .iter()
                                .skip(1)
                                .step_by(3)
                                .filter(|bound| start < **bound)
                                .fold(u64::MAX, |closest, next| closest.min(next - start));
                            let capacity = rem.min(diff);
                            (start, capacity)
                        }
                    };
                    working.push(mapped_start);
                    working.push(capcity);
                    rem -= capcity;
                    start += capcity;
                }
            }
            seeds = working;
        }
        let lowest = seeds
            .iter()
            .step_by(2)
            .min()
            .expect("Should be a minimum value");
        println!("{:?}", lowest);
    }
}
