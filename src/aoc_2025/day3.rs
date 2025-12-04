const FILE_PATH_E1: &str = "./inputs/aoc_2025/day3/inputs_e1.txt";
//const FILE_PATH_TEST: &str = "./inputs/aoc_2025/day3/inputs_test.txt";
fn parse_input(_file_path: &str) -> Vec<Vec<u64>> {
    let content = std::fs::read_to_string(_file_path)
        .expect("Failed to read file");

    content
        .lines()
        .map(|line|
            line.chars().map(|ch| ch.to_digit(10).unwrap() as u64).collect()
        )
        .collect()
}

pub fn get_response() -> (u64, u64) {
    let enigme1_result = enigme1();
    let enigme2_result = enigme2();
    (enigme1_result, enigme2_result)
}

pub fn enigme1() -> u64 {
    let bank_array = parse_input(FILE_PATH_E1);
    let mut combinaison: Vec<Vec<u64>> = Vec::new();
    for bank_line in bank_array {
        let mut bank_line_combinaison: Vec<u64> = Vec::new();
        for (idx, bank) in bank_line.iter().enumerate() {
            for bank2 in &bank_line[idx+1..bank_line.len()] {
                bank_line_combinaison.push((bank.to_string() + &bank2.to_string()).parse().unwrap());
            }
        }
        combinaison.push(bank_line_combinaison);
    }

    combinaison.iter().map(|v| *v.iter().max().unwrap()).collect::<Vec<u64>>().iter().sum::<u64>()
}

fn find_largest_joltage(digits: Vec<u64>) -> String {
    let total_len = digits.len();
    let target_len = 12;
    let to_remove = total_len - target_len;

    let mut result = digits.clone();

    // On supprime "to_remove" chiffres en choisissant toujours
    // le plus petit qui bloque un plus grand à sa droite
    for _ in 0..to_remove {
        let mut min_pos = 0;

        /* Pour maximiser le nombre final, on veut supprimer les "petits" chiffres qui bloquent des "grands" chiffres à leur droite.*/
        let mut found = false;
        for i in 0..result.len() - 1 {
            if result[i] < result[i + 1] {
                min_pos = i;
                found = true;
                break;
            }
            // Si on arrive à la fin sans trouver, on prend le dernier
            if i == result.len() - 2 {
                min_pos = result.len() - 1;
            }
        }

        // Si tous les chiffres sont en ordre décroissant, supprimer le dernier
        if found == false && result.len() > 1 {
            min_pos = result.len() - 1;
        }

        result.remove(min_pos);
    }

    result.iter().map(|d| d.to_string()).collect()
}

pub fn enigme2() -> u64 {
    let bank_array = parse_input(FILE_PATH_E1);
    let mut joltages = Vec::new();

    for bank_line in bank_array {
        joltages.push(find_largest_joltage(bank_line).parse::<u64>().unwrap());
    }

    joltages.iter().sum::<u64>()
}
