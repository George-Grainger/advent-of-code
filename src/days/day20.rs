use advent_of_code::Day;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
};

const SRC: &str = include_str!("../../input/day20.txt");

#[derive(Default, Clone, Debug)]
struct FlipFlop {
    state: bool,
}

impl Module for FlipFlop {
    fn tick(&mut self, signal: bool, _: String) {
        if !signal {
            self.state = !self.state;
        }
    }

    fn get_state(&self) -> bool {
        self.state
    }

    fn cont(&self, signal: bool) -> bool {
        !signal
    }
}

#[derive(Default, Clone, Debug)]
struct Conjunction {
    inputs: HashMap<String, bool>,
    state: bool,
}

impl Module for Conjunction {
    fn tick(&mut self, signal: bool, name: String) {
        self.inputs.insert(name, signal);
        self.state = !self.inputs.values().all(|&p| p);
    }
    fn get_state(&self) -> bool {
        self.state
    }
    fn account(&mut self, name: String) {
        self.inputs.insert(name, false);
    }
}
#[derive(Default, Clone, Debug)]
struct Broadcaster {
    state: bool,
}

impl Module for Broadcaster {
    fn tick(&mut self, signal: bool, _: String) {
        self.state = signal;
    }
    fn get_state(&self) -> bool {
        self.state
    }
}

trait Module: Debug {
    fn tick(&mut self, signal: bool, name: String);
    fn get_state(&self) -> bool;
    fn account(&mut self, _: String) {}
    fn cont(&self, _: bool) -> bool {
        true
    }
}

pub struct Day20 {}

impl Day20 {
    fn parse_input() -> (
        HashMap<String, Box<dyn Module>>,
        HashMap<String, Vec<String>>,
    ) {
        let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
        let mut connections: HashMap<String, Vec<String>> = HashMap::new();

        SRC.lines().for_each(|l| {
            let (src, dst) = l.split_once(" -> ").unwrap();
            let (comp, name): (Box<dyn Module>, &str) = match src.split_at(1) {
                ("%", src) => (Box::new(FlipFlop::default()), src),
                ("&", src) => (Box::new(Conjunction::default()), src),
                ("b", _) => (Box::new(Broadcaster::default()), "broadcaster"),
                (p, r) => panic!("Unexpected prefix {} found on line '{} {}'", p, p, r),
            };

            let next = dst.split(", ").map(|s| s.to_string()).collect();
            assert!(modules.insert(name.trim().to_string(), comp).is_none());

            connections.insert(name.trim().to_string(), next);
        });

        for (key, value) in connections.iter() {
            for module in value {
                modules.get_mut(module).map(|n| n.account(key.clone()));
            }
        }

        (modules, connections)
    }

    fn get_inputs(target: String, connections: &HashMap<String, Vec<String>>) -> Vec<&String> {
        connections
            .iter()
            .filter(|(_, module)| module.contains(&target))
            .map(|(name, _)| name)
            .collect()
    }

    fn get_types(keys: &Vec<&String>, modules: &HashMap<String, Box<dyn Module>>) -> Vec<String> {
        keys.iter()
            .map(|&key| modules.get(key).expect("Key should be in map"))
            .map(|val| {
                format!("{:?}", val)
                    .split_once(" ")
                    .expect("Should include ' '")
                    .0
                    .to_string()
            })
            .collect()
    }

    // Find GCD
    fn gcd(mut a: u64, mut b: u64) -> u64 {
        if a == b {
            return a;
        }
        if b > a {
            let temp = a;
            a = b;
            b = temp;
        }
        while b > 0 {
            let temp = a;
            a = b;
            b = temp % b;
        }
        return a;
    }

    fn lcm(a: u64, b: u64) -> u64 {
        // LCM = a*b / gcd
        return a * (b / Self::gcd(a, b));
    }
}

impl Day for Day20 {
    fn problem1() {
        let (mut modules, connections) = Self::parse_input();

        let mut hc = 0;
        let mut lc = 0;
        let mut queue = VecDeque::new();
        for _ in 0..1000 {
            lc += 1;
            queue.push_back(("broadcaster".to_string(), false));
            while let Some((name, signal)) = queue.pop_front() {
                for n in connections.get(&name).expect("Name is in connections") {
                    match signal {
                        false => lc += 1,
                        true => hc += 1,
                    }

                    // Don't care if output or rx value?
                    if n == "rx" || n == "output" {
                        continue;
                    }

                    let next = modules.get_mut(n).unwrap();
                    if next.cont(signal) {
                        next.tick(signal, name.clone());
                        queue.push_back((n.to_string(), next.get_state()));
                    }
                }
            }
        }

        println!("{}", hc * lc)
    }

    fn problem2() {
        let (mut modules, connections) = Self::parse_input();

        // Notice that rx only has one element feeding it
        let rx_inputs = Self::get_inputs("rx".to_string(), &connections);
        let types = Self::get_types(&rx_inputs, &modules);
        let rx_zip: Vec<(_, _)> = rx_inputs.iter().zip(types).collect::<Vec<_>>();
        println!("\nInputs to rx: {:?} ", rx_zip);
        assert_eq!(rx_inputs.len(), 1, "Notice that rx only has one element");
        println!("Note: cn is conjugation meaning rx produces high pulse iff cn produces low\n");

        // Notice that cd only has 4 element feeding it
        let cd_inputs = Self::get_inputs("cn".to_string(), &connections);
        let types = Self::get_types(&cd_inputs, &modules);
        let cd_zip: Vec<(_, _)> = cd_inputs.iter().zip(types).collect();
        println!("Inputs to cn: {:?} ", cd_zip);
        assert_eq!(cd_inputs.len(), 4, "Notice that cd only has one element");
        println!("Note: cn's inputs all conjugations too. cn will produce low pulse when all inputs are high pulse.");
        println!("This will occur at lcm of the cycle lengths of these inputs\n");

        let cd_inputs: HashSet<&String> = cd_inputs.into_iter().collect();
        let mut seen: HashMap<_, _> = cd_inputs.iter().map(|&s| (s.clone(), None)).collect();

        // Find the cycles
        let mut presses = 0;
        let mut queue = VecDeque::new();
        while seen.values().any(|val| val.is_none()) {
            presses += 1;
            queue.push_back(("broadcaster".to_string(), false));
            while let Some((name, signal)) = queue.pop_front() {
                for n in connections.get(&name).expect("Name is in connections") {
                    if n == "cn" && signal && seen.get(&name).unwrap().is_none() {
                        seen.insert(name.clone(), Some(presses));
                    }

                    // Don't care if output or rx value?
                    if n == "rx" || n == "output" {
                        continue;
                    }

                    let next = modules.get_mut(n).unwrap();
                    if next.cont(signal) {
                        next.tick(signal, name.clone());
                        queue.push_back((n.to_string(), next.get_state()));
                    }
                }
            }
        }

        // Calculate the lcm
        let lcm = seen
            .values()
            .fold(1, |acc, &val| Self::lcm(acc, val.unwrap()));
        println!("{}", lcm);
    }
}
