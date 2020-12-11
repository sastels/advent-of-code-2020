#[cfg(test)]
use advent_2020::day11;
use advent_2020::utils::read_lines;

#[test]
fn test_day11_solve_a() {
    let data = read_lines("./data/day11_test.txt");
    assert_eq!(day11::solve_a(&data), 666);
}

#[test]
#[ignore]
fn test_day11_solve_b() {
    let data = read_lines("./data/day11_test.txt");
    assert_eq!(day11::solve_b(&data), 666);
}
