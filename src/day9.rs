use std::collections::HashSet;

pub fn all_sums(data: &[usize]) -> HashSet<usize> {
    let mut sums: HashSet<usize> = HashSet::new();
    for x in data {
        for y in data {
            if x != y {
                sums.insert(x + y);
            }
        }
    }
    sums
}

pub fn solve_a(data: &[String], period: usize) -> usize {
    let data: Vec<usize> = data.iter().map(|x| x.parse::<usize>().unwrap()).collect();
    for i in period..data.len() {
        if !all_sums(&data[i - period..i]).contains(&data[i]) {
            return data[i];
        }
    }
    panic!()
}

pub fn solve_b(data: &[String], target: usize) -> usize {
    let data: Vec<usize> = data.iter().map(|x| x.parse::<usize>().unwrap()).collect();
    for i in 0..data.len() {
        for j in i..data.len() {
            let sum: usize = data[i..j].iter().sum();
            if sum == target {
                return data[i..j].iter().min().unwrap() + data[i..j].iter().max().unwrap();
            }
            if sum > target {
                continue;
            }
        }
    }
    panic!()
}
