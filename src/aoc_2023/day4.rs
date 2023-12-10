use std::fs::read_to_string;
use std::io;
use io::Result;
use std::ops::Range;

const FILE_PATH: &str = "./inputs/aoc_2023/day4/inputs.txt";

pub fn day4_main() -> Result<(u32, u32)> {
    let mut winning_numbers_found = vec![];
    let file = read_to_string(FILE_PATH)?;
    let mut card_count = vec![0; file.lines().count()];

    for (indx, line) in file.lines().enumerate() {
        card_count[indx] += 1;

        let (winning_numbers, numbers) = parse_line(line)?;
        winning_numbers_found.push(winning_numbers.iter().filter_map(|winning_number| numbers.iter().position(|number| number == winning_number)).collect::<Vec<usize>>());

        let range_indx = Range {start: 1, end: winning_numbers_found.last().unwrap().len()+1};
        for index in range_indx {
            card_count[indx+index] += 1 * card_count[indx];
        }
    }

    winning_numbers_found.retain(|element| element.len() > 0);

    Ok((winning_numbers_found.iter().map(|array| 2u32.pow((array.len()-1) as u32)).sum(), card_count.iter().sum::<u32>()))
}

fn parse_line(line:&str) -> Result<(Vec<u32>, Vec<u32>)>{
    let cards_numbers: Vec<&str> = line.split(':').collect::<Vec<&str>>().last().unwrap().split('|').collect();

    Ok((extract_numbers(cards_numbers[0])?, extract_numbers(cards_numbers[1])?))
}

fn extract_numbers(card_numbers:&str) -> Result<Vec<u32>>{
    let mut numbers = vec![];

    //card_numbers.trim().split(' ').map(|number| number.parse::<u32>()).collect::<Vec<u32>>();

    for number in card_numbers.trim().split(' ') {
        if number == "" || number == " " {continue};
        numbers.push(number.parse::<u32>().unwrap())
    }

    Ok(numbers)
}

