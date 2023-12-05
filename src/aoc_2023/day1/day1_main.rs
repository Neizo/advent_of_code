use std::fs::read_to_string;

pub fn day1_main() {
    let file_path = "./inputs/aoc_2023/day1/inputs.txt";
    let mut sum = 0;

    for line in read_to_string(file_path).unwrap().lines() {
        let mut line_value = 0;
        let mut line_numbers = enigne1(line);

        if line_numbers.len() == 1 {line_numbers.push(line_numbers[0]);}

        if line_numbers.len() >= 2 {
            line_value = &line_numbers[0] * 10 + &line_numbers[line_numbers.len()-1];
        }

        sum += line_value;
    }

    println!("The answer for step 1 of day 1 is {}", sum);
}

pub fn enigne1(_line:&str) -> Vec<u32>{
    let mut line_numbers = vec![];

    for char in _line.chars() {
        let digit = char.to_digit(10);
        match digit {
            Some(data) => line_numbers.push(data),
            _ => {}
        }
    }

    line_numbers
}