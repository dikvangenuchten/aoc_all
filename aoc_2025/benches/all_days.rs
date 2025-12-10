use aoc_2025::days::{
    day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08, day_09, read_day_input,
};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn benchmark_day_01(c: &mut Criterion) {
    let input = read_day_input("01");
    c.bench_function("day_01", |b| {
        b.iter(|| day_01::solve_day(black_box(&input)))
    });
}

fn benchmark_day_02(c: &mut Criterion) {
    let input = read_day_input("02");
    c.bench_function("day_02", |b| {
        b.iter(|| day_02::solve_day(black_box(&input)))
    });
}

fn benchmark_day_03(c: &mut Criterion) {
    let input = read_day_input("03");
    c.bench_function("day_03", |b| {
        b.iter(|| day_03::solve_day(black_box(&input)))
    });
}

fn benchmark_day_04(c: &mut Criterion) {
    let input = read_day_input("04");
    c.bench_function("day_04", |b| {
        b.iter(|| day_04::solve_day(black_box(&input)))
    });
}

fn benchmark_day_05(c: &mut Criterion) {
    let input = read_day_input("05");
    c.bench_function("day_05", |b| {
        b.iter(|| day_05::solve_day(black_box(&input)))
    });
}

fn benchmark_day_06(c: &mut Criterion) {
    let input = read_day_input("06");
    c.bench_function("day_06", |b| {
        b.iter(|| day_06::solve_day(black_box(&input)))
    });
}

fn benchmark_day_07(c: &mut Criterion) {
    let input = read_day_input("07");
    c.bench_function("day_07", |b| {
        b.iter(|| day_07::solve_day(black_box(&input)))
    });
}

fn benchmark_day_08(c: &mut Criterion) {
    let input = read_day_input("08");
    c.bench_function("day_08", |b| {
        b.iter(|| day_08::solve_day(black_box(&input)))
    });
}

fn benchmark_day_09(c: &mut Criterion) {
    let input = read_day_input("09");
    c.bench_function("day_09", |b| {
        b.iter(|| day_09::solve_day(black_box(&input)))
    });
}

criterion_group!(
    benches,
    benchmark_day_01,
    benchmark_day_02,
    benchmark_day_03,
    benchmark_day_04,
    benchmark_day_05,
    benchmark_day_06,
    benchmark_day_07,
    benchmark_day_08,
    benchmark_day_09
);
criterion_main!(benches);
