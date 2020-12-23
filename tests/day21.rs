use advent_2020::day21::{solve_a, solve_b, Food};
use advent_2020::utils::read_lines;
use std::collections::HashSet;
use std::iter::FromIterator;

#[test]
fn test_food() {
    let food = Food::new("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)");
    assert_eq!(
        food.ingredients,
        HashSet::from_iter(vec![
            "mxmxvkd".to_string(),
            "kfcds".to_string(),
            "sqjhc".to_string(),
            "nhms".to_string()
        ])
    );
    assert_eq!(
        food.allergens,
        HashSet::from_iter(vec!["dairy".to_string(), "fish".to_string()])
    );
}

#[test]
fn a() {
    let data = read_lines("./data/day21_test.txt");
    assert_eq!(solve_a(&data), 5);
}

#[test]
#[ignore]
fn b() {
    let data = read_lines("./data/day21_test.txt");
    assert_eq!(solve_b(&data), 0);
}
