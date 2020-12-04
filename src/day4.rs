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

    pub fn all_present(&self) -> bool {
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|k| self.fields.contains_key(&k.to_string()))
    }

    pub fn valid_key(&self, key: &str) -> bool {
        match key {
            "byr" => match self.fields.get(key).unwrap().parse::<i32>() {
                Ok(val) => 1920 <= val && val <= 2002,
                Err(_) => false,
            },
            "iyr" => match self.fields.get(key).unwrap().parse::<i32>() {
                Ok(val) => 2010 <= val && val <= 2020,
                Err(_) => false,
            },
            "eyr" => match self.fields.get(key).unwrap().parse::<i32>() {
                Ok(val) => 2020 <= val && val <= 2030,
                Err(_) => false,
            },
            "hgt" => {
                let re = Regex::new(r"(\d+)(\S\S)").unwrap();
                let cap = re.captures(self.fields.get(key).unwrap());
                match cap {
                    Some(cap) => {
                        let val = cap[1].parse::<i32>().unwrap();
                        (&cap[2] == "cm" && 150 <= val && val <= 193)
                            || (&cap[2] == "in" && 59 <= val && val <= 76)
                    }
                    None => false,
                }
            }
            "hcl" => {
                let val = self.fields.get(key).unwrap();
                if val.len() != 7 {
                    return false;
                }
                let re = Regex::new(r"#[0-9a-f]{6}").unwrap();
                let cap = re.captures(self.fields.get(key).unwrap());
                match cap {
                    Some(_) => true,
                    None => false,
                }
            }
            "ecl" => {
                let val = self.fields.get(key).unwrap();
                ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                    .iter()
                    .filter(|c| c.to_string() == *val)
                    .count()
                    == 1
            }
            "pid" => {
                let val = self.fields.get(key).unwrap();
                if val.len() != 9 {
                    return false;
                }
                let re = Regex::new(r"[\d]{9}").unwrap();
                let cap = re.captures(self.fields.get(key).unwrap());
                match cap {
                    Some(_) => true,
                    None => false,
                }
            }
            &_ => true,
        }
    }

    pub fn all_valid(&self) -> bool {
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
            .iter()
            .all(|k| self.valid_key(k))
    }
}

pub fn solve_a(data: &[String]) -> usize {
    let data = join_lines(&data);
    data.iter()
        .map(|line| Passport::new(line))
        .filter(|passport| passport.all_present())
        .count()
}

pub fn solve_b(data: &[String]) -> usize {
    let data = join_lines(&data);
    data.iter()
        .map(|line| Passport::new(line))
        .filter(|passport| passport.all_present() && passport.all_valid())
        .count()
}
