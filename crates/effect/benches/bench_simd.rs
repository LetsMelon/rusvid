#![feature(portable_simd)]

// use std::simd::f64x2;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rusvid_core::holder::gradient::base::BaseGradient;
use rusvid_core::holder::gradient::linear::LinearGradient;
use rusvid_core::holder::gradient::stop::Stop;
use rusvid_core::holder::likes::color_like::ColorLike;
use rusvid_core::holder::likes::path_like::PathLike;
use rusvid_core::holder::likes::types_like::TypesLike;
use rusvid_core::holder::object::Object;
use rusvid_core::holder::svg_holder::{SvgHolder, SvgItem};
use rusvid_core::plane::Plane;
use rusvid_core::point::Point;
use rusvid_effect::library::{GaussianBlur, GaussianBlurSimd};
use rusvid_effect::EffectLogic;

// fn simd_add_point_to_point(p1: Point, p2: Point) -> Point {
//     let p1_packed = f64x2::from_array([p1.x(), p1.y()]);
//     let p2_packed = f64x2::from_array([p2.x(), p2.y()]);
//
//     let p_packed = p1_packed + p2_packed;
//     let p_raw = p_packed.to_array();
//
//     Point::new(p_raw[0], p_raw[1])
// }

// const P1: Point = Point::new(2.5, 3.0);
// const P2: Point = Point::new(0.0, -2.0);

const SIZE: u32 = 300;

fn setup() -> Plane {
    let mut svg = SvgHolder::new();

    let rect_size = Point::new_symmetric((SIZE as f64) * 0.3);
    let rect_pos = Point::new(100.0, 50.0);
    let triangle = SvgItem::new(
        vec![
            PathLike::Move(rect_pos),
            PathLike::Line(rect_size * Point::new(1.0, 0.0) + rect_pos),
            PathLike::Line(rect_size * Point::new(1.0, 1.0) + rect_pos),
            PathLike::Line(rect_size * Point::new(0.0, 1.0) + rect_pos),
            PathLike::Close,
        ],
        Some(ColorLike::LinearGradient(LinearGradient::new(
            BaseGradient::new(vec![
                Stop::new([2, 0, 36, 255], 0.0),
                Stop::new([9, 9, 121, 255], 0.35),
                Stop::new([0, 212, 255, 255], 1.0),
            ]),
        ))),
    );
    svg.add_item(triangle);

    let heart = SvgItem::new(
        vec![
            PathLike::Move(Point::new(100.0, 100.0)),
            PathLike::Line(Point::new(150.0, 50.0)),
            PathLike::CurveTo(
                Point::new(100.0, 25.0),
                Point::new(169.0, 11.0),
                Point::new(119.0, -13.0),
            ),
            PathLike::CurveTo(
                Point::new(50.0, 50.0),
                Point::new(80.0, -13.0),
                Point::new(30.0, 11.0),
            ),
            PathLike::Close,
        ],
        Some(ColorLike::LinearGradient(LinearGradient::new(
            BaseGradient::new_from_colors(vec![[255, 0, 0, 255], [255, 100, 0, 255]]),
        ))),
    );
    svg.add_item(heart);

    let object = Object::new(TypesLike::Svg(svg));
    object.render(SIZE, SIZE).unwrap()
}

fn criterion_benchmark(c: &mut Criterion) {
    let stdev = 7.0;
    let effect = GaussianBlur::new(stdev);
    let effect_simd = GaussianBlurSimd::new(stdev);

    let mut group = c.benchmark_group("gaussian blur");

    // group.bench_function("default", |b| b.iter(|| effect.apply(black_box(setup()))));
    // group.bench_function("simd", |b| b.iter(|| effect_simd.apply(black_box(setup()))));

    for stdev in (15..=21).step_by(2) {
        let effect = GaussianBlur::new(stdev as f64);
        group.bench_with_input(BenchmarkId::new("default", stdev), &stdev, |b, _| {
            b.iter(|| effect.apply(black_box(setup())))
        });

        let effect_simd = GaussianBlurSimd::new(stdev as f64);
        group.bench_with_input(BenchmarkId::new("simd", stdev), &stdev, |b, _| {
            b.iter(|| effect_simd.apply(black_box(setup())))
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
