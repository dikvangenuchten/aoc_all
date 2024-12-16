pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;

use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

pub fn solve_days() {
    println!("Day 01 {:?}", day_01::solve_day(&read_day_input("01")));
    println!("Day 02 {:?}", day_02::solve_day(&read_day_input("02")));
    println!("Day 03 {:?}", day_03::solve_day(&read_day_input("03")));
    println!("Day 04 {:?}", day_04::solve_day(&read_day_input("04")));
    println!("Day 05 {:?}", day_05::solve_day(&read_day_input("05")));
    println!("Day 06 {:?}", day_06::solve_day(&read_day_input("06")));
    println!("Day 07 {:?}", day_07::solve_day(&read_day_input("07")));
    println!("Day 08 {:?}", day_08::solve_day(&read_day_input("08")));
    println!("Day 09 {:?}", day_09::solve_day(&read_day_input("09")));
    println!("Day 10 {:?}", day_10::solve_day(&read_day_input("10")));
    println!("Day 11 {:?}", day_11::solve_day(&read_day_input("11")));
    println!("Day 12 {:?}", day_12::solve_day(&read_day_input("12")));
    println!("Day 13 {:?}", day_13::solve_day(&read_day_input("13")));
    println!("Day 14 {:?}", day_14::solve_day(&read_day_input("14")));
    println!("Day 15 {:?}", day_15::solve_day(&read_day_input("15")));
    println!("Day 16 {:?}", day_16::solve_day(&read_day_input("16")));
}

pub fn read_day_input(day: &str) -> String {
    let file_path = format!("inputs/{day}.txt");
    fs::read_to_string(&file_path).unwrap_or_else(|_| panic!("Could not load: {file_path}"))
}

pub fn read_test_day_input(day: &str) -> String {
    let file_path = format!("test_inputs/test_{day}.txt");
    fs::read_to_string(&file_path).unwrap_or_else(|_| panic!("Could not load: {file_path}"))
}

lazy_static! {
    static ref RE_DIGITS: Regex = Regex::new(r"(\d+)").unwrap();
}
