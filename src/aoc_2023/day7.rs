use std::io;
use io::Result;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::Lines;

const FILE_PATH: &str = "./inputs/aoc_2023/day7/inputs.txt";

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<char>,
    bids: u64,
    strength: StrengthRank
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
enum StrengthRank {
    HighCard = 1,
    OnePair = 2,
    TwoPairs = 3,
    ThreeOfAKind = 4,
    Full = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

pub fn day7_main() -> Result<(u64, u64)> {
    let mut hands = parse_file(&read_to_string(FILE_PATH)?.lines());
    let mut sums = vec![];

    for part1 in [true, false] {
        for (indx, _) in hands.clone().iter().enumerate() {
            hands[indx].strength = get_rank(&hands[indx].cards, part1);
        }

        hands.sort_unstable_by_key(|hand1| (hand1.strength, build_cards_value(&hand1.cards, part1)));
        sums.push(hands.iter().enumerate().map(|(indx, hand)| hand.bids * (indx as u64 + 1)).sum());
    }

    Ok((sums[0], sums[1]))
}

fn parse_file(lines:&Lines) -> Vec<Hand>{
    let split_line:Vec<Vec<&str>> = lines.clone().map(|line| line.split_whitespace().collect()).collect();

    split_line.iter().map(|line| Hand {cards:line.first().unwrap().chars().map(|value| value).collect(), bids:line.last().unwrap().parse::<u64>().unwrap(), strength: StrengthRank::HighCard}).collect::<Vec<Hand>>()
}

fn get_rank(cards:&Vec<char>, _part1:bool) -> StrengthRank {
    let mut hashmap = HashMap::new();

    for card in cards {
        hashmap.entry(card).and_modify(|number| *number += 1).or_insert(1);
    }

    if _part1 == false {hashmap = replace_joker(hashmap);}

    let mut values = hashmap.clone().into_values().collect::<Vec<i32>>();

    values.sort();
    values.reverse();

    let rank = match values[0] {
        5 => StrengthRank::FiveOfAKind,
        4 => StrengthRank::FourOfAKind,
        3 => { if values.len() > 1 && values[1] == 2 { StrengthRank::Full } else { StrengthRank::ThreeOfAKind } },
        2 => { if values.len() > 1 && values[1] == 2 { return StrengthRank::TwoPairs } else { StrengthRank::OnePair} },
        1 => StrengthRank::HighCard,
        _ => StrengthRank::HighCard,
    };

    rank
}

fn build_cards_value(cards:&Vec<char>, _part1:bool) -> Vec<u32> {
    let mut values = vec![];

    for card in cards {
        match card.is_digit(10) {
            true => {values.push(card.to_digit(10).unwrap())}
            false => {
                match card {
                    'T' => {values.push(10)},
                    'J' => {if _part1 {values.push(11)} else {values.push(1)}},
                    'Q' => {values.push(12)},
                    'K' => {values.push(14)},
                    'A' => {values.push(15)},
                    _ => {}
                }
            }
        }
    }

    values
}

fn replace_joker(cards:HashMap<&char, i32>) -> HashMap<&char, i32> {
    if cards.len() <= 1 || cards.get(&'J').is_some() == false {return cards.clone()}

    let mut lcl_cards = cards.clone();
    let nb_joker =cards.get(&'J').unwrap();
    lcl_cards.remove(&'J');

    let mut max_nb_entry = 0;
    let mut letter = &' ';
    for (card, nb_entry) in lcl_cards.clone() {
        if max_nb_entry < nb_entry {
            max_nb_entry = nb_entry;
            letter = card
        }
    }

    lcl_cards.entry(letter).and_modify(|number| *number += nb_joker);

    lcl_cards.clone()
}