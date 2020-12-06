use criterion::{black_box, criterion_group, criterion_main, Criterion};
use epiphany::*;

pub fn day03_benchmark(c: &mut Criterion) {
  let input: Vec<_> = include_str!("../data/03.txt")
    .lines()
    .map(|line| line.chars().map(|c| c == '#').collect())
    .collect();

  c.bench_function("day04:01", |b| {
    b.iter(|| day03::first_star(black_box(&input)))
  });

  c.bench_function("day04:02", |b| {
    b.iter(|| day03::second_star(black_box(&input)))
  });
}

pub fn day05_benchmark(c: &mut Criterion) {
  let input: Vec<_> = include_str!("../data/05.txt").lines().collect();

  c.bench_function("day05:01", |b| {
    b.iter(|| day05::first_star(black_box(&input)))
  });

  c.bench_function("day05:02", |b| {
    b.iter(|| day05::second_star(black_box(&input)))
  });
}

criterion_group!(benches, day03_benchmark, day05_benchmark);
criterion_main!(benches);
