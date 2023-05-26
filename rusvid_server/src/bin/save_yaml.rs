use std::fs::File;

use rusvid_core::holder::gradient::base::BaseGradient;
use rusvid_core::holder::gradient::linear::LinearGradient;
use rusvid_core::holder::likes::ColorLike;
use rusvid_core::holder::svg_item::SvgItem;
use rusvid_core::holder::transform::{Transform, TransformLogic};
use rusvid_lib::animation::{EaseType, FunctionType};
use rusvid_lib::figures::circle::circle;
use rusvid_lib::figures::rect::rect;
use rusvid_lib::figures::triangle::equilateral_triangle;
use rusvid_lib::prelude::*;

fn main() {
    let resolution = Resolution::FourK;

    let mut composition = Composition::builder()
        .resolution(resolution)
        .framerate(30)
        .duration(3)
        .build();

    let layer = composition.create_layer(LayerType::Svg).unwrap();

    layer
        .add_svg_item({
            let fill = Some(ColorLike::LinearGradient(LinearGradient::new(
                BaseGradient::new_from_colors(vec![
                    Pixel::new(0, 255, 0, 255),
                    Pixel::new(0, 0, 255, 255),
                ]),
            )));

            let mut item =
                SvgItem::new(equilateral_triangle(Point::new(400.0, 400.0), 350.0), fill);

            item.transform(&Transform::Rotate(2.5)).unwrap();

            item
        })
        .unwrap();

    let circle_position = Point::new(700.0, 850.0);
    let circle_id = layer
        .add_svg_item({
            let fill = Some(ColorLike::LinearGradient(LinearGradient::new(
                BaseGradient::new_from_colors(vec![
                    Pixel::from_hex_string("9769f0").unwrap(),
                    Pixel::from_hex_string("fbc7d4").unwrap(),
                ]),
            )));

            let item = SvgItem::new(circle(circle_position, 250.0), fill);

            item
        })
        .unwrap();

    let rect_id = layer
        .add_svg_item({
            let fill = Some(ColorLike::LinearGradient(LinearGradient::new(
                BaseGradient::new_from_colors(vec![
                    Pixel::new(0, 255, 0, 155),
                    Pixel::new(0, 0, 255, 155),
                ]),
            )));

            let item = SvgItem::new(
                rect(
                    Point::new_symmetric(20.0),
                    resolution.as_point() / Point::new(2.0, 3.0),
                ),
                fill,
            );

            item
        })
        .unwrap();

    layer.add_position_animation(PositionAnimation::new(
        &rect_id,
        0..200,
        (Point::new_symmetric(20.0), Point::new(1250.0, 500.0)),
        FunctionType::Linear,
        EaseType::default(),
    ));

    layer.add_position_animation(PositionAnimation::new(
        &rect_id,
        220..290,
        (Point::new(1250.0, 500.0), Point::ZERO),
        FunctionType::Linear,
        EaseType::default(),
    ));

    layer.add_animation(AnimationType::Position(PositionAnimation::new(
        &circle_id,
        0..90,
        (circle_position, resolution.as_point() / 2.0),
        FunctionType::Sine,
        EaseType::default(),
    )));

    layer.add_animation(AnimationType::ChangeColor(ChangeColorAnimation::new(
        &rect_id,
        (0, 100),
        (Pixel::new(255, 100, 0, 255), Pixel::new(255, 0, 255, 255)),
        FunctionType::Sine,
        EaseType::InOut,
    )));

    let mut file = File::create("composition.yaml").unwrap();
    serde_yaml::to_writer(&mut file, &composition).unwrap();
}
