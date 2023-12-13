use std::io;
use io::Result;
use std::fs::read_to_string;
use std::str::Lines;

const FILE_PATH: &str = "./inputs/aoc_2023/day8/inputs.txt";

#[derive(Debug)]
struct Node {
    name: String,
    left_node:String,
    right_node:String,
}
pub fn day8_main() -> Result<(u64, u64)> {
    let (instructions, network) = parse_file(&read_to_string(FILE_PATH)?.lines());

    let mut node_index = find_node(&network, "AAA");
    let mut step = 0;
    loop {
       for instruction in &instructions {
           match instruction {
               'L' => {node_index = find_node(&network, &network[node_index].left_node)},
               'R' => {node_index = find_node(&network, &network[node_index].right_node)},
               _ => {}
           }

           step += 1;
           if network[node_index].name.eq("ZZZ") {break;}
       }
        if network[node_index].name.eq("ZZZ") {break;}
    }

    Ok((step, 42))
}

fn parse_file(lines:&Lines) -> (Vec<char>, Vec<Node>){
    let mut instructions = vec![];
    let mut network = vec![];

    for (indx, line) in lines.clone().enumerate() {
        match indx {
            0 => {instructions = line.chars().collect::<Vec<char>>()}
            _ => {
                if line.eq("") {continue;}
                let (node_name, next_element) = parse_line(line);
                network.push(Node{name: node_name, left_node: next_element[0].clone(), right_node: next_element[1].clone()});
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

fn find_node(network:&Vec<Node>, node_name:&str) -> usize {
    let mut node_index = 0;
    for (index, node) in network.iter().enumerate() {
        if node.name.eq(node_name) {
            node_index = index;
            break;
        }
    }

    node_index
}