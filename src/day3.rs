#[derive(Debug)]
pub struct Hill {
    pub data: Vec<String>,
}

impl Hill {
    pub fn new(data: Vec<String>) -> Self {
        Hill { data }
    }

    pub fn tree_at(&self, x: usize, y: usize) -> bool {
        let width = self.data[x].chars().count();
        let c = self.data[x].chars().nth(y % width).unwrap();
        c == '#'
    }
}

pub fn solve_a(_data: &[String]) -> usize {
    0
}

pub fn solve_b(_data: &[String]) -> usize {
    0
}
