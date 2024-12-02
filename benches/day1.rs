use std::hint::black_box;

use aoc::{
    day_1::{Day1Part1, Day1Part2},
    Day,
};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day1part1", |b| b.iter(|| black_box(Day1Part1::run)));
    c.bench_function("day1part2", |b| b.iter(|| black_box(Day1Part2::run)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
