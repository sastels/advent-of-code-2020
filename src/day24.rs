use crate::utils::Grid;

pub fn move_once(x: i32, y: i32, direction: &str) -> (i32, i32) {
    match direction {
        "e" => (x + 1, y),
        "se" => (x, y + 1),
        "sw" => (x - 1, y + 1),
        "w" => (x - 1, y),
        "nw" => (x, y - 1),
        "ne" => (x + 1, y - 1),
        _ => panic!("Unknown direction!"),
    }
}

pub fn move_many(x_start: i32, y_start: i32, moves: &str) -> (i32, i32) {
    let mut direction = "".to_string();
    let mut x = x_start;
    let mut y = y_start;
    for c in moves.chars() {
        direction = format!("{}{}", direction, c);
        if c == 'e' || c == 'w' {
            let pt = move_once(x, y, &direction);
            x = pt.0;
            y = pt.1;
            direction = "".to_string();
        }
    }
    (x, y)
}

pub struct HexGrid {
    pub coords: Grid<bool>,
}

impl HexGrid {
    pub fn new(size: usize, offset: i32) -> Self {
        let mut coords = Grid::new(size, false);
        coords.offset = offset;
        HexGrid { coords }
    }

    pub fn flip(&mut self, x: i32, y: i32) {
        self.coords.set(x, y, !*self.coords.get(x, y));
    }
}

pub fn solve_a(data: &[String]) -> usize {
    let mut grid = HexGrid::new(100, 50);
    for moves in data {
        let (x, y) = move_many(0, 0, moves);
        grid.flip(x, y);
    }
    grid.coords.count_equals(true)
}

pub fn solve_b(_data: &[String]) -> usize {
    unimplemented!()
}
