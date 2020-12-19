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

pub fn solve_a(_data: &[String]) -> usize {
    unimplemented!()
}

pub fn solve_b(_data: &[String]) -> usize {
    unimplemented!()
}
