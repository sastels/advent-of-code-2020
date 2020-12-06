use crate::utils::join_lines;
use itertools::Itertools;
use std::collections::HashMap;

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
    let groups = join_lines(data);
    let mut retval = 0;
    for group in groups {
        let num_people = group.chars().filter(|c| *c == ' ').count() + 1;
        retval += group
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
            .count();
    }
    retval
}
