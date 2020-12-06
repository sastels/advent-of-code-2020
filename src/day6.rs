use crate::utils::join_lines;
use itertools::Itertools;

pub fn solve_a(data: &[String]) -> usize {
    let groups = join_lines(data);
    groups
        .iter()
        .map(|line| {
            line.chars()
                .filter(|c| *c != ' ') // join_lines substitutes spaces for newlines
                .unique()
                .count()
        })
        .sum()
}

pub fn solve_b(data: &[String]) -> usize {
    0
}
