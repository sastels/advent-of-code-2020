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
