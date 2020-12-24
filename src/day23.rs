use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Game {
    next_cup: HashMap<usize, usize>,
    prev_cup: HashMap<usize, usize>,
    all_cups: Vec<usize>,
    current: usize,
    destination: usize,
    picked_up: Vec<usize>,
}

const MAX_CUP: usize = 1000000;

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cups: ({}) ", self.current)?;
        let mut cup = self.next_cup[&self.current];
        let mut n = 1;
        while n < 10 && cup != self.current {
            write!(f, "{} ", cup)?;
            cup = self.next_cup[&cup];
            n += 1;
        }
        writeln!(f, "\npick up: ")?;
        for cup in &self.picked_up {
            write!(f, "{} ", cup)?;
        }
        writeln!(f, "\ndestination: {}", self.destination)?;
        Ok(())
    }
}

impl Game {
    pub fn new(data: &str) -> Self {
        let all_cups: Vec<usize> = data
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        let current = all_cups[0];
        let mut next_cup: HashMap<usize, usize> = HashMap::new();
        let mut prev_cup: HashMap<usize, usize> = HashMap::new();

        for n in 0..all_cups.len() - 1 {
            next_cup.insert(all_cups[n], all_cups[n + 1]);
        }
        next_cup.insert(all_cups[all_cups.len() - 1], current);
        for n in 1..all_cups.len() {
            prev_cup.insert(all_cups[n], all_cups[n - 1]);
        }
        prev_cup.insert(current, all_cups[all_cups.len() - 1]);

        Game {
            all_cups,
            current,
            destination: 0,
            next_cup,
            prev_cup,
            picked_up: vec![],
        }
    }

    pub fn insert_after(&mut self, cup: usize, new_cup: usize) {
        let after_cup = self.next_cup[&cup];
        self.next_cup.insert(new_cup, after_cup);
        self.prev_cup.insert(new_cup, cup);
        self.next_cup.insert(cup, new_cup);
        self.prev_cup.insert(after_cup, new_cup);
    }

    pub fn remove_after(&mut self, cup: usize) -> usize {
        let after_cup = self.next_cup[&cup];
        let after_after_cup = self.next_cup[&after_cup];
        self.next_cup.insert(cup, after_after_cup);
        self.prev_cup.insert(after_after_cup, cup);
        self.next_cup.remove(&after_cup);
        self.prev_cup.remove(&after_cup);
        after_cup
    }

    pub fn pick_up_cups(&mut self) {
        let cup = self.current;
        self.picked_up = (0..3).map(|_| self.remove_after(cup)).collect();
    }

    pub fn set_destination(&mut self) {
        let mut destination = self.current - 1;
        while self.next_cup.get(&destination).is_none() {
            if destination == 0 {
                destination = *self.all_cups.iter().max().unwrap();
            } else {
                destination -= 1;
            }
        }
        self.destination = destination;
    }

    pub fn put_down_cups(&mut self) {
        let picked_up = self.picked_up.clone();
        for cup in picked_up.iter().rev() {
            self.insert_after(self.destination, *cup);
        }
    }

    pub fn set_current(&mut self) {
        self.current = self.next_cup[&self.current];
    }

    pub fn solve_a(&self) -> String {
        let mut order = "".to_string();
        let mut cup = self.next_cup[&1];
        while cup != 1 {
            order = format!("{}{}", order, cup);
            cup = self.next_cup[&cup];
        }
        order
    }

    pub fn extend_cups(&mut self) {
        let mut last_cup = self.prev_cup[&self.current];
        for cup in (self.all_cups.iter().max().unwrap() + 1)..=MAX_CUP {
            self.insert_after(last_cup, cup);
            self.all_cups.push(cup);
            last_cup = cup;
        }
    }

    pub fn solve_b(&self) -> usize {
        let after_1 = self.next_cup[&1];
        let after_after_1 = self.next_cup[&after_1];
        println!("1 -> {} -> {}", after_1, after_after_1);

        println!(
            "{} -> 934001 -> {}",
            self.prev_cup[&934001], self.next_cup[&934001]
        );

        after_1 * after_after_1
    }
}

pub fn solve_a(data: &str) -> String {
    let mut game = Game::new(data);

    for _ in 0..100 {
        game.pick_up_cups();
        game.set_destination();
        game.put_down_cups();
        game.set_current();
    }
    game.solve_a()
}

pub fn solve_b(data: &str) -> usize {
    let mut game = Game::new(data);
    game.extend_cups();

    for _ in 0..10000000 {
        game.pick_up_cups();
        game.set_destination();
        game.put_down_cups();
        game.set_current();
    }
    game.solve_b()
}
