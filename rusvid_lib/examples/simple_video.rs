use std::path::PathBuf;

use rusvid_core::holder::likes::{ColorLike, TypesLike};
use rusvid_core::holder::svg_item::SvgItem;
use rusvid_lib::figures::prelude::*;
use rusvid_lib::prelude::*;

fn main() {
    let resolution = Resolution::HD;

    let mut composition = Composition::builder()
        .resolution(resolution)
        .framerate(24)
        .duration(2)
        .build();

    let layer = composition.create_layer(LayerType::Svg).unwrap();

    let rect_size = Point::new(250.0, 250.0);
    let pixel_position = (resolution.as_point() / 2.0) - (rect_size / 2.0);
    if let TypesLike::Svg(svg_holder) = layer.object.data_mut() {
        let fill = Some(ColorLike::Color(Pixel::from_hex_string("#1212FF").unwrap()));

        let rect = SvgItem::new(rect(pixel_position, rect_size), fill);

        svg_holder.add_item(rect);
    }

    let out_path = PathBuf::from("simple_path.mp4");

    let mut renderer = EmbeddedRenderer::new(out_path);
    renderer.render(composition).unwrap()
}
