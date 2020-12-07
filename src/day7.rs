// use crate::utils::join_lines;
// use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

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

    let mut contained_in: HashMap<String, HashSet<String>> = HashMap::new();
    for rule in &rules {
        contained_in.insert(rule.outside.clone(), HashSet::new());
    }
    for rule in &rules {
        for bag_in in rule.inside.keys() {
            let mut in_set: HashSet<String> = contained_in.get(bag_in).unwrap().clone();
            in_set.insert(rule.outside.clone());
            contained_in.insert(bag_in.to_string(), in_set);
        }
    }

    let mut containers: HashSet<String> = HashSet::new();
    containers.insert("shiny gold bag".to_string());
    loop {
        let mut new_containers: HashSet<String> = HashSet::new();
        for bag in &containers {
            for outer_bag in contained_in.get(bag).unwrap() {
                new_containers.insert(outer_bag.to_string());
            }
        }
        if new_containers.is_subset(&containers) {
            break;
        }
        containers.extend(new_containers);
    }
    containers.len() - 1 // still has the shiny gold bag
}

pub fn solve_b(_data: &[String]) -> usize {
    0
}
