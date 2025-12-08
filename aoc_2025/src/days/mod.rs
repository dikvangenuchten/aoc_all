use std::fs;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;

pub fn solve_days() {
    let start = std::time::Instant::now();
    println!("Solving Advent of Code 2025:");
    timeit_day(1, || day_01::solve_day(&read_day_input("01")));
    timeit_day(2, || day_02::solve_day(&read_day_input("02")));
    timeit_day(3, || day_03::solve_day(&read_day_input("03")));
    timeit_day(4, || day_04::solve_day(&read_day_input("04")));
    timeit_day(5, || day_05::solve_day(&read_day_input("05")));
    timeit_day(6, || day_06::solve_day(&read_day_input("06")));
    timeit_day(7, || day_07::solve_day(&read_day_input("07")));
    timeit_day(8, || day_08::solve_day(&read_day_input("08")));
    let duration = start.elapsed();
    println!("Total time: {:.2?}", duration);
}

fn timeit_day<F: FnOnce() -> R, R: std::fmt::Debug>(i: u8, f: F) {
    let start = std::time::Instant::now();
    let result = f();
    let duration = start.elapsed();
    let result_string = format!("Day {i}: {result:?}");
    println!("{result_string:60}  (took: {:.2?})", duration);
}

pub fn read_day_input(day: &str) -> String {
    let file_path = format!("inputs/{day}.txt");
    fs::read_to_string(&file_path).unwrap_or_else(|_| panic!("Could not load: {file_path}"))
}

pub fn read_test_day_input(day: &str) -> String {
    let file_path = format!("test_inputs/test_{day}.txt");
    fs::read_to_string(&file_path).unwrap_or_else(|_| panic!("Could not load: {file_path}"))
}
