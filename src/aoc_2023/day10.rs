use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::read_to_string;
use std::io;
use io::Result;

type Grid = Vec<Vec<char>>;
const FILE_PATH: &str = "./inputs/aoc_2023/day10/inputs.txt";
fn parse_input() -> Grid {
    read_to_string(FILE_PATH).unwrap().lines().map(|line| line.chars().collect()).collect()
}

// Map pour chaque type de tuyau et ses connexions
fn get_connections(tile: char) -> Vec<(isize, isize)> {
    match tile {
        '|' => vec![(-1, 0), (1, 0)], // Nord-Sud
        '-' => vec![(0, -1), (0, 1)], // Ouest-Est
        'L' => vec![(-1, 0), (0, 1)], // Nord-Est
        'J' => vec![(-1, 0), (0, -1)], // Nord-Ouest
        '7' => vec![(1, 0), (0, -1)], // Sud-Ouest
        'F' => vec![(1, 0), (0, 1)], // Sud-Est
        'S' => vec![(0, -1), (0, 1)],
        '.' => vec![(0, 0), (0, 0)],
        _ => vec![], // Pas de connexion
    }
}

// Vérifie si deux tuiles sont connectées correctement
fn are_connected(dir1: (isize, isize), tile2: char) -> bool {
    let opposite_dir = (-dir1.0, -dir1.1);
    get_connections(tile2).contains(&opposite_dir)
}

fn find_start(grid: &Grid) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                return (i, j);
            }
        }
    }
    panic!("Position de départ introuvable !");
}

fn find_loop(grid: &Grid) -> HashSet<(usize, usize)> {
    let start = find_start(grid);
    let mut visited = HashSet::new();
    let mut stack = vec![start];

    while let Some((x, y)) = stack.pop() {
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        let tile = grid[x][y];
        for &(dx, dy) in &get_connections(tile) {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 {
                let nx = nx as usize;
                let ny = ny as usize;

                if let Some(&next_tile) = grid.get(nx).and_then(|row| row.get(ny)) {
                    if next_tile != '.' && are_connected((dx, dy), next_tile) {
                        stack.push((nx, ny));
                    }
                }
            }
        }
    }

    visited
}

fn flood_fill(grid: &Vec<Vec<char>>, loop_tiles: &HashSet<(usize, usize)>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    // Définition des connexions pour chaque type de tuyau
    let pipe_connections: HashMap<char, Vec<((isize, isize), (isize, isize))>> = [
        ('|', vec![((-1, 0), (1, 0))]),    // Connecte nord-sud
        ('-', vec![((0, -1), (0, 1))]),      // Connecte ouest-est
        ('L', vec![((0, -1), (-1, 0))]),   // Connecte ouest-nord
        ('7', vec![((1, 0), (0, -1))]),    // Connecte sud-ouest
        ('J', vec![((1, 0), (0, 1))]),     // Connecte sud-est
        ('F', vec![((0, 1), (-1, 0))]),    // Connecte nord-est
        ('S', vec![((0, -1), (0, 1))])
    ]
        .iter()
        .cloned()
        .collect();

    // Ajouter les bords à la file d'attente
    for i in 0..rows {
        queue.push_back((i, 0));
        queue.push_back((i, cols - 1));
    }
    for j in 0..cols {
        queue.push_back((0, j));
        queue.push_back((rows - 1, j));
    }

    // Flood fill pour marquer les tuiles extérieures
    while let Some((x, y)) = queue.pop_front() {
        if x >= rows || y >= cols || visited.contains(&(x, y)) {
            continue;
        }

        // Vérifier si c'est une tuile valide à visiter
        if grid[x][y] != '.' && !pipe_connections.contains_key(&grid[x][y]) && !loop_tiles.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));

        // Explorer les voisins
        if grid[x][y] == '.' {
            for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && ny >= 0 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if nx < rows && ny < cols && !visited.contains(&(nx, ny)) {
                        queue.push_back((nx, ny));
                    }
                }
            }
        } else if let Some(connections) = pipe_connections.get(&grid[x][y]) {
            for &((dx1, dy1), (dx2, dy2)) in connections {
                let nx1 = x as isize + dx1;
                let ny1 = y as isize + dy1;
                let nx2 = x as isize + dx2;
                let ny2 = y as isize + dy2;

                for &(nx, ny) in &[(nx1, ny1), (nx2, ny2)] {
                    if nx >= 0 && ny >= 0 {
                        let nx = nx as usize;
                        let ny = ny as usize;
                        if nx < rows && ny < cols && !visited.contains(&(nx, ny)) {
                            queue.push_back((nx, ny));
                        }
                    }
                }
            }
        }
    }

    // Compter les tuiles enfermées
    let mut enclosed_count = 0;
    for x in 0..rows {
        for y in 0..cols {
            if grid[x][y] == '.' && !visited.contains(&(x, y)) && !loop_tiles.contains(&(x, y)) {
                enclosed_count += 1;
            }
        }
    }

    enclosed_count
}

fn max_distance(grid: &Grid, loop_tiles: &HashSet<(usize, usize)>) -> usize {
    let start = find_start(grid);
    let mut distances = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    let mut max_dist = 0;

    while let Some(((x, y), dist)) = queue.pop_front() {
        if distances.contains(&(x, y)) {
            continue;
        }
        distances.insert((x, y));
        max_dist = max_dist.max(dist);

        let tile = grid[x][y];
        for &(dx, dy) in &get_connections(tile) {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 {
                let nx = nx as usize;
                let ny = ny as usize;

                if loop_tiles.contains(&(nx, ny)) {
                    queue.push_back(((nx, ny), dist + 1));
                }
            }
        }
    }

    max_dist
}

pub fn day10_main() -> Result<(usize, usize)> {
    let grid = parse_input();
    let loop_tiles = find_loop(&grid);
    let enclosed_area = flood_fill(&grid, &loop_tiles);
    let result = max_distance(&grid, &loop_tiles);

    Ok((result, enclosed_area))
}
