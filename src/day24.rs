use crate::utils::Grid;

#[derive(Clone)]
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
                let pt = HexGrid::move_once(x, y, &direction);
                x = pt.0;
                y = pt.1;
                direction = "".to_string();
            }
        }
        (x, y)
    }

    pub fn num_true_neighbours(&self, x: i32, y: i32) -> usize {
        [(1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1), (1, -1)]
            .iter()
            .filter(|(xo, yo)| *self.coords.get(x + xo, y + yo))
            .count()
    }

    pub fn daily_change(&mut self) {
        let mut new_grid = self.clone();
        for x in -(self.coords.max_xy + 2)..(self.coords.max_xy + 2) {
            for y in -(self.coords.max_xy + 2)..(self.coords.max_xy + 2) {
                let &current = self.coords.get(x, y);
                let nbrs = self.num_true_neighbours(x, y);
                if current == true && (nbrs == 0 || nbrs > 2) {
                    new_grid.flip(x, y);
                } else if current == false && (nbrs == 2) {
                    new_grid.flip(x, y);
                }
            }
        }
        self.coords = new_grid.coords;
    }
}

pub fn solve_a(data: &[String]) -> usize {
    let mut grid = HexGrid::new(100, 50);
    for moves in data {
        let (x, y) = HexGrid::move_many(0, 0, moves);
        grid.flip(x, y);
    }
    grid.coords.count_equals(true)
}

pub fn solve_b(data: &[String]) -> usize {
    let mut grid = HexGrid::new(400, 200);

    // initial setup
    for moves in data {
        let (x, y) = HexGrid::move_many(0, 0, moves);
        grid.flip(x, y);
    }

    // run for 100 days
    for _ in 1..=100 {
        grid.daily_change();
    }
    grid.coords.count_equals(true)
}
