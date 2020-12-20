use crate::utils::join_lines;
use bit_vec::BitVec;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;

pub fn bits_to_usize(slice: &[bool]) -> usize {
    slice.iter().fold(0, |acc, &b| acc * 2 + b as usize)
}

#[derive(Clone, Debug)]
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
        tile.set_signature();
        tile
    }

    pub fn set_signature(&mut self) {
        let bits: Vec<bool> = (0..10).map(|x| self.get_bit(x, 0)).collect();
        self.top = bits_to_usize(&bits);
        let bits: Vec<bool> = (0..10).map(|x| self.get_bit(x, 9)).collect();
        self.bottom = bits_to_usize(&bits);
        let bits: Vec<bool> = (0..10).map(|y| self.get_bit(0, y)).collect();
        self.left = bits_to_usize(&bits);
        let bits: Vec<bool> = (0..10).map(|y| self.get_bit(9, y)).collect();
        self.right = bits_to_usize(&bits);
    }

    pub fn get_bit(&self, x: usize, y: usize) -> bool {
        self.data[10 * y + x]
    }

    pub fn set_bit(&mut self, x: usize, y: usize, val: bool) {
        self.data.set(10 * y + x, val);
    }

    pub fn rotate(&self) -> Self {
        let data = BitVec::from_elem(100, false);

        let mut tile = Tile {
            id: self.id,
            data,
            top: 0,
            bottom: 0,
            left: 0,
            right: 0,
        };
        for y in 0..10 {
            for x in 0..10 {
                tile.set_bit(9 - y, x, self.get_bit(x, y));
            }
        }
        tile.set_signature();
        tile
    }

    pub fn flip(&self) -> Self {
        let data = BitVec::from_elem(100, false);

        let mut tile = Tile {
            id: self.id,
            data,
            top: 0,
            bottom: 0,
            left: 0,
            right: 0,
        };
        for y in 0..10 {
            for x in 0..10 {
                tile.set_bit(y, x, self.get_bit(x, y));
            }
        }
        tile.set_signature();
        tile
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

pub fn find_match(sig: usize, pos: &str, variants: &HashMap<usize, Vec<Tile>>) -> Option<Tile> {
    for (_, tiles) in variants {
        for tile in tiles {
            if (pos == "top" && sig == tile.top)
                || (pos == "bottom" && sig == tile.bottom)
                || (pos == "left" && sig == tile.left)
                || (pos == "right" && sig == tile.right)
            {
                return Some(tile.clone());
            }
        }
    }
    None
}

pub fn solve_a(data: &[String]) -> usize {
    let data = join_lines(&data);
    let tiles: Vec<Tile> = data.iter().map(|s| Tile::new(s)).collect();
    let mut tile_variants: HashMap<usize, Vec<Tile>> = HashMap::new();

    for tile in &tiles {
        let mut variants: Vec<Tile> = vec![];
        variants.push(tile.rotate()); // 0
        variants.push(variants[0].rotate()); // 1
        variants.push(variants[1].rotate()); // 2
        variants.push(variants[2].rotate()); // 3
        variants.push(tile.flip()); // 4
        variants.push(variants[4].rotate()); // 5
        variants.push(variants[5].rotate()); // 6
        variants.push(variants[6].rotate()); // 7
        tile_variants.insert(tile.id, variants);
    }

    let mut corner_prod = 1;
    for tile in &tiles {
        let mut variants = tile_variants.clone();
        variants.remove(&tile.id);
        let mut around: Vec<Option<Tile>> = vec![];

        around.push(find_match(tile.top, "bottom", &variants));
        around.push(find_match(tile.bottom, "top", &variants));
        around.push(find_match(tile.left, "right", &variants));
        around.push(find_match(tile.right, "left", &variants));

        // println!("{}", tile.id);
        // for x in &around {
        //     if x.is_some() {
        //         print!("{}  ", x.clone().unwrap().id);
        //     }
        // }
        // println!();

        if around.iter().filter(|x| x.is_some()).count() == 2 {
            println!("corner: {}", tile.id);
            corner_prod *= tile.id;
        }
    }

    corner_prod
}

pub fn solve_b(_data: &[String]) -> usize {
    unimplemented!()
}
