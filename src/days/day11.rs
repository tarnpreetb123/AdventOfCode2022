#![allow(unused)]
use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

pub fn solve() -> SolutionPair {
    (Solution::U64(part_one()), Solution::U64(part_two()))
}

fn do_operation(input: u64, mul: u64, add: u64, pow: u32) -> u64{
    return (input * mul + add).pow(pow)
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    lhs: String,
    operation: String,
    rhs: String,
    divisible_by: u32,
    if_true: u32,
    if_false: u32,
    items_inspected: u64
}

impl Monkey {
    fn new (items: &Vec<u64>, lhs: &str, operation: &str, rhs: &str,
            divisible_by: u32, if_true: u32, if_false: u32) -> Monkey{
        Monkey{
            items: items.clone(),
            lhs: lhs.to_string(),
            operation: operation.to_string(),
            rhs: rhs.to_string(),
            divisible_by,
            if_true,
            if_false,
            items_inspected: 0
        }
    }
}

fn parse_monkeys() -> Vec<Monkey>{
    let input = read_to_string("input/day11.txt").unwrap();
    let input_monkeys: Vec<_> = input.split("\n\n").collect();
    let mut monkeys: Vec<Monkey> = Vec::new();
    input_monkeys.iter().for_each(|monkey| {
        let monkey_data: Vec<_> = monkey.lines().collect();
        let monkey_items = monkey_data[1]
            .split(":")
            .collect::<Vec<_>>()[1]
            .trim()
            .split(", ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        let monkey_operations: Vec<String> = monkey_data[2]
            .split("=")
            .collect::<Vec<_>>()[1]
            .split_whitespace()
            .map(|x| x.to_string())
            .collect();
        let monkey_test: Vec<_> = monkey_data[3..6]
            .iter()
            .map(|line| line
                .split_whitespace()
                .next_back()
                .unwrap()
                .parse::<u32>()
                .unwrap()
            )
            .collect();
        let monkey = Monkey::new(&monkey_items,&monkey_operations[0],&monkey_operations[1],
                                 &monkey_operations[2],monkey_test[0],monkey_test[1],monkey_test[2]);
        monkeys.push(monkey);
    });
    return monkeys
}

fn run_round (monkeys: &mut Vec<Monkey>, i: usize, is_part_two: bool, divisor: u64) {
    let mut item = monkeys[i].items.pop().unwrap();
    let rhs = &monkeys[i].rhs;
    let mut mul = 1;
    let mut add = 0;
    let mut pow = 1;

    if monkeys[i].operation == "*"{
        if rhs == "old"{
            pow = 2
        } else {
            mul = rhs.parse::<u64>().unwrap();
        }
    } else if monkeys[i].operation == "+"{
        if rhs == "old"{
            add = item;
        } else {
            add = rhs.parse::<u64>().unwrap();
        }
    }
    if is_part_two{
        item = do_operation(item, mul, add, pow)%divisor;
    } else{
        item = do_operation(item, mul, add, pow)/3;
    }

    let next_monkey: usize;
    if item % monkeys[i].divisible_by as u64 == 0{
        next_monkey = monkeys[i].if_true as usize;
    } else {
        next_monkey = monkeys[i].if_false as usize;
    }

    monkeys[next_monkey].items.push(item);
}

fn get_monkey_business(monkeys: &mut Vec<Monkey>) -> u64{
    let mut monkey_business: u64 = 1;
    let mut items_inspected: Vec<_> = monkeys.iter().map(|monkey| monkey.items_inspected).collect();
    items_inspected.sort();

    for _ in 0..2{
        monkey_business *= items_inspected.pop().unwrap().clone();
    }
    return monkey_business;
}

fn part_one() -> u64{
    let mut monkeys: Vec<Monkey> = parse_monkeys();

    let num_rounds = 20;
    for round in 0..num_rounds{
        for i in 0..monkeys.len(){
            monkeys[i].items_inspected += monkeys[i].items.len() as u64;
            for j in 0..monkeys[i].items.len(){
                run_round(&mut monkeys, i, false, 0);
            }
        }
    }
    return get_monkey_business(&mut monkeys);
}

fn part_two() -> u64{
    let mut monkeys: Vec<Monkey> = parse_monkeys();

    let mut common_divisor = 1;
    for i in 0..monkeys.len(){
        common_divisor *= monkeys[i].divisible_by as u64;
    }

    let num_rounds = 10000;
    for round in 0..num_rounds{
        for i in 0..monkeys.len(){
            monkeys[i].items_inspected += monkeys[i].items.len() as u64;
            for j in 0..monkeys[i].items.len(){
                run_round(&mut monkeys, i, true, common_divisor);
            }
        }
    }
    return get_monkey_business(&mut monkeys);
}
