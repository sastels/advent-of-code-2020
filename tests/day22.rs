use advent_2020::day22::{solve_a, solve_b};
use advent_2020::utils::read_lines;

#[test]
#[ignore]
fn a() {
    let data = read_lines("./data/day13_test.txt");
    assert_eq!(solve_a(&data), 0);
}

#[test]
#[ignore]
fn b() {
    let data = read_lines("./data/day13_test.txt");
    assert_eq!(solve_b(&data), 0);
}
