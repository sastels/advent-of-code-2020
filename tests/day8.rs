#[cfg(test)]
use advent_2020::day8;
use advent_2020::utils::read_lines;

#[test]
fn test_day8_solve_a() {
    let data = read_lines("./data/day8_test.txt");
    assert_eq!(day8::solve_a(&data), 4);
}

#[test]
fn test_day8_solve_b() {
    let data = read_lines("./data/day8_test.txt");
    assert_eq!(day8::solve_b(&data), 32);
}
