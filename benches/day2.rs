use std::hint::black_box;

use aoc::{day_2::Day2Part1, Day};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day2part1", |b| b.iter(|| black_box(Day2Part1::run)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
