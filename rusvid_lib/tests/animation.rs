use rusvid_lib::animation::curves::linear::Linear;
use rusvid_lib::animation::curves::Points;
use rusvid_lib::animation::position_animation::PositionAnimation;
use rusvid_lib::composition::Composition;
use rusvid_lib::figures::rect::rect;
use rusvid_lib::layer::LayerLogic;
use rusvid_lib::prelude::Function;
use rusvid_lib::renderer::raw::RawRender;
use rusvid_lib::resolution::Resolution;
use rusvid_lib::usvg::{Fill, NodeKind, Paint, Path};
use rusvid_lib::utils::color_from_hex;
use std::rc::Rc;

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
            data: Rc::new(rect(0.0, 0.0, 50.0, 50.0)),
            ..Path::default()
        }))
        .unwrap();

    // TODO why not with end_frame=1
    composition.add_animation(PositionAnimation::new(
        "rect".to_string(),
        Linear::new(0, 1, Points::zero_2d(), Points::Point2d(50.0, 50.0)).unwrap(),
    ));

    let image_render = RawRender::new();

    composition.update(0).unwrap();
    let frame_1 = image_render.calculate_image_buffer(&composition);

    composition.update(1).unwrap();
    let frame_2 = image_render.calculate_image_buffer(&composition);

    match (frame_1, frame_2) {
        (Ok(frame_1), Ok(frame_2)) => {
            assert_eq!(frame_1.get_pixel(0, 0).0, [255, 0, 0, 255]);
            assert_eq!(frame_1.get_pixel(49, 49).0, [255, 0, 0, 255]);
            assert_eq!(frame_1.get_pixel(50, 50).0, [0, 0, 0, 0]);
            assert_eq!(frame_1.get_pixel(99, 99).0, [0, 0, 0, 0]);

            assert_eq!(frame_2.get_pixel(0, 0).0, [0, 0, 0, 0]);
            assert_eq!(frame_2.get_pixel(49, 49).0, [0, 0, 0, 0]);
            assert_eq!(frame_2.get_pixel(50, 50).0, [255, 0, 0, 255]);
            assert_eq!(frame_2.get_pixel(99, 99).0, [255, 0, 0, 255]);
        }
        (_, _) => assert!(false),
    };
}
