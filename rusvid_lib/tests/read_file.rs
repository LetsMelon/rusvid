/*
use std::env;

use rusvid_core::pixel::Pixel;
use rusvid_lib::metrics::MetricsVideo;
use rusvid_lib::prelude::*;

mod dummy;

use dummy::DummyRender;

const PIXEL_TRANSPARENT: Pixel = Pixel::new_raw([0; 4]);
const PIXEL_WHITE: Pixel = Pixel::new_raw([255; 4]);
const PIXEL_BLACK: Pixel = Pixel::new_raw([0, 0, 0, 255]);

#[test]
fn renders_svg_file() {
    let mut composition = Composition::builder()
        .resolution(Resolution::Custom(1000, 1000))
        .framerate(1)
        .duration(1)
        .build();

    let layer = Layer::from_file(
        composition.resolution(),
        &env::current_dir()
            .unwrap()
            .as_path()
            .join("./tests/ferris.svg"),
    )
    .unwrap();
    composition.add_layer(layer);

    let image_render = DummyRender::default();

    let buffer = image_render.render_frame(&composition).unwrap();

    // corners
    assert_eq!(buffer.pixel_unchecked(0, 0), &PIXEL_TRANSPARENT);
    assert_eq!(buffer.pixel_unchecked(0, 999), &PIXEL_TRANSPARENT);
    assert_eq!(buffer.pixel_unchecked(999, 0), &PIXEL_TRANSPARENT);
    assert_eq!(buffer.pixel_unchecked(999, 999), &PIXEL_TRANSPARENT);

    // eye's
    assert_eq!(buffer.pixel_unchecked(500, 450), &PIXEL_WHITE);
    assert_eq!(buffer.pixel_unchecked(500, 500), &PIXEL_BLACK);
    assert_eq!(buffer.pixel_unchecked(700, 450), &PIXEL_WHITE);
    assert_eq!(buffer.pixel_unchecked(700, 500), &PIXEL_BLACK);

    // body
    assert_eq!(
        buffer.pixel_unchecked(200, 300),
        &Pixel::new(245, 117, 0, 255)
    );
    assert_eq!(
        buffer.pixel_unchecked(999, 300),
        &Pixel::new(246, 107, 0, 255)
    );

    // count pixels with alpha
    let pixels_with_alpha = buffer.into_iter().filter(|&item| item[3] > 0).count();
    assert_eq!(pixels_with_alpha, 473_746);
    assert!(pixels_with_alpha < composition.pixels());
}
 */

#[test]
fn todo() {
    // TODO
}
