use std::fs::read_to_string;
use std::io;

#[warn(dead_code)]
const FILE_PATH: &str = "./inputs/aoc_2023/day1/inputs.txt";

pub fn day1_main() -> io::Result<(u32, u32)>{

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
    let mut line_numbers = vec![];
    let words = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut indx = 0;

    while indx < line.len() {
        if let Some(idx_word) = words.iter().position(|&word| line[indx..].starts_with(word)) {
            line_numbers.push((idx_word % 10) as u32);
        }
        indx += 1;
    }

    get_response(line_numbers)
}