use std::collections::{HashMap, HashSet};
use crate::utils::utils_files::{afficher_resultats, mesurer};

const FILE_PATH_TEST: &str = "./inputs/aoc_2025/day11/inputs_test.txt";
const FILE_PATH_E1: &str = "./inputs/aoc_2025/day11/inputs_e1.txt";

/// Structure pour représenter un ensemble de nœuds visités (trié)
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct VisitedSet {
    nodes: Vec<String>,  // Toujours trié
}

impl VisitedSet {
    fn new() -> Self {
        VisitedSet { nodes: Vec::new() }
    }

    fn with(node: &str) -> Self {
        VisitedSet {
            nodes: vec![node.to_string()],
        }
    }

    fn merge(&self, other: &VisitedSet) -> Self {
        let mut merged: HashSet<String> = self.nodes.iter().cloned().collect();
        merged.extend(other.nodes.iter().cloned());

        let mut nodes: Vec<String> = merged.into_iter().collect();
        nodes.sort();

        VisitedSet { nodes }
    }

    fn contains_all(&self, required: &[String]) -> bool {
        required.iter().all(|req| self.nodes.contains(req))
    }
}

fn parse_input(_file_path: &str) -> HashMap<String, Vec<String>> {
    let input = std::fs::read_to_string(_file_path).expect("Failed to read file");
    let mut graph = HashMap::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let key = parts[0].trim().to_string();
        let values: Vec<String> = parts[1]
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        graph.insert(key, values);
    }

    graph
}

pub fn get_response() {
    let (enigme1_result, time_e1) = mesurer(enigme1);
    let (enigme2_result, time_e2) = mesurer(enigme2);

    afficher_resultats(11, enigme1_result, time_e1, enigme2_result, time_e2);
}

/// Calcule le nombre de chemins qui mènent à "out" depuis chaque nœud
/// en utilisant la programmation dynamique avec détection de cycles
fn count_paths_to_out(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, i64>,
    visiting: &mut HashSet<String>,
    visited: &mut HashSet<String>,
) -> i64 {
    // Si déjà calculé, retourner le résultat
    if let Some(&count) = memo.get(node) {
        return count;
    }

    // Détection de cycle : si on est en train de visiter ce nœud
    if visiting.contains(node) {
        return 0; // Un cycle ne mène pas à "out"
    }

    // Cas de base : si le nœud est "out"
    if node == "out" {
        memo.insert(node.to_string(), 1);
        return 1;
    }

    // Si le nœud n'a pas de dépendances
    if !graph.contains_key(node) {
        memo.insert(node.to_string(), 0);
        return 0;
    }

    // Marquer ce nœud comme en cours de visite
    visiting.insert(node.to_string());
    visited.insert(node.to_string());

    // Calculer récursivement le nombre de chemins
    let mut total_paths: i64 = 0;
    if let Some(deps) = graph.get(node) {
        for dep in deps {
            let paths = count_paths_to_out(dep, graph, memo, visiting, visited);
            total_paths = total_paths.saturating_add(paths); // Éviter l'overflow
        }
    }

    // Retirer le nœud de la liste des nœuds en visite
    visiting.remove(node);

    memo.insert(node.to_string(), total_paths);
    total_paths
}

pub fn enigme1() -> i64 {
    let graph = parse_input(FILE_PATH_E1);

    // Calculer le nombre de chemins pour chaque nœud
    let mut path_counts: HashMap<String, i64> = HashMap::new();
    let mut visiting: HashSet<String> = HashSet::new();
    let mut visited: HashSet<String> = HashSet::new();

    for node in graph.keys() {
        count_paths_to_out(node, &graph, &mut path_counts, &mut visiting, &mut visited);
    }

    *path_counts.get("you").unwrap()
}

type PathsByVisited = HashMap<VisitedSet, i64>;

fn count_paths_with_tracking(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    required_nodes: &[String],
    memo: &mut HashMap<String, PathsByVisited>,
    visiting: &mut HashSet<String>,
) -> PathsByVisited {
    // Si déjà calculé
    if let Some(result) = memo.get(node) {
        return result.clone();
    }

    // Détection de cycle
    if visiting.contains(node) {
        return PathsByVisited::new();
    }

    // Cas de base : "out"
    if node == "out" {
        let mut result = PathsByVisited::new();
        result.insert(VisitedSet::new(), 1);
        memo.insert(node.to_string(), result.clone());
        return result;
    }

    // Si pas de dépendances
    if !graph.contains_key(node) {
        let result = PathsByVisited::new();
        memo.insert(node.to_string(), result.clone());
        return result;
    }

    visiting.insert(node.to_string());

    let mut combined_result: PathsByVisited = HashMap::new();

    if let Some(deps) = graph.get(node) {
        for dep in deps {
            let dep_paths = count_paths_with_tracking(dep, graph, required_nodes, memo, visiting);

            for (visited_set, count) in dep_paths {
                let new_visited = if required_nodes.contains(&node.to_string()) {
                    visited_set.merge(&VisitedSet::with(node))
                } else {
                    visited_set
                };

                *combined_result.entry(new_visited).or_insert(0) += count;
            }
        }
    }

    visiting.remove(node);

    memo.insert(node.to_string(), combined_result.clone());
    combined_result
}

pub fn enigme2() -> i64 {
    let graph = parse_input(FILE_PATH_E1);

    let mut required_nodes = vec!["dac".to_string(), "fft".to_string()];
    required_nodes.sort();
    let mut memo: HashMap<String, PathsByVisited> = HashMap::new();
    let mut visiting: HashSet<String> = HashSet::new();

    let mut results: Vec<(String, i64, i64)> = Vec::new();

    for node in graph.keys() {
        if !memo.contains_key(node) {
            count_paths_with_tracking(node, &graph, &required_nodes, &mut memo, &mut visiting);
        }

        if let Some(paths) = memo.get(node) {
            let total: i64 = paths.values().sum();

            let with_all_required: i64 = paths.iter()
                .filter(|(visited, _)| visited.contains_all(&required_nodes))
                .map(|(_, count)| count)
                .sum();

            if with_all_required > 0 {
                results.push((node.clone(), with_all_required, total));
            }
        }
    }

    for (node, count_required, _) in results {
        if node.eq("svr") {return count_required;}
    }

    0
}
