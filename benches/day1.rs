use std::hint::black_box;

use aoc::{day_1::Day1, Day};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day1 input", |b| b.iter(|| black_box(Day1::run)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
