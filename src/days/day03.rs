use crate::{Solution, SolutionPair};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() -> SolutionPair {
    (Solution::U32(part_one()), Solution::U32(part_two()))
}

pub fn part_one() -> u32 {
    let file = File::open("input/day03.txt").unwrap();
    let reader = BufReader::new(file).lines();
    let mut sum_priorities: u32 = 0;

    for line in reader {
        let current_line = line.unwrap();
        let (first, second) = current_line.split_at(current_line.len()/2);
        let common_char = first
            .chars()
            .filter(|c| second.contains(c.clone()))
            .next()
            .unwrap()
            .clone();

        if common_char.is_uppercase(){
            sum_priorities += ((common_char as u32) - 65) + 27;
        } else{
            sum_priorities += (common_char as u32) - 96;
        }
    }

    return sum_priorities
}

pub fn part_two() -> u32 {
    let file = File::open("input/day03.txt").unwrap();
    let reader = BufReader::new(file).lines();
    let mut sum_priorities: u32 = 0;
    let mut group: Vec<String> = Vec::new();


    for line in reader {
        let current_line = line.unwrap();
        group.push(current_line);

        if group.len() == 3{
            let common_char = group[0]
                .chars()
                .filter(|c1| group[1].contains(c1.clone()))
                .filter(|c2| group[2].contains(c2.clone()))
                .next()
                .unwrap()
                .clone();

            if common_char.is_uppercase(){
                sum_priorities += ((common_char as u32) - 65) + 27;
            } else{
                sum_priorities += (common_char as u32) - 96;
            }
            group.clear();
        }
    }

    return sum_priorities
}
