use advent_of_code_2022::days::*;
use advent_of_code_2022::run_all_days;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_day_1(c: &mut Criterion) {
    let input = read_day_input("day_01");
    c.bench_function("day 1", |b| b.iter(|| day_01::solve(black_box(&input))));
}

fn bench_day_2(c: &mut Criterion) {
    let input = read_day_input("day_02");
    c.bench_function("day 2", |b| b.iter(|| day_02::solve(black_box(&input))));
}

fn bench_day_3(c: &mut Criterion) {
    let input = read_day_input("day_03");
    c.bench_function("day 3", |b| b.iter(|| day_03::solve(black_box(&input))));
}

fn bench_day_4(c: &mut Criterion) {
    let input = read_day_input("day_04");
    c.bench_function("day 4", |b| b.iter(|| day_04::solve(black_box(&input))));
}

fn bench_day_5(c: &mut Criterion) {
    let input = read_day_input("day_05");
    c.bench_function("day 5", |b| b.iter(|| day_05::solve(black_box(&input))));
}

fn bench_day_6(c: &mut Criterion) {
    let input = read_day_input("day_06");
    c.bench_function("day 6", |b| b.iter(|| day_06::solve(black_box(&input))));
}

fn bench_day_7(c: &mut Criterion) {
    let input = read_day_input("day_07");
    c.bench_function("day 7", |b| b.iter(|| day_07::solve(black_box(&input))));
}

fn bench_day_8(c: &mut Criterion) {
    let input = read_day_input("day_08");
    c.bench_function("day 8", |b| b.iter(|| day_08::solve(black_box(&input))));
}

fn bench_day_9(c: &mut Criterion) {
    let input = read_day_input("day_09");
    c.bench_function("day 9", |b| b.iter(|| day_09::solve(black_box(&input))));
}

fn bench_day_10(c: &mut Criterion) {
    let input = read_day_input("day_10");
    c.bench_function("day 10", |b| b.iter(|| day_10::solve(black_box(&input))));
}

fn bench_day_11(c: &mut Criterion) {
    let input = read_day_input("day_11");
    c.bench_function("day 11", |b| b.iter(|| day_11::solve(black_box(&input))));
}

fn bench_day_12(c: &mut Criterion) {
    let input = read_day_input("day_12");
    c.bench_function("day 12", |b| b.iter(|| day_12::solve(black_box(&input))));
}

fn bench_day_13(c: &mut Criterion) {
    let input = read_day_input("day_13");
    c.bench_function("day 13", |b| b.iter(|| day_13::solve(black_box(&input))));
}

fn bench_day_14(c: &mut Criterion) {
    let input = read_day_input("day_14");
    c.bench_function("day 14", |b| b.iter(|| day_14::solve(black_box(&input))));
}

fn bench_day_15(c: &mut Criterion) {
    let input = read_day_input("day_15");
    c.bench_function("day 15", |b| b.iter(|| day_15::solve(black_box(&input))));
}

fn bench_all_days(c: &mut Criterion) {
    c.bench_function("All days", |b| b.iter(|| run_all_days()));
}

criterion_group!(
    benches,
    bench_all_days,
    bench_day_1,
    bench_day_2,
    bench_day_3,
    bench_day_4,
    bench_day_5,
    bench_day_6,
    bench_day_7,
    bench_day_8,
    bench_day_9,
    bench_day_10,
    bench_day_11,
    bench_day_12,
    bench_day_13,
    bench_day_14,
    bench_day_15,
);
criterion_main!(benches);
