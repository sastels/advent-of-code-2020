use std::collections::HashMap;

pub struct Rule {
    pub id: String,
    pub lhs: Vec<String>,
    pub rhs: Vec<String>,
}

impl Rule {
    pub fn new(line: &str) -> Self {
        let line = line.replace("\"", "");
        let mut split = line.split(": ");
        let id = split.next().unwrap().to_string();
        let rest = split.next().unwrap();
        let mut split = rest.split(" | ");
        let lhs = split
            .next()
            .unwrap()
            .to_string()
            .split(" ")
            .map(|s| s.to_string())
            .collect();
        let mut rhs: Vec<String> = vec![];
        let rhs_str = split.next();
        if rhs_str.is_some() {
            rhs = rhs_str
                .unwrap()
                .to_string()
                .split(" ")
                .map(|s| s.to_string())
                .collect();
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

pub fn solve_a(_data: &[String]) -> usize {
    unimplemented!()
}

pub fn solve_b(_data: &[String]) -> usize {
    unimplemented!()
}
