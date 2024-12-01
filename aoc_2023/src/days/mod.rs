use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

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
pub mod day_17;

pub fn solve_all() -> Vec<(u128, u128)> {
    vec![
        convert(day_01::solve_day(&read_day_input("day_01"))),
        convert(day_02::solve_day(&read_day_input("day_02"))),
        convert(day_03::solve_day(&read_day_input("day_03"))),
        convert(day_04::solve_day(&read_day_input("day_04"))),
        convert(day_05::solve_day(&read_day_input("day_05"))),
        convert(day_06::solve_day(&read_day_input("day_06"))),
        convert(day_07::solve_day(&read_day_input("day_07"))),
        convert(day_08::solve_day(&read_day_input("day_08"))),
        convert(day_09::solve_day(&read_day_input("day_09"))),
        convert(day_10::solve_day(&read_day_input("day_10"))),
        convert(day_11::solve_day(&read_day_input("day_11"))),
        // convert(day_12::solve_day(&read_day_input("day_12"))), // Uncomment as it is verry slow
        convert(day_13::solve_day(&read_day_input("day_13"))),
        convert(day_14::solve_day(&read_day_input("day_14"))),
        convert(day_15::solve_day(&read_day_input("day_15"))),
        convert(day_16::solve_day(&read_day_input("day_16"))),
        convert(day_17::solve_day(&read_day_input("day_17"))),
    ]
}

fn convert<A, B>(solution: (A, B)) -> (u128, u128)
where
    u128: From<A>,
    u128: From<B>,
{
    (solution.0.into(), solution.1.into())
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
