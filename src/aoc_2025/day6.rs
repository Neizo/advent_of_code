use crate::utils::utils_files::{afficher_resultats, mesurer};

const FILE_PATH_TEST: &str = "./inputs/aoc_2025/day6/inputs_test.txt";
const FILE_PATH_E1: &str = "./inputs/aoc_2025/day6/inputs_e1.txt";

fn parse_input(_file_path: &str) -> (Vec<Vec<usize>>, Vec<char>){
    let content = std::fs::read_to_string(_file_path)
        .expect("Failed to read file");

    let numbers: Vec<Vec<usize>> = content
        .lines()
        .by_ref()
        .take_while(|line| !line.trim().is_empty())
        .map(|line| {

            line.split_whitespace().map(|number| number.parse().unwrap()).collect()
        })
        .collect();

    let operators :  Vec<char> =  content
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.chars().any(|c| c.is_numeric()))
        .flat_map(|line| {
            line.chars()
                .filter(|c| !c.is_whitespace())
        })
        .collect();

    (numbers, operators)
}

pub fn get_response() {
    let (enigme1_result, time_e1) = mesurer(enigme1);
    let (enigme2_result, time_e2) = mesurer(enigme2);

    afficher_resultats(6, enigme1_result, time_e1, enigme2_result, time_e2);
}

pub fn transposition_matrice(matrice: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    // Trouver la longueur maximale des lignes (nombre de colonnes)
    let max_len = matrice.iter().map(|row| row.len()).max().unwrap_or(0);

    (0..max_len)
        .map(|i| {
            matrice.iter()
                .filter_map(|row| row.get(i).copied())
                .collect()
        })
        .collect()
}

pub fn enigme1() -> usize {
    let (numbers, operators) = parse_input(FILE_PATH_E1);
    let numbers = transposition_matrice(numbers);
    let mut result = 0;

    for (idx, operator) in operators.iter().enumerate() {
        result += match operator {
            '*' => numbers[idx].iter().product::<usize>(),
            '+' => numbers[idx].iter().sum::<usize>(),
            _ => {println!("Operateur {} non géré", operator);
                0
            }
        };
    }

    result
}
pub fn decompose(numbers: &Vec<usize>) -> Vec<usize> {
    let mut result = Vec::new();
    let mut remaining: Vec<usize> = numbers.clone();

    while !remaining.is_empty() {
        // Extraire les chiffres les plus à droite
        let concatenated = remaining
            .iter()
            .map(|&n| (n % 10).to_string())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        result.push(concatenated);

        // Retirer le chiffre le plus à droite de chaque nombre
        remaining = remaining
            .iter()
            .map(|&n| n / 10)
            .filter(|&n| n > 0)
            .collect();
    }

    result
}

pub fn enigme2() -> usize {
    let (numbers, operators) = parse_input(FILE_PATH_TEST);
    let numbers = transposition_matrice(numbers);
    println!("{:?}", numbers);

    for (idx, operator) in operators.iter().enumerate() {
        let decomposer = decompose(&numbers[idx]);
        println!("{:?}", numbers[idx]);
        println!("{:?}", decomposer);
        println!();
    }
    // TODO: Implement solution for part 2
    0
}