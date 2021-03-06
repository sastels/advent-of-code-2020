use regex::Regex;

pub struct Password {
    pub min: usize,
    pub max: usize,
    pub letter: char,
    pub password: String,
}

impl Password {
    pub fn new(line: &str) -> Self {
        let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
        let cap = re.captures(line).unwrap();
        Password {
            min: cap[1].parse::<usize>().unwrap(),
            max: cap[2].parse::<usize>().unwrap(),
            letter: cap[3].parse::<char>().unwrap(),
            password: String::from(&cap[4]),
        }
    }

    pub fn passes_a(&self) -> bool {
        let num_letters_in_password = self.password.chars().filter(|c| *c == self.letter).count();
        self.min <= num_letters_in_password && num_letters_in_password <= self.max
    }

    pub fn passes_b(&self) -> bool {
        let letter1 = self.password.as_bytes()[self.min - 1] as char;
        let letter2 = self.password.as_bytes()[self.max - 1] as char;
        (letter1 == self.letter && letter2 != self.letter)
            || (letter1 != self.letter && letter2 == self.letter)
    }
}

pub fn parse_many(line: &str) -> Vec<Password> {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    re.captures_iter(line)
        .map(|cap| Password {
            min: cap[1].parse::<usize>().unwrap(),
            max: cap[2].parse::<usize>().unwrap(),
            letter: cap[3].parse::<char>().unwrap(),
            password: String::from(&cap[4]),
        })
        .collect()
}

pub fn solve_a(data: &[String]) -> usize {
    let data = data.join(" ");
    let data = parse_many(&data);
    data.iter().filter(|p| p.passes_a()).count()
}

pub fn solve_b(data: &[String]) -> usize {
    let data = data.join(" ");
    let data = parse_many(&data);
    data.iter().filter(|p| p.passes_b()).count()
}
