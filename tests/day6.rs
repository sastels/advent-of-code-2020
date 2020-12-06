#[cfg(test)]
use advent_2020::day6;
use advent_2020::utils::read_lines;

#[test]
fn test_day5_solve_a() {
    let data = read_lines("./data/day6.txt");
    assert_eq!(day6::solve_a(&data), 666);
}

#[test]
fn test_day5_solve_b() {
    let data = read_lines("./data/day6.txt");
    assert_eq!(day6::solve_b(&data), 666);
}
