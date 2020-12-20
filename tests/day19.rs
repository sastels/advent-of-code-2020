use advent_2020::day19::{find_first_diff_pos, matches, parse_input, solve_a, solve_b, Rule};
use advent_2020::utils::read_lines;

#[test]
fn diff_strings() {
    assert_eq!(find_first_diff_pos("abc", "abc"), None);
    assert_eq!(find_first_diff_pos("abd", "abc"), Some(2));
    assert_eq!(find_first_diff_pos("ab", "abc"), Some(2));
    assert_eq!(find_first_diff_pos("ab", ""), Some(0));
}

#[test]
fn rule_new_1() {
    let rule = Rule::new("0: 4 1 5");
    assert_eq!(rule.id, "0");
    assert_eq!(rule.lhs, "4 1 5");
    assert_eq!(rule.rhs, "");
}

#[test]
fn rule_new_2() {
    let rule = Rule::new("4: \"a\"");
    assert_eq!(rule.lhs, "a");
    assert_eq!(rule.rhs, "");
}

#[test]
fn rule_new_3() {
    let rule = Rule::new("12: 11 2 | 312 56");
    assert_eq!(rule.id, "12");
    assert_eq!(rule.lhs, "11 2");
    assert_eq!(rule.rhs, "312 56");
}

#[test]
fn parse_input_rules() {
    let data = read_lines("./data/day19_test.txt");
    let (rules, _) = parse_input(&data);
    assert_eq!(rules.len(), 6);
    assert_eq!(rules.get("2").unwrap().lhs, "4 4")
}

#[test]
fn parse_input_messages() {
    let data = read_lines("./data/day19_test.txt");
    let (_, messages) = parse_input(&data);
    assert_eq!(messages.len(), 5);
    assert_eq!(messages[2], "abbbab");
}

#[test]
fn matches_simple() {
    let data = read_lines("./data/day19_test.txt");
    let (rules, _) = parse_input(&data);
    assert!(matches("aba", "a b a", &rules));
    assert!(matches("abbbab", "a b b b a b", &rules));
    assert!(!matches("ba", "a b a", &rules));
    assert!(!matches("adbbbab", "a b b b a b", &rules));
}

#[test]
fn matches_medium() {
    let data = read_lines("./data/day19_test.txt");
    let (rules, _) = parse_input(&data);
    assert!(matches("a", "4", &rules));
    assert!(matches("b", "5", &rules));
    assert!(matches("aab", "4 4 5", &rules));
    assert!(!matches("aa", "4", &rules));
    assert!(!matches("a", "5", &rules));
    assert!(!matches("aabb", "4 4 5", &rules));
}

#[test]
fn matches_complicated() {
    let data = read_lines("./data/day19_test.txt");
    let (rules, _) = parse_input(&data);
    assert!(matches("ab", "3", &rules));
    assert!(matches("ba", "3", &rules));
    assert!(!matches("baa", "3", &rules));
    assert!(matches("ababbb", "0", &rules));
    assert!(matches("abbbab", "0", &rules));
    assert!(!matches("bababa", "0", &rules));
    assert!(!matches("aaaabbb", "0", &rules));
}

#[test]
fn a() {
    let data = read_lines("./data/day19_test.txt");
    assert_eq!(solve_a(&data), 2);
}

#[test]
#[ignore]
fn b() {
    let data = read_lines("./data/day19_test.txt");
    assert_eq!(solve_b(&data), 0);
}
