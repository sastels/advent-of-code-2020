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
    pub dim: usize,
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
        let dim = 10;
        let mut data = BitVec::from_elem(dim * dim, false);
        for (n, bit) in cap[2].chars().map(|c| c == '#').enumerate() {
            // must be a better way
            data.set(n, bit);
        }
        let mut tile = Tile {
            id,
            data,
            dim,
            top: 0,
            bottom: 0,
            left: 0,
            right: 0,
        };
        tile.set_signature();
        tile
    }

    pub fn set_signature(&mut self) {
        let bits: Vec<bool> = (0..self.dim).map(|x| self.get_bit(x, 0)).collect();
        self.top = bits_to_usize(&bits);
        let bits: Vec<bool> = (0..self.dim).map(|x| self.get_bit(x, 9)).collect();
        self.bottom = bits_to_usize(&bits);
        let bits: Vec<bool> = (0..self.dim).map(|y| self.get_bit(0, y)).collect();
        self.left = bits_to_usize(&bits);
        let bits: Vec<bool> = (0..self.dim).map(|y| self.get_bit(9, y)).collect();
        self.right = bits_to_usize(&bits);
    }

    pub fn get_bit(&self, x: usize, y: usize) -> bool {
        self.data[self.dim * y + x]
    }

    pub fn set_bit(&mut self, x: usize, y: usize, val: bool) {
        self.data.set(self.dim * y + x, val);
    }

    pub fn rotate(&self) -> Self {
        let data = BitVec::from_elem(self.dim * self.dim, false);

        let mut tile = Tile {
            id: self.id,
            dim: self.dim,
            data,
            top: 0,
            bottom: 0,
            left: 0,
            right: 0,
        };
        for y in 0..self.dim {
            for x in 0..self.dim {
                tile.set_bit(9 - y, x, self.get_bit(x, y));
            }
        }
        tile.set_signature();
        tile
    }

    pub fn flip(&self) -> Self {
        let data = BitVec::from_elem(self.dim * self.dim, false);

        let mut tile = Tile {
            id: self.id,
            dim: self.dim,
            data,
            top: 0,
            bottom: 0,
            left: 0,
            right: 0,
        };
        for y in 0..self.dim {
            for x in 0..self.dim {
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

        for y in 0..self.dim {
            for x in 0..self.dim {
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

        let around: Vec<String> = around
            .iter()
            .map(|t| {
                if t.is_some() {
                    format!("{}", t.clone().unwrap().id)
                } else {
                    " .. ".to_string()
                }
            })
            .collect();
        // println!("     {}", around[0]);
        // println!("{} {} {}", around[2], tile.id, around[3]);
        // println!("     {}", around[1]);
        // println!("{}:  ", tile.id);

        if around.iter().filter(|x| *x != " .. ").count() == 2 {
            println!("corner: {}", tile.id);
            corner_prod *= tile.id;
        }
    }

    corner_prod
}

pub fn compute_variants(tiles: &Vec<Tile>) -> HashMap<usize, Vec<Tile>> {
    let mut tile_variants: HashMap<usize, Vec<Tile>> = HashMap::new();
    for tile in tiles {
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
    tile_variants
}

pub fn find_top_corner(tile_variants: &HashMap<usize, Vec<Tile>>) -> Tile {
    for (id, tiles) in tile_variants {
        let mut variants = tile_variants.clone();
        variants.remove(id);
        for tile in tiles {
            if find_match(tile.top, "bottom", &variants).is_none()
                && find_match(tile.left, "right", &variants).is_none()
            {
                return tile.clone();
            }
        }
    }
    panic!()
}

pub fn find_pic(variants: &HashMap<usize, Vec<Tile>>) -> Vec<Tile> {
    let mut left_side = find_top_corner(variants);
    let mut pic: Vec<Tile> = vec![left_side.clone()];

    let mut tile = left_side.clone();
    println!("{}", tile.id);
    let mut variants = variants.clone();
    variants.remove(&tile.id);

    loop {
        match find_match(tile.right, "left", &variants) {
            Some(t) => {
                println!("RRR {}", t.id);
                pic.push(t.clone());
                variants.remove(&t.id);
                tile = t.clone();
            }
            None => match find_match(left_side.bottom, "top", &variants) {
                Some(t) => {
                    println!("DDD {}", t.id);
                    pic.push(t.clone());
                    variants.remove(&t.id);
                    left_side = t;
                    tile = left_side.clone();
                }
                None => break,
            },
        }
    }
    pic
}

pub fn glue_tiles(pic: &Vec<Tile>) -> Tile {
    let tiles_per_row = (pic.len() as f64).sqrt() as usize; // per col too
    let tile_dim = pic[0].dim;

    let mut image = BitVec::new();

    println!("image size: {}", image.len());

    for pic_row in 0..tiles_per_row {
        for tile_row in 0..tile_dim {
            for tile in &pic[(pic_row * tiles_per_row)..((pic_row + 1) * tiles_per_row)] {
                for tile_col in 0..tile.dim {
                    image.push(tile.get_bit(tile_col, tile_row))
                }
            }
        }
    }

    Tile {
        id: 0,
        dim: (image.len() as f64).sqrt() as usize,
        data: image,
        top: 0,
        bottom: 0,
        left: 0,
        right: 0,
    }
}

pub fn solve_b(data: &[String]) -> usize {
    let data = join_lines(&data);
    let tiles: Vec<Tile> = data.iter().map(|s| Tile::new(s)).collect();

    let tile_variants = compute_variants(&tiles);

    let pic = find_pic(&tile_variants);

    let big_pic = glue_tiles(&pic);

    println!("{}", big_pic);

    0
}
