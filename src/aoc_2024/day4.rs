use std::error::Error;
use std::fs;
#[allow(dead_code)]
const FILE_PATH: &str = "./inputs/aoc_2024/inputs_day4.txt";

#[allow(dead_code)]
fn parse_file() -> Vec<Vec<char>> {
    let input = fs::read_to_string(FILE_PATH).expect("Unable to read file");

    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn get_response() -> Result<(i64, i64), Box<dyn Error>> {
    let grid = parse_file();
    let word = "XMAS";

    Ok((count_xmas_occurrences(&grid, word), count_xmas_patterns(&grid)))
}

fn count_xmas_occurrences(grid: &Vec<Vec<char>>, word: &str) -> i64 {
    let directions = [
        (0, 1),   // droite
        (0, -1),  // gauche
        (1, 0),   // bas
        (-1, 0),  // haut
        (1, 1),   // diagonale bas-droite
        (-1, -1), // diagonale haut-gauche
        (1, -1),  // diagonale bas-gauche
        (-1, 1),  // diagonale haut-droite
    ];
    let mut count_p1 = 0;
    let rows = grid.len();
    let cols = grid[0].len();
    let word_len = word.len();

    for r in 0..rows {
        for c in 0..cols {
            for &(dx, dy) in &directions {
                let mut found = true;
                for i in 0..word_len {
                    let nr = r as isize + i as isize * dx;
                    let nc = c as isize + i as isize * dy;
                    if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize {
                        found = false;
                        break;
                    }
                    if grid[nr as usize][nc as usize] != word.chars().nth(i).unwrap() {
                        found = false;
                        break;
                    }
                }
                if found {
                    count_p1 += 1;
                }
            }
        }
    }
    count_p1
}

fn count_xmas_patterns(grid: &Vec<Vec<char>>) -> i64 {
    let directions = [
        ((-1, -1), (1, 1)), // Diagonale montante gauche-droite
        ((-1, 1), (1, -1)), // Diagonale montante droite-gauche
        ((1, -1), (-1, 1)), // Diagonale descendante gauche-droite
        ((1, 1), (-1, -1)), // Diagonale descendante droite-gauche
    ];

    let mut count_p2 = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    // Parcourir chaque cellule de la grille
    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            if grid[r][c] == 'A' {
                let count = directions
                    .iter()
                    .filter(|&&((dx1, dy1), (dx2, dy2))| is_valid_mas(grid, r, c, dx1, dy1, dx2, dy2))
                    .count();

                if count >= 2 {count_p2 += 1}
            }
        }
    }

    count_p2
}

fn is_valid_mas(
    grid: &Vec<Vec<char>>,
    r: usize,
    c: usize,
    dx1: isize,
    dy1: isize,
    dx2: isize,
    dy2: isize,
) -> bool {
    let directions = [
        ((dx1, dy1), 'M'),
        ((dx2, dy2), 'S'),
    ];

    directions.iter().all(|((dx, dy), expected_char)| {
        let nr = r as isize + dx;
        let nc = c as isize + dy;

        // Vérifie les limites et la correspondance des caractères
        nr >= 0
            && nc >= 0
            && nr < grid.len() as isize
            && nc < grid[0].len() as isize
            && grid[nr as usize][nc as usize] == *expected_char
    })
}