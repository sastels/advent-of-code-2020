use advent_2020::day24::{move_many, solve_a, solve_b};
use advent_2020::utils::read_lines;

#[test]
fn moving_points() {
    assert_eq!(move_many(0, 0, "nwwswee"), (0, 0));
    assert_eq!(move_many(1, -1, "esewwnwneeesw"), (2, -1));
}
#[test]

fn a() {
    let data = read_lines("./data/day24_test.txt");
    assert_eq!(solve_a(&data), 10);
}

#[test]
#[ignore]
fn b() {
    let data = read_lines("./data/day24_test.txt");
    assert_eq!(solve_b(&data), 0);
}
