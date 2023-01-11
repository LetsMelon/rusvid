#![feature(portable_simd)]

use std::simd::f64x2;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rusvid_core::point::{add_point_for_point, Point};

fn simd_add_point_to_point(p1: Point, p2: Point) -> Point {
    let p1_packed = f64x2::from_array([p1.x(), p1.y()]);
    let p2_packed = f64x2::from_array([p2.x(), p2.y()]);

    let p_packed = p1_packed + p2_packed;
    let p_raw = p_packed.to_array();

    Point::new(p_raw[0], p_raw[1])
}

const P1: Point = Point::new(2.5, 3.0);
const P2: Point = Point::new(0.0, -2.0);

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("point + point");

    group.bench_function("default", |b| {
        b.iter(|| add_point_for_point(black_box(P1), black_box(P2)))
    });
    group.bench_function("simd", |b| {
        b.iter(|| simd_add_point_to_point(black_box(P1), black_box(P2)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
