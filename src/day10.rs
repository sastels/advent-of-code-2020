use std::collections::HashMap;

pub fn solve_a(data: &[String]) -> usize {
    let mut data: Vec<usize> = data.iter().map(|x| x.parse::<usize>().unwrap()).collect();
    data.push(0);
    data.push(data.iter().max().unwrap() + 3);
    data.sort();

    let mut diffs: HashMap<usize, usize> = HashMap::new();
    for i in 1..data.len() {
        let counter = diffs.entry(data[i] - data[i - 1]).or_insert(0);
        *counter += 1;
    }
    diffs[&1] * diffs[&3]
}

pub fn is_chain_valid(_data: &[usize]) -> bool {
    true
}

pub fn solve_b(_data: &[String]) -> usize {
    unimplemented!();
}
