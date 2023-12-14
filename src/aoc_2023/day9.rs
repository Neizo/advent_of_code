use std::fs::read_to_string;
use std::io;
use io::Result;
use std::cmp::{max, min};
use num::Signed;

const FILE_PATH: &str = "./inputs/aoc_2023/day9/inputs.txt";

pub fn day9_main() -> Result<(i64, i64)>{
    let numbers = read_to_string(FILE_PATH)?.lines().map(|line| extract_number(line)).collect::<Vec<Vec<i64>>>();

    Ok((part_1(&numbers), part_2(&numbers)))
}

fn extract_number(line:&str) -> Vec<i64> {
   line.trim().split_whitespace().map(|value_str| value_str.parse::<i64>().unwrap()).collect::<Vec<i64>>()
}

fn part_1(numbers:&Vec<Vec<i64>>) -> i64{
    numbers.iter().map(|vec_element| vec_element.last().unwrap() + extract_diff(&vec_element)).collect::<Vec<i64>>().iter().sum()
}

fn extract_diff(numbers:&Vec<i64>) -> i64{
    let mut diffs = vec![];

    diffs.push(numbers[0]);
    diffs = (1..numbers.len()).map(|indx| numbers[indx] - numbers[indx-1]).collect::<Vec<i64>>();

    if diffs.iter().any(|&diff| diff != 0) == true {
        return diffs.last().unwrap() + extract_diff(&diffs);
    }

    return 0i64
}

fn part_2(numbers:&Vec<Vec<i64>>) -> i64 {
    let mut sum = 0;

    for mut number in numbers.clone() {
        number.reverse();
        sum += number.last().unwrap() + extract_diff(&number);
    }

    sum
}