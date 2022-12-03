use crate::{Solution, SolutionPair};

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() -> SolutionPair{
    return (Solution::U32(part_one()), Solution::U32(part_two()))
}

pub fn part_one() -> u32 {
    let file = File::open("input/day01.txt").unwrap();
    let reader = BufReader::new(file).lines();
    let mut elf_calories = Vec::new();

    let mut current_elf_calories: u32 = 0;

    for line in reader {
        let current_line = line.unwrap();
        if current_line.is_empty() {
            elf_calories.push(current_elf_calories);
            current_elf_calories = 0;
        } else {
            current_elf_calories += current_line.parse::<u32>().unwrap();
        }
    }

    let max_calories = elf_calories.iter().max().unwrap().clone();

    return max_calories;
}

pub fn part_two() -> u32{
    let file = File::open("input/day01.txt").unwrap();
    let reader = BufReader::new(file).lines();
    let mut elf_calories = Vec::new();

    let mut current_elf_calories: u32 = 0;

    for line in reader {
        let current_line = line.unwrap();
        if current_line.is_empty() {
            elf_calories.push(current_elf_calories);
            current_elf_calories = 0;
        } else {
            current_elf_calories += current_line.parse::<u32>().unwrap();
        }
    }

    elf_calories.sort();
    let mut max_calories = 0;

    for _ in 0..3{
        max_calories +=  elf_calories.pop().unwrap().clone();
    }

    return max_calories;
}
