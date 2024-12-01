use aoc_2023::days::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_day_01(c: &mut Criterion) {
    let input = read_day_input("day_01");
    c.bench_function("day 01", |b| {
        b.iter(|| day_01::solve_day(black_box(&input)))
    });
}

fn bench_day_01a(c: &mut Criterion) {
    let input = read_day_input("day_01");
    c.bench_function("day 01a", |b| b.iter(|| day_01::part_a(black_box(&input))));
}

fn bench_day_01b(c: &mut Criterion) {
    let input = read_day_input("day_01");
    c.bench_function("day 01b", |b| b.iter(|| day_01::part_b(black_box(&input))));
}

fn bench_day_02(c: &mut Criterion) {
    let input = read_day_input("day_02");
    c.bench_function("day 02", |b| {
        b.iter(|| day_02::solve_day(black_box(&input)))
    });
}

fn bench_day_03(c: &mut Criterion) {
    let input = read_day_input("day_03");
    c.bench_function("day 03", |b| {
        b.iter(|| day_03::solve_day(black_box(&input)))
    });
}

fn bench_day_04(c: &mut Criterion) {
    let input = read_day_input("day_04");
    c.bench_function("day 04", |b| {
        b.iter(|| day_04::solve_day(black_box(&input)))
    });
}

fn bench_day_05(c: &mut Criterion) {
    let input = read_day_input("day_05");
    c.bench_function("day 05", |b| {
        b.iter(|| day_05::solve_day(black_box(&input)))
    });
}

fn bench_day_06(c: &mut Criterion) {
    let input = read_day_input("day_06");
    c.bench_function("day 06", |b| {
        b.iter(|| day_06::solve_day(black_box(&input)))
    });
}

fn bench_day_07(c: &mut Criterion) {
    let input = read_day_input("day_07");
    c.bench_function("day 07", |b| {
        b.iter(|| day_07::solve_day(black_box(&input)))
    });
}

fn bench_day_08(c: &mut Criterion) {
    let input = read_day_input("day_08");
    c.bench_function("day 08", |b| {
        b.iter(|| day_08::solve_day(black_box(&input)))
    });
}

fn bench_day_09(c: &mut Criterion) {
    let input = read_day_input("day_09");
    c.bench_function("day 09", |b| {
        b.iter(|| day_09::solve_day(black_box(&input)))
    });
}

fn bench_day_10(c: &mut Criterion) {
    let input = read_day_input("day_10");
    c.bench_function("day 10", |b| {
        b.iter(|| day_10::solve_day(black_box(&input)))
    });
}

fn bench_day_11(c: &mut Criterion) {
    let input = read_day_input("day_11");
    c.bench_function("day 11", |b| {
        b.iter(|| day_11::solve_day(black_box(&input)))
    });
}

fn bench_day_12(c: &mut Criterion) {
    let input = read_day_input("day_12");
    c.bench_function("day 12", |b| {
        b.iter(|| day_12::solve_day(black_box(&input)))
    });
}

fn bench_day_13(c: &mut Criterion) {
    let input = read_day_input("day_13");
    c.bench_function("day 13", |b| {
        b.iter(|| day_13::solve_day(black_box(&input)))
    });
}

fn bench_day_14(c: &mut Criterion) {
    let input = read_day_input("day_14");
    c.bench_function("day 14", |b| {
        b.iter(|| day_14::solve_day(black_box(&input)))
    });
}

fn bench_day_15(c: &mut Criterion) {
    let input = read_day_input("day_15");
    c.bench_function("day 15", |b| {
        b.iter(|| day_15::solve_day(black_box(&input)))
    });
}

fn bench_day_16(c: &mut Criterion) {
    let input = read_day_input("day_16");
    c.bench_function("day 16", |b| {
        b.iter(|| day_16::solve_day(black_box(&input)))
    });
}

fn bench_day_17(c: &mut Criterion) {
    let input = read_day_input("day_17");
    c.bench_function("day 17", |b| {
        b.iter(|| day_17::solve_day(black_box(&input)))
    });
}

fn bench_all_days(c: &mut Criterion) {
    c.bench_function("All days", |b| b.iter(solve_all));
}

criterion_group!(
    benches,
    bench_day_17,
    bench_day_16,
    bench_day_15,
    bench_day_14,
    bench_day_13,
    bench_day_12, // Uncomment as it is very slow
    bench_day_11,
    bench_day_10,
    bench_day_09,
    bench_day_08,
    bench_day_07,
    bench_day_06,
    bench_day_05,
    bench_day_04,
    bench_day_03,
    bench_day_02,
    bench_day_01b,
    bench_day_01a,
    bench_day_01,
    bench_all_days,
);
criterion_main!(benches);
