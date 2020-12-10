#[cfg(test)]
use advent_2020::day10;
use advent_2020::utils::read_lines;

#[test]
fn test_day10_solve_a() {
    let data = read_lines("./data/day10_test_1.txt");
    assert_eq!(day10::solve_a(&data), 0);
}

#[test]
#[ignore]
fn test_day10_solve_b() {
    let data = read_lines("./data/day10_test_1.txt");
    assert_eq!(day10::solve_b(&data), 0);
}
