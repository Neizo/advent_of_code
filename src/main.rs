use std::env;
use crate::aoc_2023::resous_annee_2023;

mod aoc_2023;
mod aoc_2024;

fn main() {
    let year = get_year();

    match year {
        None => {}
        Some(year) => {
            match year.as_str() {
                "2023" => {resous_annee_2023()}
                &_ => {
                    println!("year {:?} not handle", year)
                }
            }
        }
    }
}

fn get_year() -> Option<String>{
    let args: Vec<String> = env::args().collect();

    for cpt_arg in 0..args.len() {
        match args[cpt_arg].as_str() {
            "year" => {
                if args.len()-1 < cpt_arg + 1 {
                    println!("Years not specified");
                    return None;
                }

                return Some(args[cpt_arg+1].clone());
            },
            _ => {}
        }
    }

    None
}