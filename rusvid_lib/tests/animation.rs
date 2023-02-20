use std::rc::Rc;

use rusvid_core::plane::Pixel;
use rusvid_core::point::Point;
use rusvid_lib::animation::prelude::*;
use rusvid_lib::figures::prelude::*;
use rusvid_lib::prelude::*;
use rusvid_lib::resolution::Resolution;
use rusvid_lib::resvg::usvg::{Fill, NodeKind, Paint, Path};
use rusvid_lib::utils::color_from_hex;

mod dummy;

use dummy::DummyRender;

#[test]
fn renders_correctly_static() {
    let mut composition = Composition::builder()
        .resolution(Resolution::Custom(100, 100))
        .framerate(2)
        .duration(1)
        .build();

    composition
        .add_to_root(NodeKind::Path(Path {
            id: "rect".to_string(),
            fill: Some(Fill {
                paint: Paint::Color(color_from_hex("ff0000".to_string()).unwrap()),
                ..Fill::default()
            }),
            data: Rc::new(rect(Point::ZERO, Point::new(50.0, 50.0))),
            ..Path::default()
        }))
        .unwrap();

    // TODO why not with end_frame=1
    composition.add_animation(PositionAnimation::new(
        "rect",
        Linear::new(0, 1, Point::ZERO, Point::new(50.0, 50.0)).unwrap(),
    ));

    let image_render = DummyRender::default();

    composition.update(0).unwrap();
    let frame_1 = image_render.render_frame(&composition);

    composition.update(1).unwrap();
    let frame_2 = image_render.render_frame(&composition);

    match (frame_1, frame_2) {
        (Ok(frame_1), Ok(frame_2)) => {
            assert_eq!(frame_1.pixel_unchecked(0, 0), &Pixel::new(255, 0, 0, 255));
            assert_eq!(frame_1.pixel_unchecked(49, 49), &Pixel::new(255, 0, 0, 255));
            assert_eq!(frame_1.pixel_unchecked(50, 50), &Pixel::new(0, 0, 0, 0));
            assert_eq!(frame_1.pixel_unchecked(99, 99), &Pixel::new(0, 0, 0, 0));

            assert_eq!(frame_2.pixel_unchecked(0, 0), &Pixel::new(0, 0, 0, 0));
            assert_eq!(frame_2.pixel_unchecked(49, 49), &Pixel::new(0, 0, 0, 0));
            assert_eq!(frame_2.pixel_unchecked(50, 50), &Pixel::new(255, 0, 0, 255));
            assert_eq!(frame_2.pixel_unchecked(99, 99), &Pixel::new(255, 0, 0, 255));
        }
        (_, _) => assert!(false),
    };
}
