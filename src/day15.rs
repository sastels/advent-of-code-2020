use std::collections::HashMap;

pub fn solve_a(data: &[String]) -> usize {
    let input: Vec<usize> = data[0]
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut last_said: HashMap<usize, usize> = HashMap::new();
    let mut new_number: usize = 0;
    for i in 0..2020 {
        let last_number = new_number;
        if i < input.len() {
            new_number = input[i];
        } else {
            if last_said.get(&last_number).is_none() {
                new_number = 0;
            } else {
                new_number = i - 1 - last_said.get(&last_number).unwrap()
            }
        }
        if i > 0 {
            last_said.insert(last_number, i - 1);
        }
    }
    new_number
}

pub fn solve_b(data: &[String]) -> usize {
    let input: Vec<usize> = data[0]
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut last_said: HashMap<usize, usize> = HashMap::new();
    let mut new_number: usize = 0;
    for i in 0..30000000 {
        if i % 100000 == 0 {
            println!("{} / {}", i / 100000, 300);
        }
        let last_number = new_number;
        if i < input.len() {
            new_number = input[i];
        } else {
            if last_said.get(&last_number).is_none() {
                new_number = 0;
            } else {
                new_number = i - 1 - last_said.get(&last_number).unwrap()
            }
        }
        if i > 0 {
            last_said.insert(last_number, i - 1);
        }
    }
    new_number
}
