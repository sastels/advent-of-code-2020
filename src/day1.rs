use std::collections::HashMap;

fn parse_data(data: &[String]) -> Vec<i32> {
    data.iter().map(|s| s.parse::<i32>().unwrap()).collect()
}

pub fn solve_a(data: &[String]) -> i32 {
    let data = parse_data(data);
    let mut numbers_seen: HashMap<i32, i32> = HashMap::new();

    for num in &data {
        match numbers_seen.get(&(2020 - num)) {
            Some(_) => return num * (2020 - num),
            None => {
                numbers_seen.insert(*num, 1);
            }
        }
    }
    -1
}

pub fn solve_b(data: &[String]) -> i32 {
    let data = parse_data(data);
    let mut sums: HashMap<i32, i32> = HashMap::new();

    for num1 in &data {
        match sums.get(&(2020 - num1)) {
            Some(num2) => return num1 * num2 * (2020 - num1 - num2),
            None => {
                for num2 in &data {
                    if num1 < num2 && num1 + num2 <= 2020 {
                        let sum = num1 + num2;
                        sums.insert(sum, *num2);
                    }
                }
            }
        }
    }
    -1
}
