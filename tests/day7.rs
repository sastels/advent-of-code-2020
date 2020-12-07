#[cfg(test)]
use advent_2020::day7;
use advent_2020::day7::Rule;
use advent_2020::utils::read_lines;

#[test]
fn test_day7_new() {
    let rule = Rule::new("light red bags contain 1 bright white bag, 2 muted yellow bags.");
    assert_eq!(rule.outside, "light red bag");
    assert!(rule.inside.contains_key("bright white bag"));
    assert!(rule.inside.contains_key("muted yellow bag"));
    assert_eq!(*rule.inside.get("bright white bag").unwrap(), 1);
    assert_eq!(*rule.inside.get("muted yellow bag").unwrap(), 2);
}

#[test]
fn test_day7_solve_a() {
    let data = read_lines("./data/day7_test.txt");
    assert_eq!(day7::solve_a(&data), 4);
}

#[test]
fn test_day7_solve_b() {
    let data = read_lines("./data/day7_test.txt");
    assert_eq!(day7::solve_b(&data), 32);
}
