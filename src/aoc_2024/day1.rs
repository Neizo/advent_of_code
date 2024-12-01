use std::collections::HashMap;
use std::error::Error;
use std::fs::read_to_string;

const FILE_PATH: &str = "./inputs/aoc_2024/inputs_day1.txt";

pub fn get_response() -> Result<(i64, i64), Box<dyn Error>> {
    match parse_file() {
        Ok((mut left_numbers, mut right_numbers)) => {
            left_numbers.sort();
            right_numbers.sort();

            return Ok((enigme1(&left_numbers, &right_numbers), enigme2(&left_numbers, &right_numbers)));
        }
        Err(e) => {return Err(e)}
    }
}

fn parse_file() -> Result<(Vec<i64>, Vec<i64>), Box<dyn Error>> {
    let (mut left_numbers, mut right_numbers) = (Vec::new(), Vec::new());

    read_to_string(FILE_PATH)?
        .lines()
        .map(|line| line.trim().split_whitespace().collect::<Vec<&str>>())
        .for_each(|values| {
            if let (Ok(left), Ok(right)) = (values[0].parse::<i64>(), values[1].parse::<i64>()) {
                left_numbers.push(left);
                right_numbers.push(right);
            }
        });

    Ok((left_numbers, right_numbers))
}

fn enigme1(left_numbers:&Vec<i64>, right_numbers:&Vec<i64>) -> i64 {
    left_numbers
        .iter()
        .zip(right_numbers.iter()) // Parcourt les deux vecteurs en parallèle
        .map(|(left, right)| (left - right).abs()) // Calcule la distance absolue pour chaque paire
        .sum() // Somme toutes les distances
}

fn enigme2(left_numbers:&Vec<i64>, right_numbers:&Vec<i64>) -> i64 {
    let mut occurrences = HashMap::new();
    for &number in right_numbers {
        *occurrences.entry(number).or_insert(0) += 1;
    }

    // Calculer le score de similarité
    left_numbers
        .iter()
        .map(|&left_number| {
            let count = occurrences.get(&left_number).copied().unwrap_or(0);
            left_number * count
        })
        .sum()
}