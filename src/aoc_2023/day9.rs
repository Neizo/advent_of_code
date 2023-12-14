use std::fs::read_to_string;
use std::io;
use io::Result;
use std::cmp::{max, min};
use num::Signed;

const FILE_PATH: &str = "./inputs/aoc_2023/day9/inputs.txt";

pub fn day9_main() -> Result<(i64, u64)>{
    let numbers = read_to_string(FILE_PATH)?.lines().map(|line| extract_number(line)).collect::<Vec<Vec<i64>>>();



    Ok((part_1(&numbers),42))
}

fn extract_number(line:&str) -> Vec<i64> {
   line.trim().split_whitespace().map(|value_str| value_str.parse::<i64>().unwrap()).collect::<Vec<i64>>()
}

fn part_1(numbers:&Vec<Vec<i64>>) -> i64 {
    numbers.iter().map(|vec_element| vec_element.last().unwrap() + extract_diff(&vec_element)).collect::<Vec<i64>>().iter().sum()
}

fn extract_diff(numbers:&Vec<i64>) -> i64{
    let mut diffs = vec![];

    println!("{:?}", numbers);

    for indx in 0..numbers.len()-1 {
        diffs.push(numbers[indx+1] - numbers[indx]);
    }

    if diffs.iter().sum::<i64>() != 0 {
        return diffs.last().unwrap() + extract_diff(&diffs);
    }

    return 0i64
}

//1984151189
//1974913982