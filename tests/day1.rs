use advent_2020::day1;
use advent_2020::utils::read_lines;

#[test]
fn test_day1_solve_a() {
    let data = read_lines("./data/day1.txt");
    assert_eq!(day1::solve_a(&data), 989824);
}

#[test]
fn test_day1_solve_b() {
    let data = read_lines("./data/day1.txt");
    assert_eq!(day1::solve_b(&data), 66432240);
}
