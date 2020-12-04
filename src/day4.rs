use crate::utils::join_lines;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Passport {
    pub fields: HashMap<String, String>,
}

impl Passport {
    pub fn new(data: &str) -> Self {
        let mut fields: HashMap<String, String> = HashMap::new();
        let re = Regex::new(r"(\S\S\S):(\S+)").unwrap();
        for cap in re.captures_iter(data) {
            fields.insert(cap[1].to_string(), cap[2].to_string());
        }
        Passport { fields }
    }

    pub fn is_valid(&self) -> bool {
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|k| self.fields.contains_key(&k.to_string()))
    }
}

pub fn solve_a(data: &[String]) -> usize {
    let data = join_lines(&data);
    data.iter()
        .map(|line| Passport::new(line))
        .filter(|passport| passport.is_valid())
        .count()
}

pub fn solve_b(_data: &[String]) -> usize {
    0
}
