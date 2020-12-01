use std::collections::HashMap;

pub fn solve_a(data: Vec<String>) {
    let data = data.iter().map(|s| s.parse::<i32>().unwrap());
    let mut numbers_seen = HashMap::new();
    for num in data {
        match numbers_seen.get(&(2020 - num)) {
            Some(_) => {
                println!("Day 1a: {}, {} -> {}", num, 2020 - num, num * (2020 - num));
            }
            None => {
                numbers_seen.insert(num, 1);
            }
        }
    }
}
