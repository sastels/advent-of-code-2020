use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Clone)]
pub struct Grid<T> {
    pub size: usize,
    pub max_xy: i32,
    pub offset: i32,
    data: Vec<T>,
}

impl<T: Clone + PartialEq> Grid<T> {
    pub fn new(size: usize, fill: T) -> Self {
        let data = vec![fill; size * size];
        Grid {
            size,
            offset: 0,
            max_xy: 0,
            data,
        }
    }

    pub fn get(&self, x: i32, y: i32) -> &T {
        let loc: usize = ((y + self.offset) * self.size as i32 + (x + self.offset)) as usize;
        &(self.data[loc])
    }

    pub fn set(&mut self, x: i32, y: i32, value: T) {
        let loc: usize = ((y + self.offset) * self.size as i32 + (x + self.offset)) as usize;
        self.max_xy = max(self.max_xy, max(x.abs(), y.abs()));
        self.data[loc] = value;
    }

    pub fn count_equals(&self, value: T) -> usize {
        self.data.iter().filter(|&x| *x == value).count()
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
