use std::time::Instant;
use crate::aoc_2023::day1::day1_main;
use crate::aoc_2023::day2::day2_main;
use crate::aoc_2023::day3::day3_main;
use crate::aoc_2023::day4::day4_main;
use crate::aoc_2023::day5::day5_main;
use crate::aoc_2023::day6::day6_main;
use crate::aoc_2023::day8::day8_main;
use crate::aoc_2023::day9::day9_main;

mod aoc_2023;

fn main() {
    let start = Instant::now();

    println!("Day 1 : The answer is {:?}", day1_main().unwrap());
    println!("Day 2 : The answer is {:?}", day2_main().unwrap());
    println!("Day 3 : The answer is {:?}", day3_main().unwrap());
    println!("Day 4 : The answer is {:?}", day4_main().unwrap());
    println!("Day 5 : The answer is {:?}", day5_main().unwrap());
    println!("Day 6 : The answer is {:?}", day6_main().unwrap());
    println!("Day 8 : The answer is {:?}", day8_main().unwrap());
    println!("Day 9 : The answer is {:?}", day9_main().unwrap());

    let elapsed = start.elapsed();
    println!("{:.2?}", elapsed);
}