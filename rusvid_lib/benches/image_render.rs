use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rusvid_lib::prelude::figures::*;
use rusvid_lib::prelude::*;
use rusvid_lib::renderer::ImageRender;
use rusvid_lib::usvg::*;
use rusvid_lib::utils::color_from_hex;
use std::rc::Rc;

#[inline(always)]
fn raw(comp: &mut Composition) {
    let mut image_render = RawRender::new();

    let data = image_render.calculate_image_buffer(comp, &0).unwrap();
    assert_eq!(data.pixels().len(), 10_000);
}

#[inline(always)]
fn png(comp: &mut Composition) {
    let mut image_render = PngRender::new();

    let pixmap = image_render.render_pixmap(comp, &0).unwrap();
    let data = pixmap.encode_png().unwrap();
    assert_eq!(data.len(), 735);
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut composition = Composition::builder()
        .resolution(Resolution::Custom(100, 100))
        .framerate(1)
        .duration(1)
        .build();

    let mut layer = Layer::new(composition.resolution());
    layer
        .add_to_root(NodeKind::Path(Path {
            id: "ul".to_string(),
            fill: Some(Fill {
                paint: Paint::Color(color_from_hex("ff0000".to_string()).unwrap()),
                ..Fill::default()
            }),
            data: Rc::new(rect(0.0, 0.0, 50.0, 50.0)),
            ..Path::default()
        }))
        .unwrap();
    layer
        .add_to_root(NodeKind::Path(Path {
            id: "ur".to_string(),
            fill: Some(Fill {
                paint: Paint::Color(color_from_hex("00ff00".to_string()).unwrap()),
                ..Fill::default()
            }),
            data: Rc::new(rect(50.0, 0.0, 50.0, 50.0)),
            ..Path::default()
        }))
        .unwrap();
    layer
        .add_to_root(NodeKind::Path(Path {
            id: "dl".to_string(),
            fill: Some(Fill {
                paint: Paint::Color(color_from_hex("0000ff".to_string()).unwrap()),
                ..Fill::default()
            }),
            data: Rc::new(rect(0.0, 50.0, 50.0, 50.0)),
            ..Path::default()
        }))
        .unwrap();
    layer
        .add_to_root(NodeKind::Path(Path {
            id: "dr".to_string(),
            fill: Some(Fill {
                paint: Paint::Color(color_from_hex("fff00f".to_string()).unwrap()),
                ..Fill::default()
            }),
            data: Rc::new(circle(75.0, 75.0, 25.0)),
            rendering_mode: ShapeRendering::CrispEdges,
            ..Path::default()
        }))
        .unwrap();
    composition.add_layer(layer);

    c.bench_function("raw", |b| b.iter(|| raw(black_box(&mut composition))));
    c.bench_function("png", |b| b.iter(|| png(black_box(&mut composition))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
