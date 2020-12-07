#[cfg(test)]
use advent_2020::day7;
use advent_2020::utils::read_lines;

#[test]
fn test_day7_solve_a() {
    let data = read_lines("./data/day7_test.txt");
    assert_eq!(day7::solve_a(&data), 4);
}

#[test]
#[ignore]
fn test_day7_solve_b() {
    let data = read_lines("./data/day7_test.txt");
    assert_eq!(day7::solve_b(&data), 0);
}
