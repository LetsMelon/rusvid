use rusvid_core::holder::likes::{ColorLike, TypesLike};
use rusvid_core::holder::svg_holder::SvgItem;
use rusvid_lib::animation::prelude::*;
use rusvid_lib::animation::AnimationType;
use rusvid_lib::figures::prelude::*;
use rusvid_lib::prelude::*;

fn main() {
    let resolution = Resolution::HD;

    let mut composition = Composition::builder()
        .resolution(resolution)
        .framerate(30)
        .duration(2)
        .build();
    let frames = composition.frames();

    let layer = composition.create_new_layer(LayerType::Svg).unwrap();

    let rect_size = Point::new(250.0, 250.0);
    let rect_pos = Point::ZERO;
    let rect_id = if let TypesLike::Svg(svg_holder) = layer.object.data_mut() {
        let fill = Some(ColorLike::Color(Pixel::from_hex_string("#1212FF").unwrap()));

        let rect = SvgItem::new(rect(rect_pos, rect_size), fill);

        svg_holder.add_item(rect)
    } else {
        panic!("Can't add a svg to the layer")
    };

    layer.add_animation(AnimationType::Position(PositionAnimation::new(
        &rect_id,
        (0, frames),
        (rect_pos, resolution.as_point() - rect_pos),
        Cubic::new_with_ease_type(EaseType::InOut),
    )));

    let mut renderer = EmbeddedRenderer::new("simple_animation.mp4");
    renderer.render(composition).unwrap()
}
