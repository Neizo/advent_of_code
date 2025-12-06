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

fn extract_columns(_file_path: &str) -> Vec<(usize,Vec<String>)> {
    let content = std::fs::read_to_string(_file_path)
        .expect("Failed to read file");
    let lines: Vec<&str> = content.lines().by_ref().take_while(|line| !line.trim().is_empty()).collect();
    if lines.is_empty() {
        return Vec::new();
    }

    let mut columns: Vec<(usize,Vec<String>)> = Vec::new();
    let mut positions = 0;

    loop {
        let mut column_data: Vec<String> = Vec::new();
        let mut column_length = vec![0; lines.len()];
        if positions >= lines[0].len() {break;}

        for (i, line) in lines.iter().enumerate() {
            let mut length = 0;
            for char in line[positions..].chars() {
               if char.is_whitespace() {break}
                length += 1;
           }
            column_length[i] = length;
        }

        let max_len = *column_length.iter().max().unwrap() as usize;

        for line in lines.iter() {
            let mut data: String = "".to_string();
            if positions + max_len > line.len() {
                let max_len_lcl = line.len() - positions;
                data.push_str(&" ".repeat(positions + max_len_lcl - (line.len() - 1)));
                let data = line[positions..positions + max_len_lcl].to_string() + data.as_str();
                column_data.push(data);
            } else {
                column_data.push(line[positions..positions + max_len].to_string());
            }

        }

        positions += max_len+1;
        columns.push((max_len, column_data));
    }

    columns
}

fn create_number_from_column(numbers_str: Vec<(usize,Vec<String>)>) -> Vec<Vec<usize>> {
    let mut final_numbers: Vec<Vec<usize>> = Vec::new();

    for (max_len, numbers) in numbers_str.iter() {
        let mut new_number:Vec<String> = vec!["".to_string(); *max_len];
        for number in numbers.iter() {
            for pos in 0..*max_len {
                if number.chars().nth(pos).unwrap().is_whitespace() {continue}
                new_number[pos].push_str(number.chars().nth(pos).unwrap().to_string().as_str());
            }
        }

        final_numbers.push(new_number.iter().map(|number_str| number_str.to_string().parse::<usize>().unwrap()).collect::<Vec<usize>>());
    }

    final_numbers
}
pub fn enigme2() -> usize {
    let (_, operators) = parse_input(FILE_PATH_E1);
    let numbers_str = extract_columns(FILE_PATH_E1);
    let numbers = create_number_from_column(numbers_str);
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