use std::fmt;

const SIZE: usize = 2 * 15 + 1;

#[derive(Clone)]
pub struct Conway {
    space: Vec<Vec<Vec<Vec<bool>>>>,
    offset: i32,
    max_x_offset: i32,
    max_y_offset: i32,
    max_z_offset: i32,
    max_w_offset: i32,
}

impl fmt::Display for Conway {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "offsets x y z w {} {} {} {}",
            self.max_x_offset, self.max_y_offset, self.max_z_offset, self.max_w_offset
        )?;
        for w in -(self.max_w_offset as i32)..=(self.max_w_offset as i32) {
            for z in -(self.max_z_offset as i32)..=(self.max_z_offset as i32) {
                writeln!(f, "\nz = {} w= {}\n", z, w)?;
                for y in -(self.max_y_offset as i32)..=(self.max_y_offset as i32) {
                    for x in -(self.max_x_offset as i32)..=(self.max_x_offset as i32) {
                        if self.cube_is_set(x, y, z, w) {
                            write!(f, "#")?
                        } else {
                            write!(f, ".")?
                        }
                    }
                    writeln!(f)?;
                }
            }
        }
        Ok(())
    }
}

impl Conway {
    pub fn new(data: &[String]) -> Self {
        let space = vec![vec![vec![vec![false; SIZE]; SIZE]; SIZE]; SIZE];
        let offset = ((SIZE - 1) / 2) as i32;
        let mut conway = Conway {
            space,
            offset,
            max_x_offset: 0,
            max_y_offset: 0,
            max_z_offset: 0,
            max_w_offset: 0,
        };
        let data_offset = ((data.len() - 1) / 2) as i32;
        for (y, row) in data.iter().enumerate() {
            for (x, col) in row.chars().enumerate() {
                if col == '#' {
                    conway.set_cube(x as i32 - data_offset, y as i32 - data_offset, 0, 0, true);
                }
            }
        }
        conway
    }

    pub fn set_cube(&mut self, x: i32, y: i32, z: i32, w: i32, value: bool) {
        if x.abs() > self.max_x_offset {
            self.max_x_offset = x.abs();
        }
        if y.abs() > self.max_y_offset {
            self.max_y_offset = y.abs();
        }
        if z.abs() > self.max_z_offset {
            self.max_z_offset = z.abs();
        }
        if w.abs() > self.max_w_offset {
            self.max_w_offset = w.abs();
        }

        self.space[(self.offset + x) as usize][(self.offset + y) as usize]
            [(self.offset + z) as usize][(self.offset + w) as usize] = value;
    }

    pub fn cube_is_set(&self, x: i32, y: i32, z: i32, w: i32) -> bool {
        self.space[(self.offset + x) as usize][(self.offset + y) as usize]
            [(self.offset + z) as usize][(self.offset + w) as usize]
    }

    pub fn num_neighbours_set(&self, x: i32, y: i32, z: i32, w: i32) -> usize {
        let mut num_set = 0;

        for ox in -1..=1 {
            for oy in -1..=1 {
                for oz in -1..=1 {
                    for ow in -1..=1 {
                        if self.cube_is_set(x + ox, y + oy, z + oz, w + ow) {
                            num_set += 1;
                        }
                    }
                }
            }
        }
        if self.cube_is_set(x, y, z, w) {
            num_set = num_set - 1;
        }
        num_set
    }

    pub fn step(&mut self) {
        let orig = self.clone();
        for x in -(self.max_x_offset + 1)..=(self.max_x_offset + 1) {
            for y in -(self.max_y_offset + 1)..=(self.max_y_offset + 1) {
                for z in -(self.max_z_offset + 1)..=(self.max_z_offset + 1) {
                    for w in -(self.max_w_offset + 1)..=(self.max_w_offset + 1) {
                        match orig.cube_is_set(x, y, z, w) {
                            true => {
                                let nn = orig.num_neighbours_set(x, y, z, w);
                                if nn != 2 && nn != 3 {
                                    self.set_cube(x, y, z, w, false);
                                }
                            }
                            false => {
                                if orig.num_neighbours_set(x, y, z, w) == 3 {
                                    self.set_cube(x, y, z, w, true);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn count_active(&self) -> usize {
        let mut num_active = 0;
        for x in -(self.max_x_offset + 1)..=(self.max_x_offset + 1) {
            for y in -(self.max_y_offset + 1)..=(self.max_y_offset + 1) {
                for z in -(self.max_z_offset + 1)..=(self.max_z_offset + 1) {
                    for w in -(self.max_w_offset + 1)..=(self.max_w_offset + 1) {
                        if self.cube_is_set(x, y, z, w) {
                            num_active += 1;
                        }
                    }
                }
            }
        }
        num_active
    }
}

pub fn solve_a(_data: &[String]) -> usize {
    0
}

pub fn solve_b(data: &[String]) -> usize {
    let mut conway = Conway::new(data);
    println!("Conway{}", conway);
    conway.step();
    conway.step();
    conway.step();
    conway.step();
    conway.step();
    conway.step();
    conway.count_active()
}
