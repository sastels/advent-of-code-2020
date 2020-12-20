use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;

pub fn find_first_diff_pos(a: &str, b: &str) -> Option<usize> {
    if a == b {
        return None;
    }
    for (n, (ac, bc)) in a.chars().zip(b.chars()).enumerate() {
        if ac != bc {
            if bc == 'a' || bc == 'b' {
                return Some(666); // this will force a mismatch in our case
            } else {
                return Some(n);
            }
        }
    }
    if a.len() != b.len() {
        return Some(cmp::min(a.len(), b.len()));
    } else {
        return None;
    }
}

pub struct Rule {
    pub id: String,
    pub lhs: String,
    pub rhs: String,
}

impl Rule {
    pub fn new(line: &str) -> Self {
        let line = line.replace("\"", "");
        let mut split = line.split(": ");
        let id = split.next().unwrap().to_string();
        let rest = split.next().unwrap();
        let mut split = rest.split(" | ");
        let lhs = split.next().unwrap().to_string();
        let mut rhs = "".to_string();
        let rhs_str = split.next();
        if rhs_str.is_some() {
            rhs = rhs_str.unwrap().to_string();
        }
        Rule { id, lhs, rhs }
    }
}

pub fn parse_input(data: &[String]) -> (HashMap<String, Rule>, Vec<String>) {
    let mut rules: HashMap<String, Rule> = HashMap::new();
    let mut messages: Vec<String> = vec![];
    let mut parsing_now = "rules";
    for line in data {
        if line.is_empty() {
            parsing_now = "messages";
            continue;
        }
        if parsing_now == "rules" {
            let rule = Rule::new(line);
            rules.insert(rule.id.clone(), rule);
        } else {
            messages.push(line.to_string());
        }
    }
    (rules, messages)
}

pub fn matches(message: &str, rule: &str, rules: &HashMap<String, Rule>) -> bool {
    let message = message.replace(" ", "").chars().join(" "); // make sure spaced out

    // rule is just a's and b's
    if rule
        .replace("a", "")
        .replace("b", "")
        .replace(" ", "")
        .is_empty()
    {
        return message.replace(" ", "") == rule.replace(" ", "");
    }

    // check ab prefix of rule
    let first_diff = find_first_diff_pos(&message, rule);

    if first_diff.is_none() {
        return true; // 100% match
    } else if first_diff.unwrap() == 666
        || first_diff.unwrap() == message.len()
        || first_diff.unwrap() == rule.len()
    {
        return false; // rule or message has extra stuff
    }

    let message = message[first_diff.unwrap()..].to_string();
    let rule = rule[first_diff.unwrap()..].to_string();

    // have a number we need to replace
    let rule_id = rule
        .split(" ")
        .filter(|s| !s.is_empty() && *s != "a" && *s != "b")
        .next()
        .unwrap();
    let sub_rule = &rules[rule_id];
    if sub_rule.rhs.is_empty() {
        let mut replaced = false;
        let rule = rule
            .split(" ")
            .map(|s| {
                if s == rule_id && !replaced {
                    replaced = true;
                    sub_rule.lhs.clone()
                } else {
                    s.to_string()
                }
            })
            .join(" ");

        return matches(&message, &rule, rules);
    } else {
        let mut replaced = false;
        let rule1 = rule
            .split(" ")
            .map(|s| {
                if s == rule_id && !replaced {
                    replaced = true;
                    sub_rule.lhs.clone()
                } else {
                    s.to_string()
                }
            })
            .join(" ");
        replaced = false;
        let rule2 = rule
            .split(" ")
            .map(|s| {
                if s == rule_id && !replaced {
                    replaced = true;
                    sub_rule.rhs.clone()
                } else {
                    s.to_string()
                }
            })
            .join(" ");
        return matches(&message, &rule1, rules) || matches(&message, &rule2, rules);
    }
}

pub fn solve_a(data: &[String]) -> usize {
    let (rules, messages) = parse_input(&data);
    messages
        .iter()
        .enumerate()
        .inspect(|(n, _)| println!("{} / {}", n, messages.len()))
        .map(|(_, m)| m)
        .map(|m| matches(m, "0", &rules))
        .filter(|x| *x)
        .count()
}

pub fn solve_b(data: &[String]) -> usize {
    let (rules, messages) = parse_input(&data);
    messages
        .iter()
        .enumerate()
        .inspect(|(n, _)| println!("{} / {}", n, messages.len()))
        .map(|(_, m)| m)
        .map(|m| matches(m, "0", &rules))
        .filter(|x| *x)
        .count()
}
