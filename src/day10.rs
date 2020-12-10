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

pub fn is_chain_valid(data: &[usize]) -> bool {
    for i in 1..data.len() {
        if data[i] < data[i - 1] || data[i] - data[i - 1] > 3 {
            return false;
        }
    }
    true
}

// if not valid, return 0
// if valid, return 1 + number of included valid sequences
// for each index > fix_upto, delete this entry and call count_included_valid recursively
// the fix_upto ensures that we don't repeat sequences
pub fn count_included_valid(data: &[usize], fix_upto: usize) -> usize {
    if data.len() == 101 {
        println!("{} / {}", fix_upto, 101);
    }

    if !is_chain_valid(data) {
        return 0;
    }
    return 1
        + ((fix_upto + 1)..(data.len() - 1))
            .map(|i| {
                let mut data_copy: Vec<usize> = data.to_vec();
                data_copy.remove(i);
                return count_included_valid(&data_copy, i - 1);
            })
            .sum::<usize>();
}

pub fn solve_b(data: &[String]) -> usize {
    let mut data: Vec<usize> = data.iter().map(|x| x.parse::<usize>().unwrap()).collect();
    data.push(0);
    data.push(data.iter().max().unwrap() + 3);
    data.sort();
    count_included_valid(&data, 0)
}
