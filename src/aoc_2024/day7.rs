use std::error::Error;
use std::fs::read_to_string;
use itertools::Itertools;

const FILE_PATH: &str = "./inputs/aoc_2024/inputs_day7.txt";

enum PART {
    PART1,
    PART2
}

fn parse_input() -> Vec<(i64, Vec<i64>)> {
    read_to_string(&FILE_PATH).expect("Unable to read file")
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let target = parts[0].trim().parse::<i64>().unwrap();
            let numbers = parts[1]
                .trim()
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect();
            (target, numbers)
        })
        .collect()
}

pub fn get_response() -> Result<(i64, i64), Box<dyn Error>> {
    let input = parse_input();

    Ok((find_total_calibration(&input, &PART::PART1), find_total_calibration(&input, &PART::PART2)))
}

fn evaluate_expression(numbers: &[i64], operators: &[&str], part:&PART) -> i64 {
    let mut result = numbers[0];
    for (i, &op) in operators.iter().enumerate() {
        match op {
            "+" => result += numbers[i + 1],
            "*" => result *= numbers[i + 1],
            _ => {
                match part {
                    PART::PART1 => {}
                    PART::PART2 => {
                        match op {
                            "||" => {
                                let concatenated = format!("{}{}", result, numbers[i + 1])
                                    .parse::<i64>()
                                    .unwrap();
                                result = concatenated;
                            },
                            &_ => {panic!("invalid operator")}
                        }
                    }
                }
            },
        }
    }
    result
}

fn find_total_calibration(equations: &Vec<(i64, Vec<i64>)>, part: &PART) -> i64 {
    let mut total = 0;

    for (target, numbers) in equations {
        let n = numbers.len() - 1; // Nombre de positions pour les opérateurs
        let mut valid = false;

        // Générer toutes les combinaisons possibles d'opérateurs
        for operators in (0..n).map(|_| ["+", "*", "||"].iter()).multi_cartesian_product() {
            let operators: Vec<&str> = operators.into_iter().cloned().collect();
            if evaluate_expression(&numbers, &operators, part) == *target {
                valid = true;
                break;
            }
        }

        if valid {
            total += target;
        }
    }

    total
}