use std::ops::RangeInclusive;
use crate::utils::utils_files::{afficher_resultats, mesurer};

//const FILE_PATH_TEST: &str = "./inputs/aoc_2025/day5/inputs_test.txt";
const FILE_PATH_E1: &str = "./inputs/aoc_2025/day5/inputs_e1.txt";
//const FILE_PATH_TEST: &str = "./inputs/aoc_2025/day5/inputs_test.txt";

fn parse_input(_file_path: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let content = std::fs::read_to_string(_file_path)
        .expect("Failed to read file");

    let ranges: Vec<RangeInclusive<usize>> = content
        .lines()
        .by_ref()
        .take_while(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            let start = parts[0].parse::<usize>().unwrap();
            let end = parts[1].parse::<usize>().unwrap();
            start..=end
        })
        .collect();

    let ids: Vec<usize> = content
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.contains("-"))
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect();

    (ranges, ids)
}

pub fn get_response() {
    let (enigme1_result, time_e1) = mesurer(enigme1);
    let (enigme2_result, time_e2) = mesurer(enigme2);

    afficher_resultats(5, enigme1_result, time_e1, enigme2_result,time_e2);
}

pub fn enigme1() -> usize {
    let (ranges, ids) = parse_input(FILE_PATH_E1);

    ids.iter()
        .filter(|id| ranges.iter().any(|range| range.contains(id)))
        .count()
}

fn merge_overlaps(ranges: Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    if ranges.is_empty() {
        return vec![];
    }

    // Trier les ranges par début
    let mut sorted_ranges = ranges;
    sorted_ranges.sort_by_key(|r| *r.start());

    let mut result = Vec::new();
    let mut current = sorted_ranges[0].clone();

    for next in sorted_ranges.into_iter().skip(1) {
        // Vérifier si les ranges se chevauchent ou sont adjacents
        if *current.end() >= *next.start() - 1 {
            // Fusionner : étendre current jusqu'au max des deux fins
            current = *current.start()..=(*current.end().max(next.end()));
        } else {
            // Pas de chevauchement : ajouter current et passer à next
            result.push(current);
            current = next;
        }
    }

    // Ajouter le dernier range
    result.push(current);

    result
}

pub fn enigme2() -> usize {
    let (ranges, _) = parse_input(FILE_PATH_E1);
    let merged_ranges =  merge_overlaps(ranges);

    merged_ranges.iter().map(|range| range.end() - range.start() + 1).collect::<Vec<usize>>().iter().sum::<usize>()
}
