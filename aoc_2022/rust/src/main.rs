use std::time::Duration;

use advent_of_code_2022::run_all_days;

pub fn main() {
    let results = run_all_days();
    let mut all_solution_time = Duration::default();
    for (i, result) in results.iter().enumerate() {
        println!("Solutions for day {}", i + 1);
        println!("part 1: {}", result.part_1);
        println!("part 2: {}", result.part_2);
        println!("It took {:#?}\n", result.time);
        all_solution_time += result.time;
    }

    println!(
        "Solving {} days took {:#?}",
        results.len(),
        all_solution_time
    );
}
