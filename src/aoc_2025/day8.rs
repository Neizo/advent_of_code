use crate::utils::utils_files::{afficher_resultats, mesurer};

//const FILE_PATH_TEST: &str = "./inputs/aoc_2025/day8/inputs_test.txt";
const FILE_PATH_E1: &str = "./inputs/aoc_2025/day8/inputs_e1.txt";

struct Node {
    parent: usize,
    size: usize,
}

fn parse_input(_file_path: &str) -> (Vec<Vec<usize>>, Vec<(usize, usize, usize)>) {
    let content = std::fs::read_to_string(_file_path)
        .expect("Failed to read file");

    let boxes: Vec<Vec<usize>> = content
        .lines()
        .map(|line| line.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>())
        .collect();

    let mut pairs = Vec::with_capacity(boxes.len() * (boxes.len() - 1));
    for (i, v1) in boxes.iter().enumerate() {
        for (j, v2) in boxes.iter().enumerate().skip(i + 1) {
            let dx = v1[0].abs_diff(v2[0]);
            let dy = v1[1].abs_diff(v2[1]);
            let dz = v1[2].abs_diff(v2[2]);
            let distance = dx * dx + dy * dy + dz * dz;
            pairs.push((i, j, distance));
        }
    }

    pairs.sort_unstable_by_key(|&(.., distance)| distance);

    (boxes, pairs)
}

pub fn get_response() {
    let (enigme1_result, time_e1) = mesurer(enigme1);
    let (enigme2_result, time_e2) = mesurer(enigme2);

    afficher_resultats(8, enigme1_result, time_e1, enigme2_result, time_e2);
}

fn find(nodes: &mut [Node], mut x: usize) -> usize {
    while nodes[x].parent != x {
        let parent = nodes[x].parent;
        (x, nodes[x].parent) = (parent, nodes[parent].parent);
    }

    x
}

fn union(nodes: &mut [Node], mut x: usize, mut y: usize) -> usize {
    x = find(nodes, x);
    y = find(nodes, y);

    if x != y {
        if nodes[x].size < nodes[y].size {
            (x, y) = (y, x);
        }

        nodes[y].parent = x;
        nodes[x].size += nodes[y].size;
    }

    nodes[x].size
}

pub fn enigme1() -> usize {
    let (boxes, pairs) = parse_input(FILE_PATH_E1);
    let mut nodes: Vec<_> = (0..boxes.len()).map(|i| Node { parent: i, size: 1 }).collect();

    for &(i, j, ..) in pairs.iter().take(1000) {
        union(&mut nodes, i, j);
    }

    nodes.sort_unstable_by_key(|node| node.size);
    nodes.iter().rev().take(3).map(|node| node.size).product()
}

pub fn enigme2() -> usize {
    let (boxes, pairs) = parse_input(FILE_PATH_E1);
    let mut nodes: Vec<_> = (0..boxes.len()).map(|i| Node { parent: i, size: 1 }).collect();

    for (i, j, ..) in pairs {
        if union(&mut nodes, i, j) == boxes.len() {
            return boxes[i][0] * boxes[j][0];
        }
    }

    0
}
