use std::collections::HashSet;
use std::error::Error;
use std::fs::read_to_string;

const FILE_PATH: &str = "./inputs/aoc_2024/inputs_day6.txt";

// Directions possibles avec leurs décalages (dx, dy)
const DIRECTIONS: [(i32, i32); 4] = [
    (0, -1), // Haut (^)
    (1, 0),  // Droite (>)
    (0, 1),  // Bas (v)
    (-1, 0), // Gauche (<)
];

pub fn get_response() -> Result<(usize, i64), Box<dyn Error>> {
    let input = read_to_string(&FILE_PATH).expect("Unable to read file");

    Ok((simulate_guard_path(input.as_str()).len(), enigme2(input.as_str())))
}

fn simulate_guard_path(map: &str) -> HashSet<(i32, i32)> {
    // Charger la carte dans une grille
    let grid: Vec<Vec<char>> = map
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();

    // Trouver la position initiale et la direction du garde
    let mut guard_position = (0, 0);
    let mut guard_direction = 0; // 0: Haut, 1: Droite, 2: Bas, 3: Gauche

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if "^v<>".contains(cell) {
                guard_position = (x as i32, y as i32);
                guard_direction = match cell {
                    '^' => 0,
                    '>' => 1,
                    'v' => 2,
                    '<' => 3,
                    _ => unreachable!(),
                };
                //grid[y][x] = '.'; // Remplacer la position initiale par un chemin vide
                break;
            }
        }
    }

    // Simuler le déplacement
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    loop {
        // Ajouter la position actuelle aux positions visitées
        visited_positions.insert(guard_position);

        // Calculer la position devant le garde
        let (dx, dy) = DIRECTIONS[guard_direction];
        let next_position = (guard_position.0 + dx, guard_position.1 + dy);

        // Vérifier si le garde quitte la carte
        if next_position.0 < 0
            || next_position.1 < 0
            || next_position.0 >= cols
            || next_position.1 >= rows
        {
            break;
        }

        // Vérifier la case devant le garde
        let (nx, ny) = (next_position.0 as usize, next_position.1 as usize);
        if grid[ny][nx] == '#' {
            // Obstacle : tourner à droite
            guard_direction = (guard_direction + 1) % 4;
        } else {
            // Pas d'obstacle : avancer
            guard_position = next_position;
        }
    }

    visited_positions
}

fn enigme2(map: &str) -> i64 {
    let mut grid: Vec<Vec<char>> = map
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // Identifier la position initiale et la direction du garde
    let mut guard_position = (0, 0);
    let mut guard_direction = 0;
    let guard_path = simulate_guard_path(map);

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if "^v<>".contains(cell) {
                guard_position = (x as i32, y as i32);
                guard_direction = match cell {
                    '^' => 0,
                    '>' => 1,
                    'v' => 2,
                    '<' => 3,
                    _ => unreachable!(),
                };

                break;
            }
        }
    }

    let mut possible_positions = 0;

    // Tester chaque position vide pour une obstruction
    for (x, y) in guard_path {
        if grid[y as usize][x as usize] == '.' && (x as i32, y as i32) != guard_position {
            // Placer une obstruction temporaire
            grid[y as usize][x as usize] = '#';

            // Vérifier si cela crée un cycle
            if creates_cycle(&grid, guard_position, guard_direction) {
                possible_positions += 1;
            }

            // Retirer l'obstruction temporaire
            grid[y as usize][x as usize] = '.';
        }
    }

    possible_positions
}

fn creates_cycle(
    grid: &Vec<Vec<char>>,
    start_position: (i32, i32),
    start_direction: usize,
) -> bool {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut visited_states = HashSet::new();
    let mut current_position = start_position;
    let mut current_direction = start_direction;

    loop {
        // Enregistrer l'état actuel
        let state = (current_position, current_direction);
        if visited_states.contains(&state) {
            return true; // Cycle détecté
        }
        visited_states.insert(state);

        // Calculer la position devant le garde
        let (dx, dy) = DIRECTIONS[current_direction];
        let next_position = (current_position.0 + dx, current_position.1 + dy);

        // Vérifier si le garde quitte la carte
        if next_position.0 < 0
            || next_position.1 < 0
            || next_position.0 >= cols
            || next_position.1 >= rows
        {
            return false; // Pas de cycle, le garde sort de la carte
        }

        // Vérifier la case devant le garde
        let (nx, ny) = (next_position.0 as usize, next_position.1 as usize);
        if grid[ny][nx] == '#' {
            // Obstacle : tourner à droite
            current_direction = (current_direction + 1) % 4;
        } else {
            // Pas d'obstacle : avancer
            current_position = next_position;
        }
    }
}