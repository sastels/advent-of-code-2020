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

    pub fn expand_address(&self, addr_string: String) -> Vec<usize> {
        let num_x = addr_string.chars().filter(|c| *c == 'X').count();

        let mut addresses: Vec<usize> = vec![];
        for n in 0..(2 << num_x) {
            let mut index = 0;
            let addr: String = addr_string
                .chars()
                .map(|c| {
                    if c != 'X' {
                        return c;
                    } else {
                        let n_bit = (n >> index) & 1;
                        index += 1;
                        if n_bit == 0 {
                            return '0';
                        } else {
                            return '1';
                        }
                    }
                })
                // .inspect(|c| println!("{}", c))
                .collect();
            addresses.push(usize::from_str_radix(&addr, 2).unwrap());
        }
        addresses
    }

    pub fn apply_set_b(&mut self, addr: usize, value: usize) {
        let addr_string: String =
            "000000000000000000000000000000000000".to_string() + &format!("{:b}", addr);
        let addr_string = addr_string[addr_string.len() - 36..].to_string();

        // new address with X's
        let addr_string: String = addr_string
            .chars()
            .zip(self.mask.chars())
            .map(|(a, m)| {
                if m == '0' {
                    return a;
                } else {
                    return m;
                }
            })
            .collect();

        for addr in self.expand_address(addr_string) {
            self.mem.insert(addr, value);
        }
    }

    pub fn execute_b(&mut self, command: &str) {
        let re = Regex::new(r"mask = (.+)").unwrap();
        let cap = re.captures(command);
        match cap {
            Some(cap) => self.apply_mask(&cap[1]),
            None => {
                let re = Regex::new(r"mem\[(\d+)\] = (.+)").unwrap();
                let cap = re.captures(command).unwrap();
                self.apply_set_b(cap[1].parse().unwrap(), cap[2].parse().unwrap());
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

pub fn solve_b(data: &[String]) -> usize {
    let mut ferry = Ferry::new();
    for command in data.iter() {
        ferry.execute_b(command);
    }
    ferry.memory_sum()
}
