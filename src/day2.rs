use regex::Regex;

#[derive(Debug)]
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

pub fn solve_a(data: &[String]) {
    let data: Vec<Password> = data.iter().map(|line| Password::new(line)).collect();
    let num_passing = data.iter().filter(|p| p.passes_a()).count();
    println!("Day 2a: {:?}", num_passing);
}

pub fn solve_b(data: &[String]) {
    let data: Vec<Password> = data.iter().map(|line| Password::new(line)).collect();
    let num_passing = data.iter().filter(|p| p.passes_b()).count();
    println!("Day 2b: {:?}", num_passing);
}
