use criterion::{black_box, criterion_group, criterion_main, Criterion};
use itertools::Itertools;
use rand::{Rng, SeedableRng};
use rusvid_core::plane::Plane;
use rusvid_effect::library::{
    BoxBlur, ColorPaletteEffect, GaussianBlur, GrayscaleEffect, PixelateEffect,
};
use rusvid_effect::*;

const WIDTH: u32 = 2_u32.pow(9);
const HEIGHT: u32 = 2_u32.pow(9);

// some magic pseudo magic number for the seed
// see https://en.wikipedia.org/wiki/Hexspeak
const SEED: u64 = 0xFEEDC0DE;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(SEED);

    let plane = Plane::from_data(
        WIDTH,
        HEIGHT,
        (0..WIDTH)
            .cartesian_product(0..HEIGHT)
            .map(|_| [rng.gen(), rng.gen(), rng.gen(), 255])
            .collect_vec(),
    )
    .unwrap();

    let effect = GrayscaleEffect::new();
    c.bench_function(effect.name(), |b| {
        b.iter(|| assert!(effect.apply(black_box(plane.clone())).is_ok()))
    });

    let effect = BoxBlur::new(3).unwrap();
    c.bench_function(&format!("{} - {:?}", effect.name(), effect.kernel()), |b| {
        b.iter(|| assert!(effect.apply(black_box(plane.clone())).is_ok()))
    });
    let effect = BoxBlur::new(5).unwrap();
    c.bench_function(&format!("{} - {:?}", effect.name(), effect.kernel()), |b| {
        b.iter(|| assert!(effect.apply(black_box(plane.clone())).is_ok()))
    });

    let effect = GaussianBlur::new(1.75);
    c.bench_function(&format!("{} - {:.2}", effect.name(), effect.stdev()), |b| {
        b.iter(|| assert!(effect.apply(black_box(plane.clone())).is_ok()))
    });
    let effect = GaussianBlur::new(5.0);
    c.bench_function(&format!("{} - {:.2}", effect.name(), effect.stdev()), |b| {
        b.iter(|| assert!(effect.apply(black_box(plane.clone())).is_ok()))
    });

    let effect = ColorPaletteEffect::new(vec![
        [255; 4],
        [0, 0, 0, 255],
        [255, 0, 0, 255],
        [rng.gen(), rng.gen(), rng.gen(), 255],
    ]);
    c.bench_function(
        &format!("{} - {}", effect.name(), effect.palette_length()),
        |b| b.iter(|| assert!(effect.apply(black_box(plane.clone())).is_ok())),
    );
    let effect = ColorPaletteEffect::new(vec![
        [rng.gen(), rng.gen(), rng.gen(), 255],
        [rng.gen(), rng.gen(), rng.gen(), 255],
        [rng.gen(), rng.gen(), rng.gen(), 255],
        [rng.gen(), rng.gen(), rng.gen(), 255],
        [rng.gen(), rng.gen(), rng.gen(), 255],
        [rng.gen(), rng.gen(), rng.gen(), 255],
        [rng.gen(), rng.gen(), rng.gen(), 255],
        [rng.gen(), rng.gen(), rng.gen(), 255],
    ]);
    c.bench_function(
        &format!("{} - {}", effect.name(), effect.palette_length()),
        |b| b.iter(|| assert!(effect.apply(black_box(plane.clone())).is_ok())),
    );

    let effect = PixelateEffect::new(4, 4);
    c.bench_function(&format!("{} - {:?}", effect.name(), effect.kernel()), |b| {
        b.iter(|| assert!(effect.apply(black_box(plane.clone())).is_ok()))
    });
    let effect = PixelateEffect::new(16, 16);
    c.bench_function(&format!("{} - {:?}", effect.name(), effect.kernel()), |b| {
        b.iter(|| assert!(effect.apply(black_box(plane.clone())).is_ok()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
