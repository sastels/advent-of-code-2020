use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(filename);
    let lines = io::BufReader::new(file.unwrap()).lines();
    let together: Vec<String> = lines.map(|line| line.unwrap()).collect();

    together
}
