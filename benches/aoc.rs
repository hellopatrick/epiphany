use criterion::{black_box, criterion_group, criterion_main, Criterion};
use epiphany::*;

pub fn day03_benchmark(c: &mut Criterion) {
  let input: Vec<_> = include_str!("../data/02.txt").lines().collect();

  c.bench_function("part 1", |b| {
    b.iter(|| day03::first_star(black_box(&input)))
  });

  c.bench_function("part 2", |b| {
    b.iter(|| day03::second_star(black_box(&input)))
  });
}

criterion_group!(benches, day03_benchmark);
criterion_main!(benches);
