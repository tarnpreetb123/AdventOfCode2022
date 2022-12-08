use std::collections::HashSet;
use std::fs::read_to_string;

use crate::{Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    (Solution::Usize(part_one()), Solution::Usize(part_two()))
}

fn helper(window_size: usize) -> usize{
        let input: Vec<_> = read_to_string("input/day06.txt").unwrap()
        .lines()
        .map(|line| line.chars())
        .flatten()
        .collect();

    let decode = input
        .windows(window_size)
        .map(|line| HashSet::<String>::from_iter(line.iter()
                .map(|char| char.to_string())
                .clone())
            .len())
        .position(|x| x == window_size)
        .unwrap();

    let index = decode + window_size;
    return index
}
fn part_one() -> usize{
    return helper(4);
}

fn part_two() -> usize{
    return helper(14);
}