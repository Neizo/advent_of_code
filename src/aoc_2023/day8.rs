use std::io;
use io::Result;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::Lines;
use num::*;

const FILE_PATH: &str = "./inputs/aoc_2023/day8/inputs.txt";

pub fn day8_main() -> Result<(u64, u64)> {
    let (instructions, network) = parse_file(&read_to_string(FILE_PATH)?.lines());

    Ok((part_1(&instructions, &network), part_2(&instructions, &network)))
}

fn part_1(instructions:&Vec<char>, network:&HashMap<String, (String, String)>) -> u64 {
    let mut node = "AAA".to_string();

    let mut step = 0;
    loop {
        for instruction in instructions {
            match instruction {
                'L' => {node = network[&node].0.to_string()},
                'R' => {node = network[&node].1.to_string()},
                _ => {}
            }

            step += 1;
            if node == "ZZZ" {break;}
        }
        if node == "ZZZ" {break;}
    }

    step
}

fn part_2(instructions:&Vec<char>, network:&HashMap<String, (String, String)>) -> u64 {
    let nodes: Vec<String> = network.keys().filter(|&node_name| node_name.ends_with('A')).cloned().collect();
    let mut counts = vec![];

    for mut node in nodes {
        let mut step = 0i64;
        while node.ends_with('Z') == false {
            for instruction in instructions {
                match instruction {
                    'L' => {node = network[&node].0.to_string()},
                    'R' => {node = network[&node].1.to_string()},
                    _ => {}
                }

                step += 1;
                if node.ends_with('Z') {break;}
            }
        }

        counts.push(step);
    }

    counts.into_iter().reduce(|a, b| a.lcm(&b)).unwrap() as u64
}

fn parse_file(lines:&Lines) -> (Vec<char>, HashMap<String, (String, String)>){
    let mut instructions = vec![];
    let mut network = HashMap::new();

    for (indx, line) in lines.clone().enumerate() {
        match indx {
            0 => {instructions = line.chars().collect::<Vec<char>>()}
            _ => {
                if line.eq("") {continue;}
                let (node_name, next_element) = parse_line(line);
                network.insert(node_name, (next_element[0].clone(), next_element[1].clone()));
            }
        }
    }

    (instructions, network)
}

fn parse_line(line:&str) -> (String, Vec<String>) {
    let split: Vec<&str> = line.split('=').collect();
    let node_name = split.first().unwrap().trim();
    let next_elements_tmp = split.last().unwrap().replace('(', "").replace(')', "").replace(' ', "");
    let next_elements = next_elements_tmp.split(',').map(|element| element.to_string()).collect();

    (node_name.to_string(), next_elements)
}