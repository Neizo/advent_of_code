use std::fs::read_to_string;
use std::io;

const FILE_PATH: &str = "./inputs/aoc_2023/day1/inputs_step1.txt";

pub fn day1_main() -> io::Result<u32>{
    let mut sum = 0;

    for line in read_to_string(FILE_PATH)?.lines() {
        let mut line_value = 0;
        let mut line_numbers = enigme1(line);

        if line_numbers.len() == 1 {line_numbers.push(line_numbers[0]);}

        if line_numbers.len() >= 2 {
            line_value = &line_numbers[0] * 10 + &line_numbers[line_numbers.len()-1];
        }

        sum += line_value;
    }

    Ok(sum)
}

pub fn enigme1(_line:&str) -> Vec<u32>{
    _line.chars().filter_map(|char| char.to_digit(10)).collect()
}