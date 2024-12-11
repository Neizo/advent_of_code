use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fs::read_to_string;

/*
* Utilisation d'une HashMap, a la fin au lieux d'avoir des millions de pierre dans un vecteur on en a quelque millier, ce qui rend les boucles vachement plus rapide
 */

const FILE_PATH: &str = "./inputs/aoc_2024/inputs_day11.txt";

pub fn get_response() -> Result<(usize, usize), Box<dyn Error>> {
    let inputs = parse_file();

    Ok((enigme1(&inputs, 25), enigme1(&inputs, 75)))
}

pub fn parse_file() -> HashMap<u128, usize> {
    read_to_string(FILE_PATH).expect("Can't open file").split_whitespace().map(|value| (value.parse::<u128>().unwrap(), 1) ).collect::<HashMap<u128, usize>>()
}

// Compte les pierres après un certain nombre de clignements
fn enigme1(initial_stones: &HashMap<u128, usize>, blinks: usize) -> usize {
    let mut stones = initial_stones.clone();

    // Simulation des clignements
    for _ in 0..blinks {
        stones = update(&stones);
    }

    stones.values().sum()
}

fn update(old_stones: &HashMap<u128, usize>) -> HashMap<u128, usize> {
    let mut stones = HashMap::with_capacity(old_stones.len());
    for (&stone, &count_stone) in old_stones {
        match stone {
            0 => *stones.entry(1).or_default() += count_stone,
            _ => {
                let digits = stone.ilog10() + 1;
                if digits % 2 == 0 {
                    *stones.entry(stone % 10u128.pow(digits / 2)).or_default() += count_stone;
                    *stones.entry(stone / 10u128.pow(digits / 2)).or_default() += count_stone;
                } else {
                    *stones.entry(stone * 2024).or_default() += count_stone
                }
            }
        }
    }

    stones
}