use advent_2020::day24::{solve_a, solve_b, HexGrid};
use advent_2020::utils::read_lines;

#[test]
fn moving_points() {
    assert_eq!(HexGrid::move_many(0, 0, "nwwswee"), (0, 0));
    assert_eq!(HexGrid::move_many(1, -1, "esewwnwneeesw"), (2, -1));
}

#[test]
fn neighbours() {
    let mut grid = HexGrid::new(20, 10);
    grid.flip(1, 0);
    grid.flip(-1, 1);
    grid.flip(1, 1);
    assert_eq!(grid.num_true_neighbours(0, 0), 2);
    assert_eq!(grid.num_true_neighbours(0, 2), 1);
}

#[test]
fn a() {
    let data = read_lines("./data/day24_test.txt");
    assert_eq!(solve_a(&data), 10);
}

#[test]
fn b() {
    let data = read_lines("./data/day24_test.txt");
    assert_eq!(solve_b(&data), 2208);
}
