mod aoc_2023;
use crate::aoc_2023::day1::day1_main::day1_main;
use crate::aoc_2023::day2::day2_main::day2_main;
use crate::aoc_2023::day3::day3_main::day3_main;

fn main() {
    println!("Day 1 : The answer is {:?}", day1_main().unwrap());
    println!("Day 2 : The answer is {:?}", day2_main().unwrap());
    println!("Day 3 : The answer is {:?}", day3_main().unwrap());
}

/*
Day 1 : The answer is 54473
Day 2 : The answer is (2505, 70265)
Day 3 : The answer is (525119, 76504829)
 */