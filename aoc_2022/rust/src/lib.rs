pub mod days;
use std::time::{self, Duration};

use days::*;

pub struct DayResult {
    pub part_1: String,
    pub part_2: String,
    pub time: Duration,
}

pub fn run_all_days() -> Vec<DayResult> {
    let start = time::Instant::now();
    let (day_1a, day_1b) = day_01::solve(&read_day_input("day_01"));
    let day_1_time = start.elapsed();
    let day_1 = DayResult {
        part_1: format!("{}", day_1a),
        part_2: format!("{}", day_1b),
        time: day_1_time,
    };

    let start = time::Instant::now();
    let (day_2a, day_2b) = day_02::solve(&read_day_input("day_02"));
    let day_2_time = start.elapsed();
    let day_2 = DayResult {
        part_1: format!("{}", day_2a),
        part_2: format!("{}", day_2b),
        time: day_2_time,
    };

    let start = time::Instant::now();
    let (day_3a, day_3b) = day_03::solve(&read_day_input("day_03"));
    let day_3_time = start.elapsed();
    let day_3 = DayResult {
        part_1: format!("{}", day_3a),
        part_2: format!("{}", day_3b),
        time: day_3_time,
    };

    let start = time::Instant::now();
    let (day_4a, day_4b) = day_04::solve(&read_day_input("day_04"));
    let day_4_time = start.elapsed();
    let day_4 = DayResult {
        part_1: format!("{}", day_4a),
        part_2: format!("{}", day_4b),
        time: day_4_time,
    };

    let start = time::Instant::now();
    let (day_5a, day_5b) = day_05::solve(&read_day_input("day_05"));
    let day_5_time = start.elapsed();
    let day_5 = DayResult {
        part_1: day_5a,
        part_2: day_5b,
        time: day_5_time,
    };

    let start = time::Instant::now();
    let (day_6a, day_6b) = day_06::solve(&read_day_input("day_06"));
    let day_6_time = start.elapsed();
    let day_6 = DayResult {
        part_1: format!("{}", day_6a),
        part_2: format!("{}", day_6b),
        time: day_6_time,
    };

    let start = time::Instant::now();
    let (day_7a, day_7b) = day_07::solve(&read_day_input("day_07"));
    let day_7_time = start.elapsed();
    let day_7 = DayResult {
        part_1: format!("{}", day_7a),
        part_2: format!("{}", day_7b),
        time: day_7_time,
    };

    let start = time::Instant::now();
    let (day_8a, day_8b) = day_08::solve(&read_day_input("day_08"));
    let day_8_time = start.elapsed();
    let day_8 = DayResult {
        part_1: format!("{}", day_8a),
        part_2: format!("{}", day_8b),
        time: day_8_time,
    };

    let start = time::Instant::now();
    let (day_9a, day_9b) = day_09::solve(&read_day_input("day_09"));
    let day_9_time = start.elapsed();
    let day_9 = DayResult {
        part_1: format!("{}", day_9a),
        part_2: format!("\n{}", day_9b),
        time: day_9_time,
    };

    let start = time::Instant::now();
    let (day_10a, day_10b) = day_10::solve(&read_day_input("day_10"));
    let day_10_time = start.elapsed();
    let day_10 = DayResult {
        part_1: format!("{}", day_10a),
        part_2: day_10b,
        time: day_10_time,
    };

    let start = time::Instant::now();
    let (day_11a, day_11b) = day_11::solve(&read_day_input("day_11"));
    let day_11_time = start.elapsed();
    let day_11 = DayResult {
        part_1: format!("{}", day_11a),
        part_2: format!("{}", day_11b),
        time: day_11_time,
    };

    let start = time::Instant::now();
    let (day_12a, day_12b) = day_12::solve(&read_day_input("day_12"));
    let day_12_time = start.elapsed();
    let day_12 = DayResult {
        part_1: format!("{}", day_12a),
        part_2: format!("{}", day_12b),
        time: day_12_time,
    };

    let start = time::Instant::now();
    let (day_13a, day_13b) = day_13::solve(&read_day_input("day_13"));
    let day_13_time = start.elapsed();
    let day_13 = DayResult {
        part_1: format!("{}", day_13a),
        part_2: format!("{}", day_13b),
        time: day_13_time,
    };

    let start = time::Instant::now();
    let (day_14a, day_14b) = day_14::solve(&read_day_input("day_14"));
    let day_14_time = start.elapsed();
    let day_14 = DayResult {
        part_1: format!("{}", day_14a),
        part_2: format!("{}", day_14b),
        time: day_14_time,
    };

    let start = time::Instant::now();
    let (day_15a, day_15b) = day_15::solve(&read_day_input("day_15"));
    let day_15_time = start.elapsed();
    let day_15 = DayResult {
        part_1: format!("{}", day_15a),
        part_2: format!("{}", day_15b),
        time: day_15_time,
    };

    println!("Starting day 16");
    let start = time::Instant::now();
    let (day_16a, day_16b) = day_16::solve(&read_day_input("day_16"));
    let day_16_time = start.elapsed();
    let day_16 = DayResult {
        part_1: format!("{}", day_16a),
        part_2: format!("{}", day_16b),
        time: day_16_time,
    };

    vec![
        day_1, day_2, day_3, day_4, day_5, day_6, day_7, day_8, day_9, day_10, day_11, day_12,
        day_13, day_14, day_15, day_16,
    ]
}
