#[cfg(test)]
use advent_2020::day3;
use advent_2020::day3::Hill;
use advent_2020::utils::read_lines;

#[test]
fn test_day3_hill_new() {
    let data = read_lines("./data/day3_test.txt");
    Hill::new(data);
}

#[test]
fn test_day3_hill_tree_at_false() {
    let data = read_lines("./data/day3_test.txt");
    let hill = Hill::new(data);
    assert!(!hill.tree_at(0, 0));
    assert!(!hill.tree_at(3, 3));
}

#[test]
fn test_day3_hill_tree_at_true() {
    let data = read_lines("./data/day3_test.txt");
    let hill = Hill::new(data);
    assert!(hill.tree_at(0, 2));
    assert!(hill.tree_at(4, 1));
}

#[test]
fn test_day3_hill_tree_at_false_wide() {
    let data = read_lines("./data/day3_test.txt");
    let hill = Hill::new(data);
    assert!(!hill.tree_at(0, 15));
    assert!(!hill.tree_at(2, 30));
}

#[test]
fn test_day3_hill_tree_at_true_wide() {
    let data = read_lines("./data/day3_test.txt");
    let hill = Hill::new(data);
    assert!(hill.tree_at(0, 13));
    assert!(hill.tree_at(4, 23));
}

#[test]
fn test_day3_hill_slide_down() {
    let data = read_lines("./data/day3_test.txt");
    let hill = Hill::new(data);
    assert_eq!(hill.slide_down(1, 3), 7);
}

#[test]
fn test_day3_solve_a() {
    let data = read_lines("./data/day3_test.txt");
    assert_eq!(day3::solve_a(&data), 7);
}

#[test]
fn test_day3_solve_b() {
    let data = read_lines("./data/day3_test.txt");
    assert_eq!(day3::solve_b(&data), 336);
}
