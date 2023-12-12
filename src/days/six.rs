use advent_of_code::Day;

const SRC: &str = include_str!("../../input/day6.txt");

pub struct Six {}

impl Six {}

impl Day for Six {
    fn problem1() {
        // Parse the input
        let mut data = SRC
            .lines()
            .map(|line| line.split_once(":").expect("Lines should contain ':'").1)
            .map(|scores| {
                scores
                    .split_whitespace()
                    .filter_map(|score| score.parse::<u32>().ok())
            });

        // Access iterator for each line
        let times = data.next().expect("First line should be times");
        let best = data.next().expect("Second should be distances");

        // Calculate number of possible ways to exceed best
        let output: usize = times
            .map(|max_time| (0..max_time).map(move |time| time * (max_time - time)))
            .zip(best)
            .map(|(seen, lim)| seen.filter(move |&s| s > lim))
            .map(|val| val.count())
            .product();

        println!("{:?}", output);
    }

    fn problem2() {
        let mut data = SRC
            .lines()
            .map(|line| line.split_once(":").expect("Line should contain ':'").1)
            .map(|scores| {
                scores
                    .split_whitespace()
                    .flat_map(|score| score.chars())
                    .collect::<String>()
            })
            .filter_map(|w| w.parse::<u64>().ok());

        // Access iterator for each line
        let max_time = data.next().expect("First line should be times");
        let best = data.next().expect("Second should be distances");

        let times = (0..max_time).map(move |time| time * (max_time - time));
        let output = times.filter(|&time| time > best).count();

        println!("{:?}", output);
    }
}
