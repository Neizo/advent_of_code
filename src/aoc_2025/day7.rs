use std::collections::{HashSet};
use crate::utils::utils_files::{afficher_resultats, mesurer};

//const FILE_PATH_TEST: &str = "./inputs/aoc_2025/day7/inputs_test.txt";
const FILE_PATH_E1: &str = "./inputs/aoc_2025/day7/inputs_e1.txt";

fn parse_input(_file_path: &str) -> Vec<Vec<char>> {
    let content = std::fs::read_to_string(_file_path)
        .expect("Failed to read file");

    content
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn get_response() {
    let (enigme1_result, time_e1) = mesurer(enigme1);
    let (enigme2_result, time_e2) = mesurer(enigme2);

    afficher_resultats(7, enigme1_result, time_e1, enigme2_result, time_e2);
}
fn trouve_initial_position(_data: Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (idx_line, line) in _data.iter().enumerate() {
        for (idx_col, column) in line.iter().enumerate() {
            if *column == 'S' {
                return Some((idx_line, idx_col))
            }
        }
    }

    None
}

fn parcourt_matrice(_data: Vec<Vec<char>>, _idx_row: usize, _idx_column: usize, _visited: &mut HashSet<(usize, usize)>) -> usize {
    let mut nb_split = 0;
    let mut idx_row = _idx_row;

    loop {
        if idx_row >= _data.len()-1 || _idx_column > _data[0].len()-1 || _visited.contains(&(idx_row, _idx_column)) {
            _visited.insert((idx_row, _idx_column));
            break;
        }
        _visited.insert((idx_row, _idx_column));

        let next_char = _data[idx_row+1][_idx_column];
        if next_char == '^' {
            //On doit split à gauche et à droite
            if (_idx_column as i32 - 1) >= 0 {nb_split += parcourt_matrice(_data.clone(), idx_row+1, _idx_column-1, _visited)};
            nb_split += parcourt_matrice(_data.clone(), idx_row+1, _idx_column+1, _visited) + 1;
            break;
        }
        idx_row += 1;
    }

    nb_split
}

pub fn enigme1() -> usize {
    let _data = parse_input(FILE_PATH_E1);
    let initial_position = trouve_initial_position(_data.clone());
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let result = match initial_position {
        Some((idx_line, _idx_col)) => parcourt_matrice(_data.clone(), idx_line, _idx_col, &mut visited),
        None => {
            println!("Pas de position initial trouvé");
            0_usize
        }
    };

    result
}
pub fn parcourt_inverser(grid: Vec<Vec<char>>) -> usize {
    let start = grid[0].iter().position(|&b| b == 'S').unwrap();
    let mut path_counts = vec![vec![0; grid[0].len()]; grid.len()];

    *path_counts.last_mut().unwrap() = vec![1; grid[0].len()];
    for i in (0..grid.len() - 1).rev() {
        for ii in 0..grid[i].len() {
            path_counts[i][ii] = if grid[i][ii] == '^' {
                let left = path_counts[i + 1][ii - 1];
                let right = path_counts[i + 1][ii + 1];
                left + right
            } else {
                path_counts[i + 1][ii]
            };
        }
    }

    path_counts[0][start]
}

pub fn enigme2() -> usize {
    let content = parse_input(FILE_PATH_E1);
    parcourt_inverser(content)
}
