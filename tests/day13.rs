#[cfg(test)]
use advent_2020::day13;
use advent_2020::utils::read_lines;

#[test]
fn test_day13_solve_a() {
    let data = read_lines("./data/day13_test.txt");
    assert_eq!(day13::solve_a(&data), 0);
}

#[test]
fn test_day13_solve_b() {
    let data = read_lines("./data/day13_test.txt");
    assert_eq!(day13::solve_b(&data), 0);
}
