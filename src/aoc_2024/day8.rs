use std::collections::{HashMap, HashSet};
use std::error::Error;
use crate::utils::utils_files::parse_file;

const FILE_PATH: &str = "./inputs/aoc_2024/inputs_day8.txt";

pub fn get_response() -> Result<(usize, usize), Box<dyn Error>> {
    let input = parse_file(FILE_PATH.to_string());
    let inputs = input.lines().map(|line| line).collect::<Vec<&str>>();

    let map = parse_map(&inputs);

    let antinode_count = count_antinodes(&map, inputs.len(), inputs[0].len());

    Ok((antinode_count, antinode_count))
}

// Parse la carte pour extraire les positions des antennes
fn parse_map(input: &[&str]) -> HashMap<char, Vec<(usize, usize)>> {
    let mut map = HashMap::new();
    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c != '.' {
                map.entry(c).or_insert_with(Vec::new).push((y, x));
            }
        }
    }
    map
}

// Compte les antinodes générés par les antennes
fn count_antinodes(
    map: &HashMap<char, Vec<(usize, usize)>>,
    height: usize,
    width: usize,
) -> usize {
    let mut antinode_positions = HashSet::new();

    for (_, positions) in map {
        // Parcourt toutes les paires d'antennes de la même fréquence
        for (_, &(y1, x1)) in positions.iter().enumerate() {
            for &(y2, x2) in positions.iter() {
                if x1 == x2 && y1 == y2 {continue}
                // Calcul de la distance et vérification des antinodes
                find_antinodes(y1, x1, y2, x2, height, width, &mut antinode_positions);
            }
        }
    }
    
    antinode_positions.len()
}

// Trouve les antinodes pour une paire d'antennes
fn find_antinodes(
    y1: usize,
    x1: usize,
    y2: usize,
    x2: usize,
    height: usize,
    width: usize,
    antinode_positions: &mut HashSet<(usize, usize)>,
) {
    let dy = y2 as isize - y1 as isize;
    let dx = x2 as isize - x1 as isize;

    // Vérifie si les antennes sont alignées

    // Calcul de l'antinode en respectant la règle des distances
    let mid_y = (y1 as isize + y2 as isize) / 2;
    let mid_x = (x1 as isize + x2 as isize) / 2;

    let far_y = y2 as isize + dy;
    let far_x = x2 as isize + dx;

    let near_y = y1 as isize - dy;
    let near_x = x1 as isize - dx;

    // Ajouter les positions valides dans la carte
    if is_valid_position(mid_y, mid_x, height, width) {
        antinode_positions.insert((mid_y as usize, mid_x as usize));
    }

    if is_valid_position(far_y, far_x, height, width) {
        antinode_positions.insert((far_y as usize, far_x as usize));
    }

    if is_valid_position(near_y, near_x, height, width) {
        antinode_positions.insert((near_y as usize, near_x as usize));
    }
}

// Vérifie si une position est dans les limites de la carte
fn is_valid_position(y: isize, x: isize, height: usize, width: usize) -> bool {
    y >= 0 && y < height as isize && x >= 0 && x < width as isize
}