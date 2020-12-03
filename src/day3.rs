#[derive(Debug)]
pub struct Hill {
    pub data: Vec<String>,
}

impl Hill {
    pub fn new(data: Vec<String>) -> Self {
        Hill { data }
    }

    pub fn tree_at(&self, row: usize, col: usize) -> bool {
        let width = self.data[row].chars().count();
        let c = self.data[row].chars().nth(col % width).unwrap();
        c == '#'
    }

    pub fn slide_down(&self, row_offset: usize, col_offset: usize) -> usize {
        let mut col = 0;
        let mut num_trees = 0;

        for row in (0..self.data.len()).step_by(row_offset) {
            if self.tree_at(row, col) {
                num_trees += 1;
            }
            col += col_offset;
        }
        num_trees
    }
}

pub fn solve_a(data: &[String]) -> usize {
    let hill = Hill::new(Vec::from(data));
    hill.slide_down(1, 3)
}

pub fn solve_b(data: &[String]) -> usize {
    let hill = Hill::new(Vec::from(data));
    hill.slide_down(1, 1)
        * hill.slide_down(1, 3)
        * hill.slide_down(1, 5)
        * hill.slide_down(1, 7)
        * hill.slide_down(2, 1)
}
