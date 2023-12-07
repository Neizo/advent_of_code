use std::fs::read_to_string;
use std::io;

const FILE_PATH: &str = "./inputs/aoc_2023/day2/inputs_step2.txt";

#[derive(Debug)]
struct LineContent {
    game_id: u32,
    cubes: Vec<Cubes>
}

#[derive(Debug)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32
}

pub fn day2_main() -> io::Result<(u32, u32)> {
    let mut sum_step1 = 0;
    let mut sum_step2 = 0;
    let game_constitution = Cubes{red: 12, green: 13, blue: 14};

    for line in read_to_string(FILE_PATH)?.lines() {
        let line_content = parse_line(line)?;

        let bag_not_possible:Vec<bool> = line_content.cubes.iter().map(|cube| cube.red <= game_constitution.red && cube.green <= game_constitution.green && cube.blue <= game_constitution.blue).collect::<Vec<bool>>()
            .iter().filter(|value | **value == false).map(|value| *value).collect();

        if bag_not_possible.is_empty() {
            sum_step1 += line_content.game_id;
        }

        sum_step2 += line_content.cubes.iter().map(|cube| cube.red).max().unwrap() *
                           line_content.cubes.iter().map(|cube| cube.green).max().unwrap() *
                           line_content.cubes.iter().map(|cube| cube.blue).max().unwrap()
    }

    Ok((sum_step1, sum_step2))
}

fn parse_line(a_line:&str) -> io::Result<LineContent> {
    let mut cubes = vec![];
    let first_split: Vec<&str> = a_line.split(':').collect();
    let str_game_id = first_split.first().unwrap();
    let str_content = first_split.last().unwrap();

    let game_id = str_game_id.split(' ').last().unwrap().parse::<u32>().unwrap();
    let str_cubes_bag = str_content.split(';');

    for cubes_subset in str_cubes_bag {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let cubes_by_color = cubes_subset.split(',');

        for cube in cubes_by_color {
            let mut cube_split: Vec<&str> = cube.split(' ').collect();
            cube_split.remove(0);

            let number = cube_split.first().unwrap().parse::<u32>().unwrap();
            let color = cube_split.last().unwrap();

            if color == &"red" {red = number;}
            if color == &"green" {green = number;}
            if color == &"blue" {blue = number;}
        }

        cubes.push(Cubes{red, green, blue});
    }

    Ok(LineContent{game_id, cubes})
}