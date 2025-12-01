const FILE_PATH_TEST: &str = "./inputs/aoc_2025/day1/inputs_test.txt";
const FILE_PATH_E1: &str = "./inputs/aoc_2025/day1/inputs_e1.txt";
fn parse_input(_file_path:&str) -> Vec<(char, i64)> {
    let content = std::fs::read_to_string(_file_path)
        .expect("Failed to read file");

    content
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }
            let direction = line.chars().next().unwrap();
            let distance: i64 = line[1..].parse().ok()?;
            Some((direction, distance))
        })
        .collect()
}

pub fn get_response() -> (i64, i32) {
    let enigme1_result = enigme1();
    let enigme2_result = enigme2();
    (enigme1_result, enigme2_result)
}

pub fn enigme1() -> i64 {
    let moves = parse_input(FILE_PATH_E1);
    let mut dial = 50i64;
    let mut compteur = 0i64;

    for (direction, nb_move) in moves {
        match direction {
            'L' => {
                dial -= nb_move % 100;
            },
            'R' => {
                dial += nb_move % 100;
            },
            _ => println!("{} non géré", direction),
        }

        dial = dial.rem_euclid(100);
        if dial == 0 {compteur += 1}
    }

    compteur
}

fn count_zero_crossings(start: i64, direction: char, steps: i64) -> i64 {
    let mut count = 0;
    let mut current = start;

    // Déterminer la direction de rotation
    let step_direction = if direction == 'L' { -1 } else { 1 };

    // Pour chaque pas de la rotation
    for _ in 0..steps {
        current = (current + step_direction).rem_euclid(100);
        if current == 0 {
            count += 1;
        }
    }

    count
}

pub fn enigme2() -> i32 {
    let moves = parse_input(FILE_PATH_E1);
    let mut dial = 50i64;
    let mut compteur = 0i64;

    for (direction, nb_move) in moves {
        compteur += count_zero_crossings(dial, direction, nb_move);
        let step_direction = if direction == 'L' { -1 } else { 1 };
        dial = (dial + step_direction * nb_move).rem_euclid(100);
    }

    compteur as i32
}
