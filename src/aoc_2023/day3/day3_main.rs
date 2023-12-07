use std::fs::read_to_string;
use std::io;
use std::num::ParseIntError;
use std::str::Chars;

const FILE_PATH: &str = "./inputs/aoc_2023/day3/inputs.txt";

pub fn day3_main() -> io::Result<(u32, u32)>{
    let file_content = read_to_string(FILE_PATH)?;
    let engine_schematic : Vec<Vec<char>> = file_content.lines().map(|line| line.chars().collect()).collect();
    let mut numbers = vec![];
    let mut gears = vec![];

    for (cpt_y, line) in engine_schematic.iter().enumerate() {
        for (cpt_x, element) in line.iter().enumerate() {
            if is_symbol(element) == false && element.eq(&'.') == false && element.to_digit(10).is_some() == false {println!("{} not a symbol", element)}
            if is_symbol(element) {
                match find_arround(&engine_schematic, cpt_x, cpt_y) {
                    Ok(data) => {
                        numbers.push(data.clone());
                        if element.eq(&'*') && data.len() == 2 {
                            gears.push(data.iter().product::<u32>());
                        }
                    }
                    Err(_) => {}
                }
            }
        }
    }

    Ok((numbers.iter().map(|sub_numbers| sub_numbers.iter().sum::<u32>()).sum(), gears.iter().sum()))
}

fn is_digit(engine_schematic:&Vec<Vec<char>>, index_x:usize, index_y:usize, offset_x:i32, offset_y:i32) -> bool {
    let index_y_lcl = index_y as i32 + offset_y as i32;
    let index_x_lcl = index_x as i32 + offset_x as i32;

    if index_y_lcl < 0 || index_x_lcl < 0 || index_y_lcl >= engine_schematic.len() as i32 || index_x_lcl >= engine_schematic[index_y_lcl as usize].len() as i32 {return false};

    engine_schematic[index_y_lcl as usize][index_x_lcl as usize].to_digit(10).is_some()
}

fn is_symbol(element:&char) -> bool {
    element.is_ascii_punctuation() && element.eq(&'.') == false
}

fn find_arround(engine_schematic:&Vec<Vec<char>>, index_x: usize, index_y:usize) -> Result<Vec<u32>, ParseIntError> {
    let mut numbers = vec![];
    let mut finded = false;

    for cpt_y in -1i32..=1 {
        finded = false;
        for cpt_x in -1i32..=1 {
            let is_digit = is_digit(&engine_schematic, index_x, index_y, cpt_x, cpt_y);
            if finded == false && is_digit{
                numbers.push(read_number(&engine_schematic[(index_y as i32 + cpt_y) as usize], index_x as i32 + cpt_x)?);
                finded = true;
            } else if is_digit == false{
                finded = false;
            }
        }
    }

    Ok(numbers)
}

fn read_number(line_engine_schematic:&Vec<char>, index_x:i32) -> Result<u32, ParseIntError> {
    let mut read_right = true;
    let mut read_left = true;
    let mut index_x1 = index_x;
    let mut index_x2 = index_x;

    while read_right || read_left {
        if index_x1-1 < 0 {read_left = false}
        if index_x2 == (line_engine_schematic.len()) as i32 {read_right = false}

        if read_left && line_engine_schematic[(index_x1-1) as usize].to_digit(10).is_some() { index_x1 -= 1} else { read_left = false}
        if read_right && line_engine_schematic[index_x2 as usize].to_digit(10).is_some(){ index_x2 +=1 } else { read_right = false}
    }

    line_engine_schematic[(index_x1 as usize)..(index_x2 as usize)].into_iter().collect::<String>().parse::<u32>()
}