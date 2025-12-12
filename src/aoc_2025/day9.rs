use num::abs;
use crate::utils::utils_files::{afficher_resultats, mesurer};

//const FILE_PATH_TEST: &str = "./inputs/aoc_2025/day9/inputs_test.txt";
const FILE_PATH_E1: &str = "./inputs/aoc_2025/day9/inputs_e1.txt";

fn parse_input(_file_path: &str) -> Vec<(i64, i64)> {
    let content = std::fs::read_to_string(_file_path)
        .expect("Failed to read file");

    content
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(',');
            Some((
                parts.next()?.trim().parse().ok()?,
                parts.next()?.trim().parse().ok()?
            ))
        })
        .collect()
}

pub fn get_response() {
    let (enigme1_result, time_e1) = mesurer(enigme1);
    let (enigme2_result, time_e2) = mesurer(enigme2);

    afficher_resultats(9, enigme1_result, time_e1, enigme2_result, time_e2);
}

pub fn enigme1() -> i64 {
    let coords = parse_input(FILE_PATH_E1);
    let mut area = Vec::new();

    for (idx, (col, row)) in coords.iter().enumerate() {
        for (col2, row2) in coords.iter().skip(idx) {
            let length = abs(col - col2) + 1;
            let width = abs(row - row2) + 1;
            area.push(length * width);
        }
    }

    *area.iter().max().unwrap()
}

fn compute_area(p1: &(i64, i64), p2: &(i64, i64)) -> i64 {
    let width = (p2.0 - p1.0).abs() + 1;
    let height = (p2.1 - p1.1).abs() + 1;
    width * height
}

fn bbox(p1: &(i64, i64), p2: &(i64, i64)) -> [i64; 4] {
    [
        p1.0.min(p2.0), // xmin
        p1.0.max(p2.0), // xmax
        p1.1.min(p2.1), // ymin
        p1.1.max(p2.1), // ymax
    ]
}

pub fn enigme2() -> i64 {
    let mut pos = parse_input(FILE_PATH_E1);

    // compute all the possible rectangles and sort by area descending
    let mut rectangles = Vec::with_capacity(pos.len() * pos.len());
    for i in 0..pos.len() {
        for j in 0..pos.len() {
            let area = compute_area(&pos[i], &pos[j]);
            rectangles.push((pos[i], pos[j], area));
        }
    }

    pos.push(pos[0]); // make it a closed polygon

    // pairs of points are lines, find the largest rectangle that doesn't intersect with a line
    let max_area = rectangles
        .iter()
        .map(|(p1, p2, area)| {
            let [bxmin, bxmax, bymin, bymax] = bbox(p1, p2);
            for w in pos.windows(2) {
                let [lxmin, lxmax, lymin, lymax] = bbox(&w[0], &w[1]);
                if lxmin == lxmax {
                    // is a vertical line
                    if lxmin > bxmin && lxmax < bxmax {
                        // vertical line is inside bbox vertical bounds
                        if !(lymax <= bymin || lymin >= bymax) {
                            // vertical line intersects with bbox
                            return None;
                        }
                    }
                }

                if lymin == lymax {
                    // is a horizontal line
                    if lymin > bymin && lymax < bymax {
                        // horizontal line is inside bbox horizontal bounds
                        if !(lxmax <= bxmin || lxmin >= bxmax) {
                            // horizontal line intersects with bbox
                            return None;
                        }
                    }
                }
            }

            Some(*area)
        })
        .filter_map(|area| area)
        .max().unwrap();

    max_area
}