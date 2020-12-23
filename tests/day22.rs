use advent_2020::day22::{solve_a, solve_b};
use advent_2020::utils::read_lines;

#[test]

fn a() {
    let data = read_lines("./data/day22_test.txt");
    assert_eq!(solve_a(&data), 306);
}

#[test]
#[ignore]
fn b() {
    let data = read_lines("./data/day22_test.txt");
    assert_eq!(solve_b(&data), 0);
}
