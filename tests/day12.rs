#[cfg(test)]
use advent_2020::day12;
use advent_2020::utils::read_lines;

#[test]
fn test_day12_solve_a() {
    let data = read_lines("./data/day12_test.txt");
    assert_eq!(day12::solve_a(&data), 0);
}

#[test]
fn test_day12_solve_b() {
    let data = read_lines("./data/day12_test.txt");
    assert_eq!(day12::solve_b(&data), 0);
}
