use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct Rule {
    field: String,
    range0: RangeInclusive<usize>,
    range1: RangeInclusive<usize>,
}

pub fn parse_range(s: &str) -> RangeInclusive<usize> {
    let parts: Vec<usize> = s.split('-').map(|s| s.parse().unwrap()).collect();
    parts[0]..=parts[1]
}

impl Rule {
    pub fn new(data: &str) -> Self {
        let parts: Vec<&str> = data.split(": ").collect();
        let field = parts[0].to_string();
        let ranges: Vec<&str> = parts[1].split(" or ").collect();

        Rule {
            field,
            range0: parse_range(ranges[0]),
            range1: parse_range(ranges[1]),
        }
    }

    pub fn is_valid(&self, n: usize) -> bool {
        self.range0.contains(&n) || self.range1.contains(&n)
    }
}

#[derive(Debug)]
pub struct Input {
    rules: Vec<Rule>,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

impl Input {
    pub fn new(data: &[String]) -> Self {
        let mut rules: Vec<Rule> = vec![];
        let mut my_ticket: Vec<usize> = vec![];
        let mut nearby_tickets: Vec<Vec<usize>> = vec![];

        let mut parsing_now = "rules";
        for line in data {
            if line.contains("your ticket") {
                parsing_now = "your ticket";
            } else if line.contains("nearby tickets") {
                parsing_now = "nearby tickets"
            } else if !line.is_empty() {
                match parsing_now {
                    "rules" => rules.push(Rule::new(line)),
                    "your ticket" => {
                        my_ticket = line.split(',').map(|s| s.parse().unwrap()).collect()
                    }
                    "nearby tickets" => {
                        nearby_tickets.push(line.split(',').map(|s| s.parse().unwrap()).collect())
                    }
                    _ => panic!(),
                }
            }
        }

        Input {
            rules,
            my_ticket,
            nearby_tickets,
        }
    }
}

pub fn is_valid_ticket(rules: &[Rule], ticket: &[usize]) -> bool {
    for n in ticket {
        if rules.iter().all(|r| !r.is_valid(*n)) {
            return false;
        }
    }
    true
}

pub fn solve_a(data: &[String]) -> usize {
    let input = Input::new(data);
    let mut error_rate = 0;
    for ticket in input.nearby_tickets {
        for n in ticket {
            if input.rules.iter().all(|r| !r.is_valid(n)) {
                error_rate += n;
                break;
            }
        }
    }
    error_rate
}

pub fn solve_b(data: &[String]) -> usize {
    let mut input = Input::new(data);

    let mut nearby_tickets = input.nearby_tickets.clone();
    nearby_tickets.retain(|t| is_valid_ticket(&(input.rules), t));
    input.nearby_tickets = nearby_tickets;

    let mut field_order: Vec<Vec<Rule>> = (0..input.my_ticket.len())
        .map(|_| input.rules.clone())
        .collect();

    for ticket in input.nearby_tickets {
        for (index, n) in ticket.iter().enumerate() {
            field_order[index].retain(|r| r.is_valid(*n));
        }
    }

    // this is painful :/
    loop {
        let mut new_field_order = field_order.clone();
        let mut changed = false;
        for (index, rules) in field_order.iter().enumerate() {
            if rules.len() == 1 {
                let name = &rules[0].field;
                for new_index in 0..field_order.len() {
                    if new_index != index {
                        new_field_order[new_index].retain(|r| r.field != name.to_string());
                    }
                    if new_field_order[new_index].len() != field_order[new_index].len() {
                        changed = true;
                    }
                }
            }
        }
        field_order = new_field_order;

        if !changed {
            break;
        }
    }

    input
        .my_ticket
        .iter()
        .zip(field_order)
        .filter(|(_, rules)| rules[0].field.contains("eparture"))
        .map(|(n, _)| n)
        .product()
}
