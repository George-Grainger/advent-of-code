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
    condition: usize,
    target: &'a str,
}

#[derive(Debug)]
struct RuleEntry<'a> {
    rules: Vec<Rule<'a>>,
    default: &'a str,
}

pub struct Day19 {}

impl Day19 {
    fn deserialize_parts(parts: &str) -> Vec<HashMap<&str, usize>> {
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
                        let v = v.parse::<usize>().unwrap();
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

                        // convert the condition to a usize and create the rule
                        let condition = condition.parse::<usize>().unwrap();
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
            let mut key = "in";
            while key != "A" && key != "R" {
                let rule_entry = ruleset
                    .get(&key)
                    .expect(format!("Key {key} should be in rule set").as_str());
                key = rule_entry
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

            if key == "A" {
                total += part.values().sum::<usize>();
            }
        }

        println!("{}", total);
    }

    fn problem2() {
        todo!();
    }
}
