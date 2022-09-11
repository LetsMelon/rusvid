use rayon::prelude::*;
use rusvid_lib::metrics::MetricsVideo;
use rusvid_lib::prelude::*;
use std::env;

const PIXEL_TRANSPARENT: [u8; 4] = [0; 4];
const PIXEL_WHITE: [u8; 4] = [255; 4];
const PIXEL_BLACK: [u8; 4] = [0, 0, 0, 255];

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

    let mut image_render = RawRender::new();

    let buffer = image_render
        .calculate_image_buffer(&mut composition, &0)
        .unwrap();

    // corners
    assert_eq!(buffer.get_pixel(0, 0).0, PIXEL_TRANSPARENT);
    assert_eq!(buffer.get_pixel(0, 999).0, PIXEL_TRANSPARENT);
    assert_eq!(buffer.get_pixel(999, 0).0, PIXEL_TRANSPARENT);
    assert_eq!(buffer.get_pixel(999, 999).0, PIXEL_TRANSPARENT);

    // eye's
    assert_eq!(buffer.get_pixel(500, 450).0, PIXEL_WHITE);
    assert_eq!(buffer.get_pixel(500, 500).0, PIXEL_BLACK);
    assert_eq!(buffer.get_pixel(700, 450).0, PIXEL_WHITE);
    assert_eq!(buffer.get_pixel(700, 500).0, PIXEL_BLACK);

    // body
    assert_eq!(buffer.get_pixel(200, 300).0, [245, 117, 0, 255]);
    assert_eq!(buffer.get_pixel(999, 300).0, [246, 107, 0, 255]);

    // count pixels with alpha
    let pixels_with_alpha = buffer
        .par_chunks_exact(4)
        .filter(|&item| item[3] > 0)
        .count();
    assert_eq!(pixels_with_alpha, 473_746);
    assert!(pixels_with_alpha < composition.pixels());
}
