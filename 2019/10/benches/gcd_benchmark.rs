use criterion::{criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use std::hint::black_box;

use aoc_2019_10::gcd;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = thread_rng();

    let a = rng.gen_range(0..=u64::MAX);
    let b = rng.gen_range(0..=u64::MAX);

    c.bench_function("stein: {a}, {b}", |n| {
        n.iter(|| gcd::stein(black_box(a), black_box(b)));
    });

    c.bench_function("euclid iterative: {a}, {b}", |n| {
        n.iter(|| gcd::euclid_iterative(black_box(a), black_box(b)));
    });

    c.bench_function("euclid recursive: {a}, {b}", |n| {
        n.iter(|| gcd::euclid_recursive(black_box(a), black_box(b)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
