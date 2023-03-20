use rusvid_core::holder::likes::{ColorLike, TypesLike};
use rusvid_core::holder::svg_holder::SvgItem;
use rusvid_core::holder::transform::{Transform, TransformLogic};
use rusvid_core::pixel::Pixel;
use rusvid_core::point::Point;
use rusvid_lib::composition::Composition;
use rusvid_lib::figures::circle::circle;
use rusvid_lib::figures::rect::rect;
use rusvid_lib::prelude::LayerType;
use rusvid_lib::resolution::Resolution;

mod dummy;

use dummy::DummyRender;

#[test]
fn renders_correctly_static() {
    let mut composition = Composition::builder()
        .resolution(Resolution::Custom(100, 100))
        .framerate(1)
        .duration(1)
        .build();

    let layer = composition.create_new_layer(LayerType::Svg).unwrap();

    if let TypesLike::Svg(svg_holder) = layer.object.data_mut() {
        let mut item = SvgItem::new(
            rect(Point::ZERO, Point::new(50.0, 50.0)),
            Some(ColorLike::Color(Pixel::from_hex_string("ff0000").unwrap())),
        );
        item.transform(&Transform::Stroke(None)).unwrap();
        svg_holder.add_item(item);

        let mut item = SvgItem::new(
            rect(Point::new(50.0, 0.0), Point::new(50.0, 50.0)),
            Some(ColorLike::Color(Pixel::from_hex_string("00ff00").unwrap())),
        );
        item.transform(&Transform::Stroke(None)).unwrap();
        svg_holder.add_item(item);

        let mut item = SvgItem::new(
            rect(Point::new(0.0, 50.0), Point::new(50.0, 50.0)),
            Some(ColorLike::Color(Pixel::from_hex_string("0000ff").unwrap())),
        );
        item.transform(&Transform::Stroke(None)).unwrap();
        svg_holder.add_item(item);

        let mut item = SvgItem::new(
            circle(Point::new(75.0, 75.0), 25.0),
            Some(ColorLike::Color(Pixel::from_hex_string("fff00f").unwrap())),
        );
        item.transform(&Transform::Stroke(None)).unwrap();
        svg_holder.add_item(item);
    };

    let image_render = DummyRender::default();

    let buffer = image_render.render_frame(&composition);
    if let Ok(buffer) = buffer {
        // Corners
        assert_eq!(buffer.pixel_unchecked(0, 0), &Pixel::new(255, 0, 0, 255));
        assert_eq!(buffer.pixel_unchecked(99, 0), &Pixel::new(0, 255, 0, 255));
        assert_eq!(buffer.pixel_unchecked(0, 99), &Pixel::new(0, 0, 255, 255));
        assert_eq!(buffer.pixel_unchecked(99, 99), &Pixel::new(0, 0, 0, 0));

        // Middle
        assert_eq!(buffer.pixel_unchecked(24, 24), &Pixel::new(255, 0, 0, 255));
        assert_eq!(buffer.pixel_unchecked(74, 24), &Pixel::new(0, 255, 0, 255));
        assert_eq!(buffer.pixel_unchecked(24, 74), &Pixel::new(0, 0, 255, 255));
        assert_eq!(
            buffer.pixel_unchecked(74, 74),
            &Pixel::new(255, 240, 15, 255)
        );
    } else {
        assert!(false);
    }
}
