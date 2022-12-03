
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;
use std::collections::HashMap;

pub fn solve() -> SolutionPair {
    (Solution::U32(part_one()), Solution::U32(part_two()))
}

fn helper_one(input: String) -> u32{
    let score_card = HashMap::from([
        ("AX", 1 + 3),
        ("AY", 2 + 6),
        ("AZ", 3 + 0),
        ("BX", 1 + 0),
        ("BY", 2 + 3),
        ("BZ", 3 + 6),
        ("CX", 1 + 6),
        ("CY", 2 + 0),
        ("CZ", 3 + 3)
    ]);
    return score_card.get(&*input).unwrap().clone();
}

fn helper_two(input: String) -> String{
    let transformation = HashMap::from([
        ("AX", "AZ"),
        ("AY", "AX"),
        ("AZ", "AY"),
        ("BX", "BX"),
        ("BY", "BY"),
        ("BZ", "BZ"),
        ("CX", "CY"),
        ("CY", "CZ"),
        ("CZ", "CX")
    ]);
    return transformation.get(&*input).unwrap().clone().to_string();
}

pub fn part_one() -> u32 {
    let total_score: u32 = read_to_string("input/day02.txt").unwrap()
        .lines()
        .map(|line| helper_one(line.replace(" ", "")))
        .sum();

    return total_score;
}

pub fn part_two() -> u32 {
    let total_score: u32 = read_to_string("input/day02.txt").unwrap()
        .lines()
        .map(|line| helper_two(line.replace(" ", "")))
        .map(|line| helper_one(line))
        .sum();
    return total_score;
}