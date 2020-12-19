use advent_2020::day19::{matches, parse_input, solve_a, solve_b, Rule};
use advent_2020::utils::read_lines;

#[test]
fn rule_new_1() {
    let rule = Rule::new("0: 4 1 5");
    assert_eq!(rule.id, "0");
    assert_eq!(rule.lhs, vec!["4", "1", "5"]);
    assert_eq!(rule.rhs, Vec::<String>::new());
}

#[test]
fn rule_new_2() {
    let rule = Rule::new("4: \"a\"");
    assert_eq!(rule.lhs, vec!["a"]);
    assert_eq!(rule.rhs, Vec::<String>::new());
}

#[test]
fn rule_new_3() {
    let rule = Rule::new("12: 11 2 | 312 56");
    assert_eq!(rule.id, "12");
    assert_eq!(rule.lhs, vec!["11", "2"]);
    assert_eq!(rule.rhs, vec!["312", "56"]);
}

#[test]
fn parse_input_rules() {
    let data = read_lines("./data/day19_test.txt");
    let (rules, _) = parse_input(&data);
    assert_eq!(rules.len(), 6);
    assert_eq!(rules.get("2").unwrap().lhs, vec!["4", "4"])
}

#[test]
fn parse_input_messages() {
    let data = read_lines("./data/day19_test.txt");
    let (_, messages) = parse_input(&data);
    assert_eq!(messages.len(), 5);
    assert_eq!(messages[2], "abbbab");
}

#[test]

fn matches_1() {
    let data = read_lines("./data/day19_test.txt");
    let (rules, _) = parse_input(&data);
    assert!(matches("ababbb", "0", &rules));
    assert!(matches("aba", "aba", &rules));
    assert!(!matches("bababa", "0", &rules));
}

#[test]
#[ignore]
fn a() {
    let data = read_lines("./data/day19_test.txt");
    assert_eq!(solve_a(&data), 0);
}

#[test]
#[ignore]
fn b() {
    let data = read_lines("./data/day19_test.txt");
    assert_eq!(solve_b(&data), 0);
}
