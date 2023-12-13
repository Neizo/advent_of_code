use std::io;
use io::Result;
use std::fs::read_to_string;
use std::str::Lines;

const FILE_PATH: &str = "./inputs/aoc_2023/day6/inputs.txt";

#[derive(Debug)]
struct Races {
    time: u64,
    distance: u64
}

pub fn day6_main() -> Result<(u64, u64)>{
    let races = parse_file(read_to_string(FILE_PATH)?.lines());

    Ok((part_1(&races), part_2(&races)))
}

fn parse_file(lines:Lines) -> Vec<Races>{
    let mut data = vec![];
    let mut races = vec![];

    for line in lines {
        data.push(extract_value(line).iter().map(|str_value| str_value.parse::<u64>().unwrap()).collect::<Vec<u64>>())
    }

    for indx in 0..data[0].len() {
        races.push(Races{time: data[0][indx], distance: data[1][indx]})
    }

    races
}

fn extract_value(line: &str) -> Vec<&str> {
    let str_value: Vec<&str> = line.split(":").last().unwrap().trim().split_whitespace().collect();

    str_value
}

fn part_1(races:&Vec<Races>) -> u64 {
    let simulation: Vec<u64> = races.iter().
        map(|race| (0..=race.time).map(|time_holding| (race.time-time_holding) * time_holding).collect::<Vec<u64>>()
            .iter().fold(0u64, |nb_winning, distance_simulation| {
            if distance_simulation > &race.distance {nb_winning + 1} else {nb_winning}
        })
        ).collect::<Vec<_>>();

    simulation.iter().fold(1, |value, simu| {value * simu})
}

fn part_2(races:&Vec<Races>) -> u64 {
    let mut races_e2=vec![];

    let (time, distance) = races.iter().rev().enumerate().fold((0, 0), |(time, distance), (_, race) | {
        (time + (race.time * 10u64.pow(time.to_string().chars().count() as u32)), distance + (race.distance * 10u64.pow(distance.to_string().chars().count() as u32)))
    });

    races_e2.push(Races{time: time/10, distance: distance/10});

    part_1(&races_e2)
}