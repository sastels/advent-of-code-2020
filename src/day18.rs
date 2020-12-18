use regex::Regex;

pub fn calc(line: &str) -> usize {
    let mut eqn = line.to_string();

    // calculate parentheses
    let re = Regex::new(r"(.*)\(([\d\w \+\*]+)\)(.*)").unwrap();
    loop {
        let cap = re.captures(&eqn);
        match cap {
            None => break,
            Some(cap) => {
                let val = calc(&cap[2]);
                eqn = format!("{}{}{}", cap[1].to_string(), val, cap[3].to_string());
            }
        }
    }

    // calculate adds - just for part b
    let re = Regex::new(r"^(.*?)(\d+) \+ (\d+)(.*?)$").unwrap();
    loop {
        let cap = re.captures(&eqn);
        match cap {
            None => break,
            Some(cap) => {
                let left_side = cap[1].to_string();
                let left_dig = cap[2].parse::<usize>().unwrap();
                let right_dig = cap[3].parse::<usize>().unwrap();
                let right_side = cap[4].to_string();
                eqn = format!("{}{}{}", left_side, left_dig + right_dig, right_side);
            }
        }
    }

    // now there's just multiplies
    let mut value = 0;
    let mut operation = "+";

    for element in eqn.split(" ") {
        match element.parse::<usize>() {
            Err(_) => operation = element,
            Ok(num) => {
                if operation == "+" {
                    value += num;
                } else {
                    value *= num;
                }
            }
        }
    }
    value
}

pub fn solve_a(data: &[String]) -> usize {
    data.iter().map(|s| calc(s)).sum()
}

pub fn solve_b(data: &[String]) -> usize {
    data.iter().map(|s| calc(s)).sum()
}
