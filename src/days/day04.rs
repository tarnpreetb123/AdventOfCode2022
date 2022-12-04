use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

pub fn solve() -> SolutionPair {
    (Solution::U32(part_one()), Solution::U32(part_two()))
}

fn helper_vec(input: &str) -> Vec<u32> {
    let values: Vec<u32> = input.split(",")
        .map(|line| line.split("-"))
        .flatten()
        .map(|val| val.parse::<u32>().unwrap())
        .collect();

    return values;
}

fn helper_one(input: &str) -> u32 {
    let values: Vec<u32> = helper_vec(input);
    let result: bool;

    result = (values[0] >= values[2] && values[1] <= values[3])
        || (values[0] <= values[2] && values[1] >= values[3]);

    return result as u32;
}

fn helper_two(input: &str) -> u32{
    let values: Vec<u32> = helper_vec(input);
    let result: bool;

    if values[0] < values[2]{
        result = values[2] <= values[1]
    } else {
        result = values[0] <= values[3]
    }

    return result as u32;
}

pub fn part_one() -> u32{
    let total_score: u32 = read_to_string("input/day04.txt").unwrap()
        .lines()
        .map(|line| helper_one(line))
        .sum();

    return total_score;
}

pub fn part_two() -> u32{
    let total_score: u32 = read_to_string("input/day04.txt").unwrap()
        .lines()
        .map(|line| helper_two(line))
        .sum();
    return total_score
}