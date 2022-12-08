
use std::cmp::min;
use std::collections::HashMap;
use std::fs::read_to_string;
use crate::{Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    (Solution::U32(part_one()), Solution::U32(part_two()))
}

fn helper() -> HashMap<String, u32>{

    let mut dir_size: HashMap<String, u32> = HashMap::new();

    let root_dir = "/".to_string();
    let mut dir_path: Vec<String> = Vec::new();
    dir_path.push(root_dir.clone());

    let _input = read_to_string("input/day07.txt").unwrap()
        .lines()
        .for_each(|line| {
            let vars: Vec<String> = line.split_whitespace()
                .map(|str| str.to_string())
                .collect();

            if vars[0] == "$"{
                if vars[1] == "cd"{
                    if vars[2] == root_dir{
                        dir_path.clear();
                        dir_path.push(root_dir.clone());
                    } else if vars[2] == ".."{
                        dir_path.pop();
                    } else {
                        dir_path.push(vars[2].clone());
                    }
                }
            } else if vars[0] != "dir" {
                let mut path_key: String = root_dir.clone();

                for dir in dir_path.iter(){
                    path_key.push_str(&*dir.clone());
                    path_key.push_str("/");
                    let size = vars[0].parse::<u32>().unwrap();
                    dir_size.entry(path_key.clone())
                        .and_modify(|dir| *dir += size)
                        .or_insert(size);
                }
            }
        });
    return dir_size;
}

fn part_one() -> u32{

    let dir_size: HashMap<String, u32> = helper();
    let result = dir_size.iter()
        .filter(|(_k, v)| (**v < 100000))
        .fold(0, |acc, (_k, v)| acc + v);

    return result;
}

fn part_two() -> u32{
    let dir_size: HashMap<String, u32> = helper();
    let total_size = 70000000;
    let mut required_size = 30000000;

    required_size = required_size - (total_size - dir_size.get("///").unwrap());

    let mut keys: Vec<String> = dir_size.keys().map(|key| key.to_string()).collect();
    keys.sort();
    let mut result = u32::MAX;

    keys.iter().for_each(|key| {
        let key_size = dir_size.get(key).unwrap();
        if key_size > &required_size {
            result = min(result, *key_size);
        }
    });

    return result;
}