use advent_2020::day15::{solve_a, solve_b};
use advent_2020::utils::read_lines;

#[test]
fn a() {
    let data = read_lines("./data/day15_test.txt");
    assert_eq!(solve_a(&data), 436);
}

#[test]
#[ignore]
fn b() {
    let data = read_lines("./data/day15_test.txt");
    assert_eq!(solve_b(&data), 0);
}
