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

// can assume data passed in is valid
// if valid, return 1 + number of included valid sequences
// for each index > fix_upto, delete this entry and check if valid. If so, call recursively
// the fix_upto ensures that we don't repeat sequences
pub fn count_included_valid(data: &[usize], fix_upto: usize) -> usize {
    if data.len() >= 90 {
        println!("{} / {}", fix_upto, data.len());
    }

    return 1
        + ((fix_upto + 1)..(data.len() - 1))
            .map(|i| {
                let mut sub_chain: Vec<usize> = data.to_vec();
                sub_chain.remove(i);
                if sub_chain[i] - sub_chain[i - 1] > 3 {
                    return 0;
                } else {
                    // sub_chain must be valid!
                    return count_included_valid(&sub_chain, i - 1);
                }
            })
            .sum::<usize>();
}

fn consecutive_slices(data: &[usize]) -> Vec<&[usize]> {
    let mut slice_start = 0;
    let mut result = Vec::new();
    for i in 1..data.len() {
        if data[i - 1] + 1 != data[i] {
            result.push(&data[slice_start..i]);
            slice_start = i;
        }
    }
    if data.len() > 0 {
        result.push(&data[slice_start..]);
    }
    result
}

fn powerset<T>(s: &[T]) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..2usize.pow(s.len() as u32))
        .map(|i| {
            s.iter()
                .enumerate()
                .filter(|&(t, _)| (i >> t) % 2 == 1)
                .map(|(_, element)| element.clone())
                .collect()
        })
        .collect()
}

pub fn delete_group_valid(data: &[usize], group: &[usize]) -> usize {
    let all_subgroups = powerset(group);
    let mut num_valid = 0;
    for group in all_subgroups {
        let sub_chain: Vec<usize> = data
            .iter()
            .enumerate()
            .filter(|(i, _)| !group.contains(i))
            .map(|(_, x)| *x)
            .collect();

        if is_chain_valid(&sub_chain) {
            num_valid += 1
        }
    }
    num_valid
}

pub fn solve_b(data: &[String]) -> usize {
    let mut data: Vec<usize> = data.iter().map(|x| x.parse::<usize>().unwrap()).collect();
    data.push(0);
    data.push(data.iter().max().unwrap() + 3);
    data.sort();

    let mut can_delete: Vec<usize> = vec![];
    for i in 1..data.len() - 1 {
        let mut sub_chain: Vec<usize> = data.to_vec();
        sub_chain.remove(i);
        if is_chain_valid(&sub_chain) {
            can_delete.push(i);
        }
    }
    let can_delete_groups = consecutive_slices(&can_delete);
    for group in &can_delete_groups {
        let num_valid = delete_group_valid(&data, group);
    }
    can_delete_groups
        .iter()
        .map(|group| delete_group_valid(&data, group))
        .product()
}
