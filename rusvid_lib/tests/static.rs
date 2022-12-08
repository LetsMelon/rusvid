use std::rc::Rc;

use rusvid_core::point::Point;
use rusvid_lib::composition::Composition;
use rusvid_lib::figures::circle::circle;
use rusvid_lib::figures::rect::rect;
use rusvid_lib::layer::LayerLogic;
use rusvid_lib::resolution::Resolution;
use rusvid_lib::usvg::{Fill, NodeKind, Paint, Path};
use rusvid_lib::utils::color_from_hex;
use usvg::ShapeRendering;

mod dummy;

use dummy::DummyRender;

#[test]
fn renders_correctly_static() {
    let mut composition = Composition::builder()
        .resolution(Resolution::Custom(100, 100))
        .framerate(1)
        .duration(1)
        .build();

    composition
        .add_to_root(NodeKind::Path(Path {
            id: "ul".to_string(),
            fill: Some(Fill {
                paint: Paint::Color(color_from_hex("ff0000".to_string()).unwrap()),
                ..Fill::default()
            }),
            data: Rc::new(rect(Point::ZERO, Point::new(50.0, 50.0))),
            ..Path::default()
        }))
        .unwrap();
    composition
        .add_to_root(NodeKind::Path(Path {
            id: "ur".to_string(),
            fill: Some(Fill {
                paint: Paint::Color(color_from_hex("00ff00".to_string()).unwrap()),
                ..Fill::default()
            }),
            data: Rc::new(rect(Point::new(50.0, 0.0), Point::new(50.0, 50.0))),
            ..Path::default()
        }))
        .unwrap();
    composition
        .add_to_root(NodeKind::Path(Path {
            id: "dl".to_string(),
            fill: Some(Fill {
                paint: Paint::Color(color_from_hex("0000ff".to_string()).unwrap()),
                ..Fill::default()
            }),
            data: Rc::new(rect(Point::new(0.0, 50.0), Point::new(50.0, 50.0))),
            ..Path::default()
        }))
        .unwrap();
    composition
        .add_to_root(NodeKind::Path(Path {
            id: "dr".to_string(),
            fill: Some(Fill {
                paint: Paint::Color(color_from_hex("fff00f".to_string()).unwrap()),
                ..Fill::default()
            }),
            data: Rc::new(circle(Point::new(75.0, 75.0), 25.0)),
            rendering_mode: ShapeRendering::CrispEdges,
            ..Path::default()
        }))
        .unwrap();

    let image_render = DummyRender::default();

    let buffer = image_render.render_frame(&composition);
    if let Ok(buffer) = buffer {
        // Corners
        assert_eq!(buffer.pixel_unchecked(0, 0), &[255, 0, 0, 255]);
        assert_eq!(buffer.pixel_unchecked(99, 0), &[0, 255, 0, 255]);
        assert_eq!(buffer.pixel_unchecked(0, 99), &[0, 0, 255, 255]);
        assert_eq!(buffer.pixel_unchecked(99, 99), &[0, 0, 0, 0]);

        // Middle
        assert_eq!(buffer.pixel_unchecked(24, 24), &[255, 0, 0, 255]);
        assert_eq!(buffer.pixel_unchecked(74, 24), &[0, 255, 0, 255]);
        assert_eq!(buffer.pixel_unchecked(24, 74), &[0, 0, 255, 255]);
        assert_eq!(buffer.pixel_unchecked(74, 74), &[255, 240, 15, 255]);
    } else {
        assert!(false);
    }
}
