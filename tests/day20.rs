use advent_2020::day20::{bits_to_usize, solve_a, solve_b};
use advent_2020::utils::read_lines;

#[test]
fn test_bits_to_usize() {
    assert_eq!(bits_to_usize(&[true]), 1);
    assert_eq!(bits_to_usize(&[false]), 0);
    assert_eq!(bits_to_usize(&[false, false, true]), 1);
    assert_eq!(bits_to_usize(&[true, false, true, true]), 11);
}

#[test]
fn a() {
    let data = read_lines("./data/day20_test.txt");
    assert_eq!(solve_a(&data), 20899048083289);
}

#[test]
fn b() {
    let data = read_lines("./data/day20_test.txt");
    assert_eq!(solve_b(&data), 273);
}
