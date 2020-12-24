use itertools::Itertools;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Game {
    all_cups: Vec<usize>,
    cups: Vec<usize>,
    current_index: usize,
    dest_index: usize,
    picked_up: Vec<usize>,
}

const MAX_CUP: usize = 1000000;

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cups: ")?;
        for (n, cup) in self.cups.iter().enumerate() {
            if n == self.current_index {
                write!(f, "({}) ", cup)?;
            } else {
                write!(f, "{} ", cup)?;
            }
        }
        writeln!(f, "\npick up: ")?;
        for cup in &self.picked_up {
            write!(f, "{} ", cup)?;
        }
        writeln!(f, "\ndestination: {}", self.cups[self.dest_index])?;
        Ok(())
    }
}

impl Game {
    pub fn new(data: &str) -> Self {
        let cups: Vec<usize> = data
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        Game {
            all_cups: cups.clone(),
            cups,
            current_index: 0,
            dest_index: 0,
            picked_up: vec![],
        }
    }

    pub fn pick_up_cups(&mut self) {
        self.picked_up = [
            self.current_index + 1,
            self.current_index + 1,
            self.current_index + 1,
        ]
        .iter()
        .map(|&index| {
            if index < self.cups.len() {
                self.cups.remove(index)
            } else {
                self.current_index -= 1;
                self.cups.remove(0)
            }
        })
        .collect();
    }

    pub fn set_destination(&mut self) {
        let mut destination_value = self.cups[self.current_index] - 1;
        loop {
            if let Some((dest_index, _)) = self
                .cups
                .iter()
                .enumerate()
                .find(|(_, &c)| c == destination_value)
            {
                self.dest_index = dest_index;
                break;
            } else {
                if destination_value > *self.all_cups.iter().min().unwrap() {
                    destination_value -= 1;
                } else {
                    destination_value = *self.all_cups.iter().max().unwrap();
                }
            }
        }
    }

    pub fn put_down_cups(&mut self) {
        if self.dest_index < self.current_index {
            self.current_index += self.picked_up.len();
        }
        for cup in self.picked_up.iter().rev() {
            self.cups.insert(self.dest_index + 1, *cup);
        }
        // self.picked_up = vec![];
    }

    pub fn set_current(&mut self) {
        self.current_index += 1;
        if self.current_index >= self.cups.len() {
            self.current_index = 0;
        }
    }

    pub fn get_order(&self) -> String {
        let index_1 = self
            .cups
            .iter()
            .enumerate()
            .find(|(_, &c)| c == 1)
            .unwrap()
            .0;

        let order_start: String = self.cups[index_1 + 1..]
            .iter()
            .map(|x| format!("{}", *x))
            .join("");

        let order_end: String = self.cups[..index_1]
            .iter()
            .map(|x| format!("{}", *x))
            .join("");

        format!("{}{}", order_start, order_end)
    }

    pub fn extend_cups(&mut self) {
        self.cups
            .extend((self.cups.iter().max().unwrap() + 1)..=MAX_CUP);
    }

    pub fn solve_b(&self) -> usize {
        let index_1 = self
            .cups
            .iter()
            .enumerate()
            .find(|(_, &c)| c == 1)
            .unwrap()
            .0;

        if index_1 == self.cups.len() - 1 {
            self.cups[0] * self.cups[1]
        } else if index_1 == self.cups.len() - 2 {
            self.cups[index_1 + 1] * self.cups[0]
        } else {
            self.cups[index_1 + 1] * self.cups[index_1 + 2]
        }
    }
}

pub fn solve_a(data: &str) -> String {
    let mut game = Game::new(data);

    println!("{}", game);

    for _ in 0..100 {
        game.pick_up_cups();
        game.set_destination();
        game.put_down_cups();
        // println!("-- move {} --\n{}", i + 1, game);
        game.set_current();
    }
    game.get_order()
}

pub fn solve_b(data: &str) -> usize {
    let mut game = Game::new(data);

    game.extend_cups();

    for _ in 0..100000 {
        game.pick_up_cups();
        game.set_destination();
        game.put_down_cups();
        game.set_current();
    }
    game.solve_b()
}

// 10K 23.03s
// 100K 3.8m
