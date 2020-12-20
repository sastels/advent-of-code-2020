use bit_vec::BitVec;
use regex::Regex;
use std::fmt;

pub fn bits_to_usize(slice: &[bool]) -> usize {
    slice.iter().fold(0, |acc, &b| acc * 2 + b as usize)
}

pub struct Tile {
    pub id: usize,
    pub data: BitVec,
    pub top: usize,
    pub bottom: usize,
    pub left: usize,
    pub right: usize,
}

impl Tile {
    pub fn new(line: &str) -> Self {
        let line = line.replace(" ", "");
        let re = Regex::new(r"Tile(\d+):(.*)").unwrap();
        let cap = re.captures(&line).unwrap();
        let id = cap[1].parse::<usize>().unwrap();

        let mut data = BitVec::from_elem(100, false);
        for (n, bit) in cap[2].chars().map(|c| c == '#').enumerate() {
            // must be a better way
            data.set(n, bit);
        }
        let mut tile = Tile {
            id,
            data,
            top: 0,
            bottom: 0,
            left: 0,
            right: 0,
        };

        let bits: Vec<bool> = (0..10).map(|x| tile.get_bit(x, 0)).collect();
        tile.top = bits_to_usize(&bits);

        let bits: Vec<bool> = (0..10).map(|x| tile.get_bit(x, 9)).collect();
        tile.bottom = bits_to_usize(&bits);

        let bits: Vec<bool> = (0..10).map(|y| tile.get_bit(0, y)).collect();
        tile.left = bits_to_usize(&bits);

        let bits: Vec<bool> = (0..10).map(|y| tile.get_bit(9, y)).collect();
        tile.right = bits_to_usize(&bits);

        tile
    }

    pub fn get_bit(&self, x: usize, y: usize) -> bool {
        self.data[10 * y + x]
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "id: {} signature: {} {} {} {}",
            self.id, self.top, self.bottom, self.left, self.right
        )?;

        for y in 0..10 {
            for x in 0..10 {
                match self.get_bit(x, y) {
                    true => write!(f, "#")?,
                    false => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn solve_a(_data: &[String]) -> usize {
    unimplemented!()
}

pub fn solve_b(_data: &[String]) -> usize {
    unimplemented!()
}
