use crate::utils::join_lines;
use itertools::Itertools;

pub fn solve_a(data: &[String]) -> usize {
    join_lines(data)
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
    join_lines(data)
        .iter()
        .map(|group| {
            let num_people = group.chars().filter(|c| *c == ' ').count() + 1;
            group
                .chars()
                .filter(|c| *c != ' ') // join_lines substitutes spaces for newlines
                .sorted()
                .map(|c| (c, 1))
                .coalesce(|(c, n), (d, m)| {
                    if c == d {
                        Ok((c, n + m))
                    } else {
                        Err(((c, n), (d, m)))
                    }
                })
                .map(|(_c, n)| n)
                .filter(|n| *n == num_people)
                .count()
        })
        .sum()
}
