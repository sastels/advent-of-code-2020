#[cfg(test)]
use advent_2020::day9::{self, all_sums};
use advent_2020::utils::read_lines;
use std::collections::HashSet;

#[test]

fn test_day9_all_sums() {
    let actual = all_sums(&[1, 2, 3]);
    let expected: HashSet<usize> = vec![3, 4, 5].into_iter().collect();
    assert_eq!(actual, expected);
}

#[test]
fn test_day9_solve_a() {
    let data = read_lines("./data/day9_test.txt");
    assert_eq!(day9::solve_a(&data, 5), 127);
}

#[test]
#[ignore]
fn test_day9_solve_b() {
    let data = read_lines("./data/day9_test.txt");
    assert_eq!(day9::solve_b(&data, 5), 8);
}
