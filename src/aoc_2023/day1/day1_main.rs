use std::fs::read_to_string;
use std::io;
use io::Result;

#[warn(dead_code)]
const FILE_PATH: &str = "./inputs/aoc_2023/day1/inputs.txt";

pub fn day1_main() -> Result<(u32, u32)>{

    let (sum_e1, sum_e2) = read_to_string(FILE_PATH)?.lines().fold((0, 0), |(sum1, sum2), line| {
        let e1 = enigme1(line);
        let e2 = enigme2(line);
        (sum1 + e1, sum2 + e2)
    });

    Ok((sum_e1, sum_e2))
}

pub fn enigme1(_line:&str) -> u32{
    get_response(_line.chars().filter_map(|char| char.to_digit(10)).collect())
}

fn get_response(numbers:Vec<u32>) -> u32{
    match numbers.len() {
        0 => 0,
        1 => numbers.first().unwrap() * 11,
        _ => numbers.first().unwrap() * 10 + numbers.last().unwrap(),
    }
}

pub fn enigme2(line:&str) -> u32 {
    let words = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    get_response(line.chars()
        .enumerate()
        .filter_map(|(indx, _)| words.iter().position(|&word| line[indx..].starts_with(word)))
        .collect::<Vec<usize>>().iter()
        .map(|idx| (idx % 10) as u32)
        .collect::<Vec<u32>>())
}