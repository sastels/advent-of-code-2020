use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl<T: Clone> Grid<T> {
    pub fn new(width: usize, height: usize, fill: T) -> Self {
        let data = vec![fill; width * height];
        Grid {
            width,
            height,
            data,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        if x >= self.width || y >= self.height {
            panic!("x or y too big")
        }
        &(self.data[y * self.width + x])
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if x >= self.width || y >= self.height {
            panic!("x or y too big")
        }
        self.data[y * self.width + x] = value;
    }
}

pub fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename);
    let lines = io::BufReader::new(file.unwrap()).lines();
    let together: Vec<String> = lines.map(|line| line.unwrap()).collect();

    together
}

// join strings together up to blank lines
pub fn join_lines(data: &[String]) -> Vec<String> {
    let mut strings: Vec<String> = vec![];
    let mut s = "".to_string();
    for line in data {
        if !line.is_empty() {
            if !s.is_empty() {
                s.push(' ');
            }
            s.push_str(line);
        } else {
            if !s.is_empty() {
                strings.push(s);
            }
            s = "".to_string();
        }
    }
    // append string in case file doesn't end in blank line
    if !s.is_empty() {
        strings.push(s);
    }
    strings
}
