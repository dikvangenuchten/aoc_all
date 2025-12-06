use std::fs;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;

pub fn solve_days() {
    println!(
        "Day 01 {:?}",
        timeit(|| day_01::solve_day(&read_day_input("01")))
    );
    println!(
        "Day 02 {:?}",
        timeit(|| day_02::solve_day(&read_day_input("02")))
    );
    println!(
        "Day 03 {:?}",
        timeit(|| day_03::solve_day(&read_day_input("03")))
    );
    println!(
        "Day 04 {:?}",
        timeit(|| day_04::solve_day(&read_day_input("04")))
    );
    println!(
        "Day 05 {:?}",
        timeit(|| day_05::solve_day(&read_day_input("05")))
    );
    println!(
        "Day 06 {:?}",
        timeit(|| day_06::solve_day(&read_day_input("06")))
    );
}

fn timeit<F: FnOnce() -> R, R>(f: F) -> R {
    let start = std::time::Instant::now();
    let result = f();
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
    result
}

pub fn read_day_input(day: &str) -> String {
    let file_path = format!("inputs/{day}.txt");
    fs::read_to_string(&file_path).unwrap_or_else(|_| panic!("Could not load: {file_path}"))
}

pub fn read_test_day_input(day: &str) -> String {
    let file_path = format!("test_inputs/test_{day}.txt");
    fs::read_to_string(&file_path).unwrap_or_else(|_| panic!("Could not load: {file_path}"))
}
