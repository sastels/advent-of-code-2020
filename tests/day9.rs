#[cfg(test)]
use advent_2020::day9;
use advent_2020::utils::read_lines;

#[test]
fn test_day9_solve_a() {
    let data = read_lines("./data/day9_test.txt");
    assert_eq!(day9::solve_a(&data, 5), 179);
}

#[test]
#[ignore]
fn test_day9_solve_b() {
    let data = read_lines("./data/day9_test.txt");
    assert_eq!(day9::solve_b(&data, 5), 8);
}
