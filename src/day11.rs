use crate::utils::join_lines;
use itertools::Itertools;
use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SeatStatus {
    Floor,
    Occupied,
    Empty,
}

pub struct Seating {
    pub plan: Vec<SeatStatus>,
    pub num_rows: usize,
    pub num_cols: usize,
}

impl fmt::Display for Seating {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.num_rows {
            for j in 0..self.num_cols {
                match self.status(i, j) {
                    SeatStatus::Empty => write!(f, "L")?,
                    SeatStatus::Floor => write!(f, ".")?,
                    SeatStatus::Occupied => write!(f, "#")?,
                }
            }
            writeln!(f)?;
        }
        write!(f, "rows: {}  cols: {}", self.num_rows, self.num_cols)?;
        Ok(())
    }
}

impl Seating {
    pub fn new(data: &[String]) -> Self {
        let num_cols = data[0].len();
        let data = &join_lines(&data)[0].replace(" ", "");

        let plan: Vec<SeatStatus> = data
            .chars()
            .map(|c| match c {
                '.' => SeatStatus::Floor,
                '#' => SeatStatus::Occupied,
                'L' => SeatStatus::Empty,
                _ => panic!(),
            })
            .collect();
        Seating {
            plan,
            num_rows: data.len() / num_cols,
            num_cols,
        }
    }

    pub fn status(&self, row: usize, col: usize) -> &SeatStatus {
        &self.plan[row * self.num_cols + col]
    }

    pub fn num_occupied(&self) -> usize {
        (0..self.num_rows)
            .cartesian_product(0..self.num_cols)
            .filter(|(row, col)| *self.status(*row, *col) == SeatStatus::Occupied)
            .count()
    }

    pub fn num_occupied_neighbours(&self, row: usize, col: usize) -> usize {
        let row = row as i32;
        let col = col as i32;
        (-1..2)
            .cartesian_product(-1..2)
            .filter(|(i, j)| *i != 0 || *j != 0)
            .map(|(i, j)| (row + i, col + j))
            .filter(|(row, col)| {
                0 <= *row && *row < self.num_rows as i32 && 0 <= *col && *col < self.num_cols as i32
            })
            .filter(|(row, col)| *self.status(*row as usize, *col as usize) == SeatStatus::Occupied)
            .count()
    }

    pub fn is_occupied_dir(&self, row: usize, col: usize, dir_row: i32, dir_col: i32) -> bool {
        let mut row = row as i32 + dir_row;
        let mut col = col as i32 + dir_col;
        loop {
            if row < 0
                || row >= self.num_rows as i32
                || col < 0
                || col >= self.num_cols as i32
                || *self.status(row as usize, col as usize) == SeatStatus::Empty
            {
                return false;
            } else if *self.status(row as usize, col as usize) == SeatStatus::Occupied {
                return true;
            }
            row += dir_row;
            col += dir_col;
        }
    }

    pub fn num_occupied_visible_neighbours(&self, row: usize, col: usize) -> usize {
        (-1..2)
            .cartesian_product(-1..2)
            .filter(|(i, j)| *i != 0 || *j != 0)
            .map(|(i, j)| self.is_occupied_dir(row, col, i, j))
            .filter(|x| *x)
            .count()
    }

    // true if there was a change
    pub fn step(&mut self) -> bool {
        let mut new_plan: Vec<SeatStatus> = vec![];
        let mut something_changed = false;
        for (row, col) in (0..self.num_rows).cartesian_product(0..self.num_cols) {
            let mut new_seat = *self.status(row, col);
            match self.status(row, col) {
                SeatStatus::Empty => {
                    if self.num_occupied_neighbours(row, col) == 0 {
                        new_seat = SeatStatus::Occupied;
                        something_changed = true;
                    }
                }
                SeatStatus::Occupied => {
                    if self.num_occupied_neighbours(row, col) >= 4 {
                        new_seat = SeatStatus::Empty;
                        something_changed = true;
                    }
                }
                SeatStatus::Floor => {}
            }
            new_plan.push(new_seat);
        }
        self.plan = new_plan;
        something_changed
    }

    // true if there was a change
    pub fn step_b(&mut self) -> bool {
        let mut new_plan: Vec<SeatStatus> = vec![];
        let mut something_changed = false;
        for (row, col) in (0..self.num_rows).cartesian_product(0..self.num_cols) {
            let mut new_seat = *self.status(row, col);
            match self.status(row, col) {
                SeatStatus::Empty => {
                    if self.num_occupied_visible_neighbours(row, col) == 0 {
                        new_seat = SeatStatus::Occupied;
                        something_changed = true;
                    }
                }
                SeatStatus::Occupied => {
                    if self.num_occupied_visible_neighbours(row, col) >= 5 {
                        new_seat = SeatStatus::Empty;
                        something_changed = true;
                    }
                }
                SeatStatus::Floor => {}
            }
            new_plan.push(new_seat);
        }
        self.plan = new_plan;
        something_changed
    }
}

pub fn solve_a(data: &[String]) -> usize {
    let mut seating = Seating::new(data);
    loop {
        if !seating.step() {
            break;
        }
    }
    seating.num_occupied()
}

pub fn solve_b(data: &[String]) -> usize {
    let mut seating = Seating::new(data);
    loop {
        if !seating.step_b() {
            break;
        }
    }
    seating.num_occupied()
}
