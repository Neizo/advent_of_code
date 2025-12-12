use minilp::{ComparisonOp, LinearExpr, OptimizationDirection, Problem, Variable};
use crate::utils::utils_files::{afficher_resultats, mesurer};

//const FILE_PATH_TEST: &str = "./inputs/aoc_2025/day10/inputs_test.txt";
const FILE_PATH_E1: &str = "./inputs/aoc_2025/day10/inputs_e1.txt";

#[derive(Debug, Clone)]
struct Machine {
    target: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<i32>,
}


fn parse_input(_file_path: &str) -> Vec<Machine> {
    let input = std::fs::read_to_string(_file_path).expect("Failed to read file");
    let mut machines = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Parse état cible [.##.#.]
        let start = line.find('[').unwrap();
        let end = line.find(']').unwrap();
        let target_str = &line[start + 1..end];
        let target: Vec<bool> = target_str.chars().map(|c| c == '#').collect();

        // Parse boutons (0,1,2) (1,3) etc.
        let buttons_part = &line[end + 1..];
        let mut buttons = Vec::new();

        let mut in_paren = false;
        let mut current = String::new();

        for ch in buttons_part.chars() {
            if ch == '(' {
                in_paren = true;
                current.clear();
            } else if ch == ')' {
                in_paren = false;
                if !current.is_empty() {
                    let button: Vec<usize> = current
                        .split(',')
                        .filter(|s| !s.is_empty())
                        .map(|s| s.trim().parse().unwrap())
                        .collect();
                    buttons.push(button);
                }
            } else if ch == '{' {
                break; // Ignorer les exigences de voltage
            } else if in_paren {
                current.push(ch);
            }
        }

        let joltage_start = line.find('{').unwrap();
        let joltage_end = line.find('}').unwrap();
        let joltage_str = &line[joltage_start + 1..joltage_end];
        let joltages: Vec<i32> = joltage_str
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();

        machines.push(Machine { target, buttons, joltages });
    }

    machines
}

pub fn get_response() {
    let (enigme1_result, time_e1) = mesurer(enigme1);
    let (enigme2_result, time_e2) = mesurer(enigme2);

    afficher_resultats(10, enigme1_result, time_e1, enigme2_result, time_e2);
}

/// Résout une machine en utilisant l'élimination de Gauss en GF(2)
/// avec exploration des variables libres pour trouver le minimum
/// Voir ficiher inptus/day10/explication.pdf pour plus d'information
fn solve_machine(machine: &Machine) -> Option<usize> {
    let n_lights = machine.target.len();
    let n_buttons = machine.buttons.len();

    // Construire la matrice augmentée [A | b]
    let mut matrix: Vec<Vec<bool>> = vec![vec![false; n_buttons + 1]; n_lights];

    // Remplir la matrice
    for (button_idx, button) in machine.buttons.iter().enumerate() {
        for &light in button {
            if light < n_lights {
                matrix[light][button_idx] = true;
            }
        }
    }

    // Définir la colonne cible
    for (i, &target) in machine.target.iter().enumerate() {
        matrix[i][n_buttons] = target;
    }

    // Élimination de Gauss en GF(2)
    let mut pivot_col = vec![None; n_lights];
    let mut pivot_row_for_col = vec![None; n_buttons];
    let mut col = 0;
    let mut row = 0;

    while row < n_lights && col < n_buttons {
        // Trouver le pivot
        let mut pivot_row = None;
        for r in row..n_lights {
            if matrix[r][col] {
                pivot_row = Some(r);
                break;
            }
        }

        if pivot_row.is_none() {
            col += 1;
            continue;
        }

        let pivot_row = pivot_row.unwrap();

        // Échanger les lignes
        if pivot_row != row {
            matrix.swap(pivot_row, row);
        }

        pivot_col[row] = Some(col);
        pivot_row_for_col[col] = Some(row);

        // Éliminer
        for r in 0..n_lights {
            if r != row && matrix[r][col] {
                for c in 0..=n_buttons {
                    matrix[r][c] ^= matrix[row][c];
                }
            }
        }

        col += 1;
        row += 1;
    }

    // Vérifier l'incohérence
    for r in row..n_lights {
        if matrix[r][n_buttons] {
            return None; // Système incohérent
        }
    }

    // Trouver les variables de base et libres
    let free_vars: Vec<usize> = (0..n_buttons)
        .filter(|i| pivot_row_for_col[*i].is_none())
        .collect();

    // Si pas de variables libres, une seule solution
    if free_vars.is_empty() {
        let mut solution = vec![false; n_buttons];
        for r in 0..n_lights {
            if let Some(c) = pivot_col[r] {
                solution[c] = matrix[r][n_buttons];
            }
        }
        return Some(solution.iter().filter(|&&x| x).count());
    }

    // Essayer toutes les combinaisons de variables libres pour trouver le minimum
    let mut min_presses = usize::MAX;
    let num_free = free_vars.len();

    for mask in 0..(1 << num_free) {
        let mut solution = vec![false; n_buttons];

        // Définir les variables libres selon le masque
        for (i, &var) in free_vars.iter().enumerate() {
            solution[var] = (mask & (1 << i)) != 0;
        }

        // Calculer les variables de base avec substitution arrière
        for r in (0..n_lights).rev() {
            if let Some(c) = pivot_col[r] {
                let mut val = matrix[r][n_buttons];
                for other_c in (c + 1)..n_buttons {
                    if matrix[r][other_c] {
                        val ^= solution[other_c];
                    }
                }
                solution[c] = val;
            }
        }

        // Compter les pressions
        let presses = solution.iter().filter(|&&x| x).count();
        min_presses = min_presses.min(presses);
    }

    Some(min_presses)
}

pub fn enigme1() -> usize {
    let machines = parse_input(FILE_PATH_E1);
    let mut total = 0;
    for (_, machine) in machines.iter().enumerate() {
        match solve_machine(machine) {
            Some(presses) => {
                total += presses;
            }
            None => {}
        }
    }

    total
}

/// Part 2: Résout le problème de voltage avec programmation linéaire en entiers
fn solve_part2(machine: &Machine) -> Option<usize> {
    let n_counters = machine.joltages.len();
    let n_buttons = machine.buttons.len();

    let mut problem = Problem::new(OptimizationDirection::Minimize);

    // Créer les variables (nombre de pressions par bouton)
    let button_vars: Vec<Variable> = (0..n_buttons)
        .map(|_| problem.add_var(1.0, (0.0, f64::INFINITY)))
        .collect();

    // Ajouter les contraintes pour chaque compteur
    for counter_idx in 0..n_counters {
        let mut expr = LinearExpr::empty();

        for (button_idx, button) in machine.buttons.iter().enumerate() {
            if button.contains(&counter_idx) {
                expr.add(button_vars[button_idx], 1.0);
            }
        }

        problem.add_constraint(
            expr,
            ComparisonOp::Eq,
            machine.joltages[counter_idx] as f64,
        );
    }

    // Résoudre
    match problem.solve() {
        Ok(solution) => {
            let total: f64 = button_vars
                .iter()
                .map(|&var| solution[var])
                .sum();
            Some(total.round() as usize)
        }
        Err(_) => None,
    }
}

pub fn enigme2() -> usize {
    let machines = parse_input(FILE_PATH_E1);

    let mut total_part2 = 0;
    for (_, machine) in machines.iter().enumerate() {
        match solve_part2(machine) {
            Some(presses) => {
                total_part2 += presses;
            }
            None => {}
        }
    }

    total_part2
}
