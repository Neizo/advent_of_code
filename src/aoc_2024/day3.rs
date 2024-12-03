use std::error::Error;
use std::fs::read_to_string;
use regex::Regex;

const FILE_PATH: &str = "./inputs/aoc_2024/inputs_day3.txt";

fn parse_file() -> Result<String, Box<dyn Error>> {
    Ok(read_to_string(FILE_PATH)?)
}

pub fn get_response() -> Result<(i64, i64), Box<dyn Error>> {
    let files = parse_file()?;

    let numbers = find_mul(&files);


    Ok((enigme1(&numbers), part2(&files)))
}

fn find_mul(files:&str) -> Vec<(i64, i64)> {
    let pattern = r"mul\((\d+),(\d+)\)";
    let regex = Regex::new(pattern).expect("Regex invalide");

    // Trouver toutes les correspondances et capturer les chiffres
    regex
        .captures_iter(files)
        .filter_map(|cap| {
            let num1 = cap.get(1)?.as_str().parse::<i64>().ok()?; // Premier chiffre capturé
            let num2 = cap.get(2)?.as_str().parse::<i64>().ok()?; // Deuxième chiffre capturé
            Some((num1, num2))
        })
        .collect()
}

fn enigme1(numbers:&Vec<(i64, i64)>) -> i64 {
    numbers.iter().map(|(value1, value2)| (value1*value2)).sum()
}

fn part2(files:&String) -> i64 {
    let inputs = files.clone() + "don't()";
    let mut enabled = true;
    let mut index_debut_tmp = 0usize;
    let mut plage_activation = vec![];

    for index in 0..inputs.len() {
        let sliced =  &inputs[index..];

        if sliced.starts_with("do()") && enabled == false {
            enabled = true;
            index_debut_tmp = index;
        }

        if sliced.starts_with("don't()") && enabled == true {
            enabled = false;
            plage_activation.push((index_debut_tmp, index));
        }
    }

    plage_activation
        .iter()
        .map(|(start, end)| {
            find_mul(&inputs[*start..=*end])
                .iter()
                .map(|(value1, value2)| value1 * value2)
                .sum::<i64>()
        })
        .sum()
}