use std::io;
use io::Result;
use std::fs::read_to_string;
use std::str::Lines;

const FILE_PATH: &str = "./inputs/aoc_2023/day6/inputs.txt";

#[derive(Debug)]
struct Races {
    time: u64,
    distance: u64
}

pub fn day6_main() -> Result<(u64, u64)>{
    let races = parse_file(read_to_string(FILE_PATH)?.lines());

    println!("{:?}", races);

    Ok((42, 42))
}

fn parse_file(lines:Lines) -> Vec<Races>{
    let mut data = vec![];
    let mut races = vec![];

    for line in lines {
        data.push(extract_value(line).iter().map(|str_value| str_value.parse::<u64>().unwrap()).collect::<Vec<u64>>())
    }

    for indx in 0..data[0].len() {
        races.push(Races{time: data[0][indx], distance: data[1][indx]})
    }

    races
}

fn extract_value(line: &str) -> Vec<&str> {
    let str_value: Vec<&str> = line.split(":").last().unwrap().trim().split_whitespace().collect();

    str_value
}