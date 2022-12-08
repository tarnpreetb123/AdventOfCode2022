use std::fs::read_to_string;
use crate::{Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    (Solution::Str(part_one()), Solution::Str(part_two()))
}

fn helper(part_two: bool) -> String{
    let input = read_to_string("input/day05.txt").unwrap();
    let section: Vec<_> = input.split("\n\n").collect();
    let section_one: Vec<_> = section[0].lines().collect();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    let last_row = section_one.last().unwrap().to_string();
    let last_row_split: Vec<_> = last_row.split_whitespace().collect();
    let last_index = last_row_split.last().unwrap().parse::<u32>().unwrap();

    for _ in 0..(last_index){
        stacks.push(Vec::new());
    }

    for i in (0..section_one.len() - 1).rev(){
        let line: Vec<_> = section_one[i].chars().collect();
        let mut place = 0;
        for stack in line.chunks(4){
            if stack[1] != ' ' {
                stacks[place].push(stack[1]);
            }
            place += 1;
        }
    }

    section[1].lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .step_by(2)
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .for_each(|line| {
            let mut temp_stack = Vec::new();
            for _ in 0..line[0]{
                if stacks[line[1]-1].len() > 0 {
                    let item = stacks[line[1] - 1].pop().unwrap();
                    temp_stack.push(item);
                }
            }

            if part_two{
                temp_stack.reverse();
            }

            for item in temp_stack {
                stacks[line[2]-1].push(item);
            }
        });

    let mut result: String = String::from("");

    for mut stack in stacks.clone(){
        if stack.len() > 0{
            result.push(stack.pop().unwrap());
        }
    }
    return result;
}

fn part_one() -> String{
    return helper(false);
}

fn part_two() -> String{
    return helper(true);
}