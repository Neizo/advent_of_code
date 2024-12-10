use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs::read_to_string;

const FILE_PATH: &str = "./inputs/aoc_2024/inputs_day10.txt";
pub fn get_response() -> Result<(usize, usize), Box<dyn Error>> {
    let map = parse_map();

    let (trailhead_scores, distinct_trailhead_scores) = calculate_trailhead_scores(&map);
    let total_score: usize = trailhead_scores.iter().map(|(_ , count)| count).sum();
    let total_distinct_score = distinct_trailhead_scores.iter().map(|(_ , count)| count).sum();

    Ok((total_score, total_distinct_score))
}

// Parse la carte en une matrice 2D
fn parse_map() -> Vec<Vec<u8>> {
    let input = read_to_string(&FILE_PATH).expect("Unable to read file");
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

// Calcule les scores pour tous les trailheads
fn calculate_trailhead_scores(map: &[Vec<u8>]) -> (HashMap<(usize, usize), usize>, HashMap<(usize, usize), usize>) {
    let mut scores = HashMap::new();
    let mut distinct_scores = HashMap::new();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 0 {
                let (score, distinct_score) = bfs_trailhead_score(map, y, x);
                for ((reacheable_x, reacheable_y), count) in score {
                   scores.entry((reacheable_x, reacheable_y)).and_modify(|number| *number += count).or_insert(count);
                }

                for ((reacheable_x, reacheable_y), count) in distinct_score {
                    distinct_scores.entry((reacheable_x, reacheable_y)).and_modify(|number| *number += count).or_insert(count);
                }
            }
        }
    }

    (scores, distinct_scores)
}

// Parcours en largeur (BFS) pour explorer les sentiers depuis un trailhead
fn bfs_trailhead_score(map: &[Vec<u8>], start_y: usize, start_x: usize) -> (HashMap<(usize, usize), usize>, HashMap<(usize, usize), usize>) {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut reachable_nines = HashMap::new();
    let mut distinct_reachable_nines = HashMap::new();

    // Ajouter le point de départ à la file
    queue.push_back((start_y, start_x));
    visited.insert((start_y, start_x));

    while let Some((y, x)) = queue.pop_front() {
        let current_height = map[y][x];

        // Vérifie les positions voisines
        for (ny, nx) in neighbors(map, y, x) {

            if map[ny][nx] == current_height + 1
            {
                queue.push_back((ny, nx));
                if !visited.contains(&(ny, nx)) {
                    visited.insert((ny, nx));

                    // Si on atteint une position de hauteur 9, l'ajouter au set
                    if map[ny][nx] == 9 {
                        reachable_nines.entry((start_x, start_y)).and_modify(|number| *number += 1).or_insert(1);
                    }
                }
                if map[ny][nx] == 9 {
                    distinct_reachable_nines.entry((start_x, start_y)).and_modify(|number| *number += 1).or_insert(1);
                }
            }

        }
    }

    (reachable_nines, distinct_reachable_nines)
}

// Trouve les voisins valides (haut, bas, gauche, droite)
fn neighbors(map: &[Vec<u8>], y: usize, x: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let height = map.len();
    let width = map[0].len();

    if y > 0 {
        result.push((y - 1, x));
    }
    if y < height - 1 {
        result.push((y + 1, x));
    }
    if x > 0 {
        result.push((y, x - 1));
    }
    if x < width - 1 {
        result.push((y, x + 1));
    }

    result
}