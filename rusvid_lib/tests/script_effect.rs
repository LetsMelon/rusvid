use rusvid_core::holder::likes::TypesLike;
use rusvid_core::holder::svg_holder::SvgItem;
use rusvid_core::pixel::Pixel;
use rusvid_core::point::Point;
use rusvid_lib::composition::Composition;
use rusvid_lib::figures::rect::rect;
use rusvid_lib::prelude::{LayerType, ScriptingEffect};
use rusvid_lib::resolution::Resolution;
use rusvid_lib::types::AsPoint;

mod dummy;

use dummy::DummyRender;

// TODO replace `min` & `max` with builtin functions, see https://github.com/rhaiscript/rhai/pull/702
const SCRIPT: &'static str = "
const WIDTH = width().to_float() - 1.0;
const HEIGHT = height().to_float() - 1.0;

fn min(x,y) {
    if (x < y) {
        x
    } else {
        y
    }
}

fn max(x,y) {
    if (x < y) {
        y
    } else {
        x
    }
}

fn constrain(value, min, max) {
    max(min(value, max), min)
}

fn width_height_gradient(x, y) {
    let r = constrain(((x.to_float() / global::WIDTH) * 255.0).to_int(), 0, 255);
    let g = constrain(((y.to_float() / global::HEIGHT) * 255.0).to_int(), 0, 255);

    pixel(r, g, 0, 255)
}
";

#[test]
fn script_effect() {
    let resolution = Resolution::Custom(10, 10);
    let mut composition = Composition::builder()
        .resolution(resolution)
        .framerate(1)
        .duration(1)
        .add_effect(ScriptingEffect::new("width_height_gradient", SCRIPT))
        .build();

    let layer = composition.create_new_layer(LayerType::Svg).unwrap();
    if let TypesLike::Svg(svg_holder) = layer.object.data_mut() {
        svg_holder.add_item(SvgItem::new(rect(Point::ZERO, resolution.as_point()), None));
    }

    let image_render = DummyRender::default();

    let buffer = image_render.render_frame(&composition);
    if let Ok(buffer) = buffer {
        let data = buffer.as_data();

        #[rustfmt::skip]
        let mock_data = vec![
            Pixel::new(0, 0, 0, 255), Pixel::new(0, 28, 0, 255), Pixel::new(0, 56, 0, 255),
            Pixel::new(0, 85, 0, 255), Pixel::new(0, 113, 0, 255), Pixel::new(0, 141, 0, 255),
            Pixel::new(0, 170, 0, 255), Pixel::new(0, 198, 0, 255), Pixel::new(0, 226, 0, 255),
            Pixel::new(0, 255, 0, 255), Pixel::new(28, 0, 0, 255), Pixel::new(28, 28, 0, 255),
            Pixel::new(28, 56, 0, 255), Pixel::new(28, 85, 0, 255), Pixel::new(28, 113, 0, 255),
            Pixel::new(28, 141, 0, 255), Pixel::new(28, 170, 0, 255), Pixel::new(28, 198, 0, 255),
            Pixel::new(28, 226, 0, 255), Pixel::new(28, 255, 0, 255), Pixel::new(56, 0, 0, 255),
            Pixel::new(56, 28, 0, 255), Pixel::new(56, 56, 0, 255), Pixel::new(56, 85, 0, 255),
            Pixel::new(56, 113, 0, 255), Pixel::new(56, 141, 0, 255), Pixel::new(56, 170, 0, 255),
            Pixel::new(56, 198, 0, 255), Pixel::new(56, 226, 0, 255), Pixel::new(56, 255, 0, 255),
            Pixel::new(85, 0, 0, 255), Pixel::new(85, 28, 0, 255), Pixel::new(85, 56, 0, 255),
            Pixel::new(85, 85, 0, 255), Pixel::new(85, 113, 0, 255), Pixel::new(85, 141, 0, 255),
            Pixel::new(85, 170, 0, 255), Pixel::new(85, 198, 0, 255), Pixel::new(85, 226, 0, 255),
            Pixel::new(85, 255, 0, 255), Pixel::new(113, 0, 0, 255), Pixel::new(113, 28, 0, 255),
            Pixel::new(113, 56, 0, 255), Pixel::new(113, 85, 0, 255), Pixel::new(113, 113, 0, 255),
            Pixel::new(113, 141, 0, 255), Pixel::new(113, 170, 0, 255), Pixel::new(113, 198, 0, 255),
            Pixel::new(113, 226, 0, 255), Pixel::new(113, 255, 0, 255), Pixel::new(141, 0, 0, 255),
            Pixel::new(141, 28, 0, 255), Pixel::new(141, 56, 0, 255), Pixel::new(141, 85, 0, 255),
            Pixel::new(141, 113, 0, 255), Pixel::new(141, 141, 0, 255), Pixel::new(141, 170, 0, 255),
            Pixel::new(141, 198, 0, 255), Pixel::new(141, 226, 0, 255), Pixel::new(141, 255, 0, 255),
            Pixel::new(170, 0, 0, 255), Pixel::new(170, 28, 0, 255), Pixel::new(170, 56, 0, 255),
            Pixel::new(170, 85, 0, 255), Pixel::new(170, 113, 0, 255), Pixel::new(170, 141, 0, 255),
            Pixel::new(170, 170, 0, 255), Pixel::new(170, 198, 0, 255), Pixel::new(170, 226, 0, 255),
            Pixel::new(170, 255, 0, 255), Pixel::new(198, 0, 0, 255), Pixel::new(198, 28, 0, 255),
            Pixel::new(198, 56, 0, 255), Pixel::new(198, 85, 0, 255), Pixel::new(198, 113, 0, 255),
            Pixel::new(198, 141, 0, 255), Pixel::new(198, 170, 0, 255), Pixel::new(198, 198, 0, 255),
            Pixel::new(198, 226, 0, 255), Pixel::new(198, 255, 0, 255), Pixel::new(226, 0, 0, 255),
            Pixel::new(226, 28, 0, 255), Pixel::new(226, 56, 0, 255), Pixel::new(226, 85, 0, 255),
            Pixel::new(226, 113, 0, 255), Pixel::new(226, 141, 0, 255), Pixel::new(226, 170, 0, 255),
            Pixel::new(226, 198, 0, 255), Pixel::new(226, 226, 0, 255), Pixel::new(226, 255, 0, 255),
            Pixel::new(255, 0, 0, 255), Pixel::new(255, 28, 0, 255), Pixel::new(255, 56, 0, 255),
            Pixel::new(255, 85, 0, 255), Pixel::new(255, 113, 0, 255), Pixel::new(255, 141, 0, 255),
            Pixel::new(255, 170, 0, 255), Pixel::new(255, 198, 0, 255), Pixel::new(255, 226, 0, 255),
            Pixel::new(255, 255, 0, 255),
        ];

        for (a, b) in data.iter().zip(mock_data) {
            assert_eq!(*a, b);
        }
    } else {
        assert!(false);
    }
}
