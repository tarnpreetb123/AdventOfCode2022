use std::fs::read_to_string;
use crate::{Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    (Solution::Usize(part_one()), Solution::Usize(part_two()))
}

fn part_one() -> usize{
    let input: Vec<Vec<u32>> = read_to_string("input/day08.txt").unwrap()
        .lines()
        .map(|line| line.chars().map(|char| char.to_digit(10).unwrap()).collect())
        .collect();

    let mut count_visible = 0;

    for (vert, row) in input.iter().enumerate(){
        for (hori, value) in row.iter().enumerate(){
            if vert != 0 && vert != input.len()-1 && hori != 0 && hori != row.len()-1{
                let mut visibility = [true, true, true, true];
                //Check above
                for y in 0..vert {
                    if input[y][hori] >= *value{
                        visibility[0] = false;
                    }
                }

                //Check below
                for y in vert+1..input.len(){
                    if input[y][hori] >= *value{
                        visibility[1] = false;
                    }
                }

                //Check left
                for x in 0..hori{
                    if input[vert][x] >= *value{
                        visibility[2] = false;
                    }
                }

                //Check right
                for x in hori+1..row.len(){
                    if input[vert][x] >= *value{
                        visibility[3] = false;
                    }
                }
                if visibility[0] || visibility[1] || visibility[2] || visibility[3] {
                    count_visible += 1;
                }
            }
        }
    }

    //Count edge pieces
    let edge_visiability = 2*(input.len()-2) + 2*input[0].len();
    count_visible += edge_visiability;

    return count_visible
}

fn part_two() -> usize{
    let input: Vec<Vec<u32>> = read_to_string("input/day08.txt").unwrap()
        .lines()
        .map(|line| line.chars().map(|char| char.to_digit(10).unwrap()).collect())
        .collect();

    let mut count_visible = 0;

    for (vert, row) in input.iter().enumerate(){
        for (hori, value) in row.iter().enumerate(){
            if vert != 0 && vert != input.len()-1 && hori != 0 && hori != row.len()-1{

                let mut num_trees_seen = [0, 0, 0, 0];
                //Check above
                for y in (0..vert).rev() {
                    if input[y][hori] >= *value{
                        num_trees_seen[0] += 1;
                        break;
                    }
                    num_trees_seen[0] += 1;
                }

                //Check below
                for y in vert+1..input.len(){
                    if input[y][hori] >= *value{
                        num_trees_seen[1] += 1;
                        break;
                    }
                    num_trees_seen[1] += 1;
                }

                //Check left
                for x in (0..hori).rev(){
                    if input[vert][x] >= *value{
                        num_trees_seen[2]+=1;
                        break;
                    }
                    num_trees_seen[2] += 1;
                }

                //Check right
                for x in hori+1..row.len(){
                    if input[vert][x] >= *value{
                        num_trees_seen[3] +=1;
                        break;
                    }
                    num_trees_seen[3] += 1;
                }

                let score = num_trees_seen[0] * num_trees_seen[1] * num_trees_seen[2] * num_trees_seen[3];
                if score > count_visible {
                    count_visible = score;
                }
            }
        }
    }

    return count_visible
}