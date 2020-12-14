use regex::Regex;
use std::collections::HashMap;

pub struct Ferry {
    mask: String,
    mem: HashMap<usize, usize>,
}

impl Ferry {
    pub fn new() -> Self {
        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string();
        let mem: HashMap<usize, usize> = HashMap::new();
        Ferry { mask, mem }
    }

    pub fn apply_mask(&mut self, mask: &str) {
        self.mask = mask.to_string();
    }

    pub fn apply_set(&mut self, addr: usize, value: usize) {
        let or_mask = self.mask.replace("X", "0");
        let or_mask = usize::from_str_radix(&or_mask, 2).unwrap();
        let and_mask = self.mask.replace("X", "1");
        let and_mask = usize::from_str_radix(&and_mask, 2).unwrap();
        let value = (value | or_mask) & and_mask;
        self.mem.insert(addr, value);
    }

    pub fn execute(&mut self, command: &str) {
        let re = Regex::new(r"mask = (.+)").unwrap();
        let cap = re.captures(command);
        match cap {
            Some(cap) => self.apply_mask(&cap[1]),
            None => {
                let re = Regex::new(r"mem\[(\d+)\] = (.+)").unwrap();
                let cap = re.captures(command).unwrap();
                self.apply_set(cap[1].parse().unwrap(), cap[2].parse().unwrap());
            }
        }
    }

    pub fn memory_sum(&self) -> usize {
        self.mem.values().sum()
    }
}

pub fn solve_a(data: &[String]) -> usize {
    let mut ferry = Ferry::new();
    for command in data.iter() {
        ferry.execute(command);
    }
    ferry.memory_sum()
}

pub fn solve_b(_data: &[String]) -> usize {
    unimplemented!()
}
