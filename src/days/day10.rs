use std::collections::HashMap;
use std::fs::read_to_string;
use crate::{Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    (Solution::I32(part_one()), Solution::Str(part_two()))
}

fn save_register(cycle: i32, register: i32, register_map: &mut HashMap<i32, i32>) {
    if cycle%20 == 0 {
        register_map.insert(cycle, register);
    }
}

fn part_one() -> i32{

    let mut register: i32 = 1;
    let mut register_map = HashMap::new();
    let mut cycle: i32 = 0;

    read_to_string("input/day10.txt").unwrap()
        .lines()
        .for_each(|line| {

            let vars: Vec<String> = line.split_whitespace()
                .map(|char| char.to_string())
                .collect();

            if vars[0] == "noop"{
                cycle += 1;
                save_register(cycle, register, &mut register_map);
            }

            if vars[0] == "addx"{
                cycle += 1;
                save_register(cycle, register, &mut register_map);
                cycle += 1;
                save_register(cycle, register, &mut register_map);
                register += vars[1].parse::<i32>().unwrap();
            }
        });

    let mut signal_strength = 0;

    register_map.iter().for_each(|(k, v)| {
        if k == &20 || k == &60 || k == &100 || k == &140 || k == &180 || k == &220 {
            signal_strength += k * v;
        }
    });

    return signal_strength;
}

fn save_row(cycle: i32, register: i32, current_pixel: &mut i32, current_row: &mut String, screen: &mut Vec<String>){
    if register - 1 == *current_pixel || register == *current_pixel || register + 1 == *current_pixel {
        current_row.push('#');
    } else {
        current_row.push('.');
    }
    *current_pixel += 1;
    if cycle%40 == 0{
        screen.push(current_row.clone());
        *current_row = "".to_string();
        *current_pixel = 0;
    }
    //println!("cycle: {}, curr_pix: {} register: {},  row: {}", cycle, current_pixel, register, current_row);
}

fn part_two() -> String{
    let mut register: i32 = 1;
    let mut cycle: i32 = 0;

    let mut current_pixel: i32 = 0;
    let mut current_row = "".to_string();
    let mut screen: Vec<String> = Vec::new();

    read_to_string("input/day10.txt").unwrap()
        .lines()
        .for_each(|line| {

            let vars: Vec<String> = line.split_whitespace()
                .map(|char| char.to_string())
                .collect();

            if vars[0] == "noop"{
                cycle += 1;
                save_row(cycle, register, &mut current_pixel, &mut current_row, &mut screen);
            }

            if vars[0] == "addx"{
                cycle += 1;
                save_row(cycle, register, &mut current_pixel, &mut current_row, &mut screen);
                cycle += 1;
                save_row(cycle, register, &mut current_pixel, &mut current_row, &mut screen);
                register += vars[1].parse::<i32>().unwrap();
            }
        });

    screen.iter().for_each(|line| println!("{}", line));
    return "RLEZFLGE".to_string();
}