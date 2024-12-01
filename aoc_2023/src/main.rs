use std::time::Instant;

use aoc_2023::days::solve_all;

fn main() {
    let time = Instant::now();
    for (i, day) in solve_all().iter().enumerate() {
        println!(
            "Solution for day {}: part_a {}, part_b: {}",
            i + 1,
            day.0,
            day.1
        );
    }
    println!("Total {:?} ms", (Instant::now() - time).as_millis());
}
