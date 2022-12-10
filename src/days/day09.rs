
use std::cmp::max;
use std::collections::HashSet;
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

pub fn solve() -> SolutionPair {
    (Solution::Usize(part_one()), Solution::Usize(part_two()))
}

fn helper(num_knots: usize) -> usize{
    let input: Vec<Vec<_>> = read_to_string("input/day09.txt").unwrap()
        .lines()
        .map(|line| line.split_whitespace()
            .map(|char| char.to_string())
            .collect::<Vec<String>>())
        .collect();

    let mut last_knot_locations: HashSet<(i32, i32)> = HashSet::new();
    let mut knot_pos: Vec<(i32, i32)> = Vec::new();

    for _ in 0..num_knots{
        knot_pos.push((0, 0));
    }

    for motion in input.iter(){
        let motion_type = &motion[0];
        let motion_amount = motion[1].parse::<i32>().unwrap();

        for _ in 0..motion_amount {
            if motion_type == "R" {
                knot_pos[0].0 += 1;
            } else if motion_type == "L" {
                knot_pos[0].0 -= 1;
            } else if motion_type == "U" {
                knot_pos[0].1 += 1;
            } else if motion_type == "D" {
                knot_pos[0].1 -= 1;
            }

            for i in 1..num_knots{

                //Update tail - check Chebyshev distance
                let x_diff: i32 = knot_pos[i - 1].0 - knot_pos[i].0;
                let y_diff: i32 = knot_pos[i - 1].1 - knot_pos[i].1;
                if max(x_diff.abs(), y_diff.abs()) > 1 {
                    if x_diff == 0 {
                        knot_pos[i].1 += y_diff / y_diff.abs();
                    } else if y_diff == 0 {
                        knot_pos[i].0 += x_diff / x_diff.abs();
                    } else {
                        knot_pos[i].0 += x_diff / x_diff.abs();
                        knot_pos[i].1 += y_diff / y_diff.abs();
                    }
                }
            }
            last_knot_locations.insert(knot_pos[num_knots-1]);
        }
    }

    return last_knot_locations.len();
}

fn part_one() -> usize {
    return helper(2);
}

fn part_two() -> usize {
    return helper(10);
}