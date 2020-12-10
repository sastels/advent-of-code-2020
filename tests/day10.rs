#[cfg(test)]
use advent_2020::day10::{self, is_chain_valid};
use advent_2020::utils::read_lines;

#[test]
fn test_day10_solve_a() {
    let data = read_lines("./data/day10_test_1.txt");
    assert_eq!(day10::solve_a(&data), 7 * 5);
}

#[test]
fn test_day10_is_chain_valid_true() {
    assert!(is_chain_valid([0, 1, 2]));
    assert!(is_chain_valid([2, 5, 8]));
    assert!(is_chain_valid([0, 1, 3, 5, 6, 9]));
}

#[test]
fn test_day10_is_chain_valid_false() {
    assert!(!is_chain_valid([0, 4]));
    assert!(!is_chain_valid([0, 2, 4, 20]));
}

#[test]
#[ignore]
fn test_day10_solve_b() {
    let data = read_lines("./data/day10_test_1.txt");
    assert_eq!(day10::solve_b(&data), 0);
}
