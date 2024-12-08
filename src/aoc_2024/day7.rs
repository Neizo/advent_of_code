use std::error::Error;
use std::fs::read_to_string;

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

fn compute(a: i64, b: i64, op: &str, part:&PART) -> Option<i64> {
    match op {
        "+" => Some(a + b),
        "*" => Some(a * b),
        _ => {
            match part {
                PART::PART1 => {None}
                PART::PART2 => {
                    match op {
                        "||" => {
                            let digits = b.to_string();
                            let concatenated = format!("{}{}", a, digits).parse::<i64>().ok()?;
                            Some(concatenated)
                        }
                        &_ => {None}
                    }
                }
            }
        },
    }
}

fn can_solve(numbers: &[i64], target: i64, part:&PART) -> bool {
    // BFS pour explorer les combinaisons d'opérateurs
    let mut queue = vec![(numbers[0], 0)]; // (résultat courant, index du nombre suivant)

    while let Some((current, index)) = queue.pop() {
        if index == numbers.len() - 1 {
            if current == target {
                return true;
            }
            continue;
        }

        /*On avance chiffre par chiffre de l'équation et on remplis queue tant qu'on dépasse pas le résultat attendu, si on le dépasse on arrête de remplir et on sort du while,
        * ce qui fait gagner du temps car on ne fait pas toutes les équations à 100% on s'arrête dès qu'on sais que c'est plus valide
        */
        for &op in &["+", "*", "||"] {
            if let Some(next_result) = compute(current, numbers[index + 1], op, part) {
                if next_result <= target {
                    queue.push((next_result, index + 1));
                }
            }
        }
    }

    false
}

fn find_total_calibration(equations: &Vec<(i64, Vec<i64>)>, part: &PART) -> i64 {
    equations
        .iter()
        .filter_map(|(target, numbers)| if can_solve(numbers, *target, part) { Some(target) } else { None })
        .sum()
}