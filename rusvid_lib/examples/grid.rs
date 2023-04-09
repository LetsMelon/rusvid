use rusvid_core::holder::gradient::base::BaseGradient;
use rusvid_core::holder::gradient::linear::LinearGradient;
use rusvid_core::holder::likes::{ColorLike, TypesLike};
use rusvid_core::holder::svg_item::SvgItem;
use rusvid_lib::animation::{AnimationType, EaseType, FunctionType};
use rusvid_lib::figures::prelude::*;
use rusvid_lib::prelude::*;

fn main() {
    let resolution = Resolution::FHD;

    let mut composition = Composition::builder()
        .resolution(resolution)
        .framerate(30)
        .duration(5)
        .build();

    let frames = composition.frames();

    let background_layer = composition.create_layer(LayerType::Svg).unwrap();

    let start_pos = resolution.as_point() * Point::NEG_ONE;
    let bg_id = if let TypesLike::Svg(svg_data) = background_layer.object.data_mut() {
        let fill = Some(ColorLike::LinearGradient(LinearGradient::new(
            BaseGradient::new_from_colors(vec![
                Pixel::new(0, 215, 255, 255),
                Pixel::new(9, 9, 121, 255),
                Pixel::new(0, 215, 255, 255),
            ]),
        )));

        let rect = SvgItem::new(rect(start_pos, resolution.as_point() * 2.0), fill);

        svg_data.add_item(rect)
    } else {
        panic!("Can't add a svg to the layer")
    };
    background_layer.add_animation(AnimationType::Position(PositionAnimation::new(
        &bg_id,
        (0, frames),
        (start_pos, Point::ZERO),
        FunctionType::Linear,
        EaseType::default(),
    )));

    let grid_layer = composition.create_layer(LayerType::Svg).unwrap();

    let grid_size = Point::new(32.0, 18.0);
    let margin = Point::new(5.0, 5.0);
    let rect_size = (resolution.as_point() - (margin * (grid_size + Point::ONE))) / grid_size;

    for x in 0..(grid_size.x() as usize) {
        for y in 0..(grid_size.y() as usize) {
            let coordinates_as_point = Point::new(x as f64, y as f64);
            let extra_margin = margin * (coordinates_as_point + Point::ONE);
            let rect_pos = coordinates_as_point * rect_size + extra_margin;

            if let TypesLike::Svg(svg_data) = grid_layer.object.data_mut() {
                let rect = SvgItem::new(
                    rect(rect_pos, rect_size),
                    Some(ColorLike::Color(Pixel::BLACK)),
                );

                svg_data.add_item(rect);
            }
        }
    }

    let mut renderer = EmbeddedRenderer::new("grid.mp4");
    renderer.render(composition).unwrap()
}
