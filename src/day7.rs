// use crate::utils::join_lines;
// use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Rule {
    pub outside: String,
    pub inside: HashMap<String, usize>,
}

impl Rule {
    pub fn new(line: &str) -> Self {
        let line = line.replace("bags", "bag").replace(".", "");
        let re = Regex::new(r"(.+) contain (.+)").unwrap();
        let cap = re.captures(&line).unwrap();
        let outside = cap[1].to_string();
        let mut inside: HashMap<String, usize> = HashMap::new();
        for bag in cap[2].split(", ") {
            if bag != "no other bag" {
                let re = Regex::new(r"(\d+) (.+)").unwrap();
                let bag_match = re.captures(bag).unwrap();
                inside.insert(
                    bag_match[2].to_string(),
                    bag_match[1].parse::<usize>().unwrap(),
                );
            }
        }
        Rule { outside, inside }
    }
}
pub fn solve_a(data: &[String]) -> usize {
    let rules: Vec<Rule> = data.iter().map(|line| Rule::new(line)).collect();
    0
}

pub fn solve_b(_data: &[String]) -> usize {
    0
}
