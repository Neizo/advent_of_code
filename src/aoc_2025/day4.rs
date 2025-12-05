use crate::utils::utils_files::{afficher_resultats, mesurer};

//const FILE_PATH_TEST: &str = "./inputs/aoc_2025/day4/inputs_test.txt";
const FILE_PATH_E1: &str = "./inputs/aoc_2025/day4/inputs_e1.txt";

fn parse_input(_file_path: &str) -> Vec<Vec<char>> {
    let content = std::fs::read_to_string(_file_path)
        .expect("Failed to read file");

    content
        .lines()
        .map(|line|
            line.chars().map(|ch| ch).collect()
        )
        .collect()
}

pub fn get_response() {
    let (enigme1_result, time_e1) = mesurer(enigme1);
    let (enigme2_result, time_e2) = mesurer(enigme2);

    afficher_resultats(4, enigme1_result, time_e1, enigme2_result,time_e2);
}

fn trouve_rouleau_isoler(grid: &mut Vec<Vec<char>>, replace:bool) -> Vec<(usize, usize)> {
    let mut isolated = Vec::new();
    let directions: [(i8, i8); 8] = [(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)];
    let initial_grid = grid.clone();

    for i in 0..initial_grid.len() {
        for j in 0..initial_grid[i].len() {
            if initial_grid[i][j] != '@' {continue}
            let mut neighbor_count = 0;
            for (x, y) in directions {
                let (ni, nj) = (i as i32 + x as i32, j as i32 + y as i32);
                if ni < 0 || nj < 0 {continue;}
                if ni >= initial_grid.len() as i32 || nj >= initial_grid[i].len() as i32 {continue;}
                if initial_grid[ni as usize][nj as usize] == '@' {
                    neighbor_count += 1;
                    if neighbor_count >= 4 {break;}
                }
            }

            if neighbor_count < 4 {
                grid[i][j] = 'x';
                isolated.push((i, j));
            }
        }
    }

    if isolated.len() > 0 && replace == true {
        isolated.append(&mut trouve_rouleau_isoler(grid, replace));
    }

    isolated
}

pub fn enigme1() -> usize {
    let mut grid = parse_input(FILE_PATH_E1);
    trouve_rouleau_isoler(&mut grid, false).len()
}

pub fn enigme2() -> usize {
    let mut grid = parse_input(FILE_PATH_E1);
    trouve_rouleau_isoler(&mut grid, true).len()
}