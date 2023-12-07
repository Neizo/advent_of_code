use std::fs::read_to_string;
use std::io;

#[warn(dead_code)]
const FILE_PATH: &str = "./inputs/aoc_2023/day1/inputs.txt";

pub fn day1_main() -> io::Result<u32>{
    let mut sum = 0;

    for line in read_to_string(FILE_PATH)?.lines() {
        let mut line_value = 0;
        let mut line_numbers = enigme2(line);

        if line_numbers.len() == 1 {line_numbers.push(line_numbers[0]);}

        if line_numbers.len() >= 2 {
            line_value = line_numbers.first().unwrap() * 10 + line_numbers.last().unwrap();
        }

        sum += line_value;
    }

    Ok(sum)
}

pub fn enigme1(_line:&str) -> Vec<u32>{
    _line.chars().filter_map(|char| char.to_digit(10)).collect()
}

pub fn enigme2(_line:&str) -> Vec<u32> {
    let mut line_numbers = vec![];
    let words = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut indx = 0;

    while indx < _line.len() {
        for (idx_word, word) in words.iter().enumerate() {
            if _line[indx..].starts_with(word) {
                line_numbers.push((&idx_word%10) as u32);
                break;
            }
        }

        indx += 1;
    }

    line_numbers
}