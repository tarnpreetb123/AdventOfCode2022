use std::cmp::{min, Ordering};
use std::fs::read_to_string;
use crate::{Solution, SolutionPair};

pub fn solve() -> SolutionPair {
    (Solution::U32(part_one()), Solution::U32(part_two()))
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Packet {
    Int(i32),
    Packet(Vec<Packet>),
    Empty
}

#[derive(PartialEq)]
enum IsInOrder {
    Left,
    Right,
    None
}

fn parse(input_list: &Vec<char>) -> Packet{
    let mut packet_data = Vec::new();
    let mut i = 0;

    while i < input_list.len(){
        let current_char = input_list[i];
        if current_char == '[' {
            let mut next_packet_size = i + 1;
            let mut count = 1;
            while count > 0 {
                if input_list[next_packet_size] == '[' {
                    count += 1;
                } else if input_list[next_packet_size] == ']' {
                    count -= 1;
                }
                next_packet_size += 1;
            }
            if next_packet_size == i + 1{
                packet_data.push(Packet::Empty);
            } else {
                packet_data.push(parse(&input_list[i+1 .. next_packet_size-1].to_vec()))
            }
            i = next_packet_size;
        } else if current_char == ','{
            i += 1;
        } else {
            let mut value = input_list[i].to_digit(10).unwrap() as i32;
            let mut next_packet_size = i + 1;
            while next_packet_size < input_list.len(){
                if !input_list[next_packet_size].is_digit(10){
                    break;
                }
                value = value * 10 + input_list[next_packet_size].to_digit(10).unwrap() as i32;
                next_packet_size += 1;
            }
            packet_data.push(Packet::Int(value));
            i = next_packet_size;
        }
    }

    return Packet::Packet(packet_data);
}

fn compare_packets(left: &Packet, right: &Packet) -> IsInOrder {
    match left{
        Packet::Int(left_number) => {
            match right{
                Packet::Int(right_number) => {
                    return if left_number < right_number {
                        IsInOrder::Left
                    } else if left_number > right_number {
                        IsInOrder::Right
                    } else {
                        IsInOrder::None
                    }
                }
                Packet::Packet(right_list) => {
                    if right_list.len() == 0{
                        return IsInOrder::Right;
                    }
                    return compare_packets(&Packet::Packet(vec![left.clone()]), right);
                }
                Packet::Empty => {
                    return IsInOrder::Right;
                }
            }
        }
        Packet::Packet(left_list) => {
            match right{
                Packet::Int(_) => {
                    if left_list.len() == 0{
                        return IsInOrder::Left
                    }
                    return compare_packets(left, &Packet::Packet(vec![right.clone()]))
                }
                Packet::Packet(right_list) => {
                    let mut i = 0;
                    let smaller_packet = min(left_list.len(), right_list.len());
                    while i < smaller_packet{
                        let compare_result = compare_packets(&left_list[i], &right_list[i]);
                        if compare_result == IsInOrder::None{
                            i += 1;
                        } else if compare_result == IsInOrder::Right{
                            return IsInOrder::Right;
                        } else{
                            return IsInOrder::Left;
                        }
                    }
                    if i < left_list.len(){
                        return IsInOrder::Right;
                    }

                    if i < right_list.len(){
                        return IsInOrder::Left;
                    }
                    return IsInOrder::None;
                }
                Packet::Empty => {
                    return IsInOrder::Right;
                }
            }
        }
        Packet::Empty => {
            return match right {
                Packet::Int(_) => {
                    IsInOrder::Left
                }
                Packet::Packet(_) => {
                    IsInOrder::Left
                }
                Packet::Empty => {
                    IsInOrder::None
                }
            }
        }
    }
}

fn part_one() -> u32{
    let input = read_to_string("input/day13.txt").unwrap();
    let packet_pairs: Vec<_> = input.split("\n\n").collect();

    let mut result_count = 0;
    let mut index = 0;

    for packet_pair in &packet_pairs{
        let packets: Vec<_> = packet_pair.lines().collect();
        let left: Vec<char> = packets[0].chars().collect();
        let right: Vec<char> = packets[1].chars().collect();
        let left_packet = parse(&left);
        let right_packet = parse(&right);

        index += 1;
        let compare_result = compare_packets(&left_packet,&right_packet);
        if compare_result == IsInOrder::Left{
            result_count += index;
        }
    }
    return result_count;
}

fn part_two() -> u32{
    let input = read_to_string("input/day13.txt").unwrap();
    let mut input_packets: Vec<Packet> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let packet: Vec<char> = line.chars().collect();
            return parse(&packet);
        })
        .collect();

    let mut result_count = 1;

    input_packets.push(Packet::Packet(vec![Packet::Int(2)]));
    input_packets.push(Packet::Packet(vec![Packet::Int(6)]));

    input_packets.sort_by(|a, b| {
        let compare_result = compare_packets(a, b);
        if compare_result == IsInOrder::Left{
            Ordering::Less
        } else if compare_result == IsInOrder::Right{
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    for (index, data_packet) in input_packets.iter().enumerate(){
        if data_packet == &Packet::Packet(vec![Packet::Int(2)]) ||
            data_packet == &Packet::Packet(vec![Packet::Int(6)]){
            result_count *= index as u32 + 1;
        }
    }
    return result_count;
}