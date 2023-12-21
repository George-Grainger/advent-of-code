use self::Operator::*;
use std::{
    collections::HashMap,
    fmt::Display,
    fmt::{Error, Formatter},
};

use advent_of_code::Day;

const SRC: &str = include_str!("../../input/day19.txt");

#[derive(Debug)]
struct InvalidOperatorError;

impl Display for InvalidOperatorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Operator should be '<' or '>'")
    }
}

#[derive(Debug)]
enum Operator {
    Less,
    Greater,
}

impl Operator {
    fn is_valid(s: char) -> bool {
        s == '>' || s == '<'
    }
}

impl TryFrom<&str> for Operator {
    type Error = InvalidOperatorError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            ">" => Ok(Greater),
            "<" => Ok(Less),
            _ => Err(InvalidOperatorError),
        }
    }
}

#[derive(Debug)]
struct Rule<'a> {
    key: &'a str,
    operator: Operator,
    condition: u64,
    target: &'a str,
}

#[derive(Debug)]
struct RuleEntry<'a> {
    rules: Vec<Rule<'a>>,
    default: &'a str,
}

pub struct Day19 {}

impl Day19 {
    fn deserialize_parts(parts: &str) -> Vec<HashMap<&str, u64>> {
        parts
            .lines()
            .map(|dict| {
                let n = dict.len();
                // Strip '{' and '}'
                &dict[1..n - 1]
            })
            .map(|entries| {
                entries
                    .split(',')
                    .map(|entry| {
                        let (k, v) = entry.split_once('=').expect("Entry should be 'k=v'");
                        let v = v.parse::<u64>().unwrap();
                        (k, v)
                    })
                    .collect()
            })
            .collect()
    }

    fn deserialize_ruleset(ruleset: &str) -> HashMap<&str, RuleEntry> {
        ruleset
            .lines()
            .map(|line| {
                let n = line.len();
                line[..n - 1]
                    .split_once('{')
                    .expect("Should be a key followed rules")
            })
            .map(|(key, rules)| {
                let mut rules = rules.split(',');
                let default = rules.next_back().expect("Should contain defualt value");
                let rules: Vec<Rule> = rules
                    .map(|rule| {
                        // Get the target and condition strings
                        let (cond, target) = rule
                            .split_once(':')
                            .expect("All other keys should contain condition then target key");

                        // Get the operator
                        let operator = if cond.contains('>') { Greater } else { Less };

                        // Split on the operator
                        let (key, condition) = cond
                            .split_once(Operator::is_valid)
                            .expect("Should contain '>' or '<'");

                        // convert the condition to a u64 and create the rule
                        let condition = condition.parse::<u64>().unwrap();
                        Rule {
                            key,
                            operator,
                            condition,
                            target,
                        }
                    })
                    .collect();

                (key, RuleEntry { rules, default })
            })
            .collect()
    }
}

impl Day for Day19 {
    fn problem1() {
        let (rules, parts) = SRC
            .split_once("\r\n\r\n")
            .expect("File should contain list of rules, then line seperator, then list of parts");

        let ruleset = Self::deserialize_ruleset(rules);
        let parts = Self::deserialize_parts(parts);

        let mut total = 0;
        for part in parts {
            // Default rule key
            let mut rule_key = "in";
            while rule_key != "A" && rule_key != "R" {
                let rule_entry = ruleset
                    .get(&rule_key)
                    .expect(format!("Key {rule_key} should be in rule set").as_str());
                rule_key = rule_entry
                    .rules
                    .iter()
                    .find_map(
                        |Rule {
                             key,
                             operator,
                             condition,
                             target,
                         }| {
                            part.get(key).and_then(|val| match operator {
                                Less if val < condition => Some(target),
                                Greater if val > condition => Some(target),
                                _ => None,
                            })
                        },
                    )
                    .unwrap_or(&rule_entry.default);
            }

            if rule_key == "A" {
                total += part.values().sum::<u64>();
            }
        }

        println!("{}", total);
    }

    fn problem2() {
        let (rules, _) = SRC
            .split_once("\r\n\r\n")
            .expect("File should contain list of rules, then line seperator, then list of parts");

        let ruleset = Self::deserialize_ruleset(rules);

        // Create initial state
        let mut total = 0;
        let mut state = HashMap::with_capacity(4);
        state.insert("x", (1, 4000));
        state.insert("a", (1, 4000));
        state.insert("m", (1, 4000));
        state.insert("s", (1, 4000));

        // Create stack of all states
        let mut stack = Vec::new();
        stack.push(("in", state));

        while let Some((rule_key, mut state)) = stack.pop() {
            if rule_key == "R" {
                continue;
            }

            if rule_key == "A" {
                total += state
                    .values()
                    .map(|(min, max)| (max + 1) - min)
                    .product::<u64>();
                continue;
            }

            let rule_entry = ruleset
                .get(&rule_key)
                .expect(format!("Key {rule_key} should be in rule set").as_str());

            for Rule {
                key,
                operator,
                condition,
                target,
            } in rule_entry.rules.iter()
            {
                let (curr_min, curr_max) = state.get(key).unwrap();
                match operator {
                    Less => {
                        if condition < curr_min {
                            continue;
                        }
                        assert!(condition < curr_max);

                        // Add value taking path
                        let mut new_state = state.clone();
                        new_state.insert(key, (*curr_min, *condition - 1));
                        stack.push((target, new_state));

                        // Update state for not taking path
                        state.insert(key, (*condition, *curr_max));
                    }
                    Greater => {
                        if condition > curr_max {
                            continue;
                        }
                        assert!(condition > curr_min);

                        // Add value taking path
                        let mut new_state = state.clone();
                        new_state.insert(key, (*condition + 1, *curr_max));
                        stack.push((target, new_state));

                        // Update state for not taking path
                        state.insert(key, (*curr_min, *condition));
                    }
                }
            }
            stack.push((rule_entry.default, state));
        }

        println!("{}", total);
    }
}
