use rusvid_core::holder::likes::{ColorLike, TypesLike};
use rusvid_core::holder::svg_holder::SvgItem;
use rusvid_lib::figures::prelude::*;
use rusvid_lib::prelude::*;

fn main() {
    let resolution = Resolution::HD;

    let mut composition = Composition::builder()
        .resolution(resolution)
        .framerate(24)
        .duration(2)
        .build();

    let layer = composition.create_new_layer(LayerType::Svg).unwrap();

    let rect_size = Point::new(250.0, 250.0);
    let pixel_position = (resolution.as_point() / 2.0) - (rect_size / 2.0);
    if let TypesLike::Svg(svg_data) = layer.object.data_mut() {
        let fill = Some(ColorLike::Color(Pixel::from_hex_string("#1212FF").unwrap()));

        let rect = SvgItem::new(rect(pixel_position, rect_size), fill);

        svg_data.add_item(rect);
    }

    let size = resolution.x() / 10.0;
    let layer = composition.create_new_layer(LayerType::Svg).unwrap();
    if let TypesLike::Svg(svg_data) = layer.object.data_mut() {
        let fill = Some(ColorLike::Color(Pixel::from_hex_string("#FF1212").unwrap()));

        let rect = SvgItem::new(circle(Point::new_symmetric(size), size), fill);

        svg_data.add_item(rect);
    }

    let mut renderer = EmbeddedRenderer::new("layers.mp4");
    renderer.render(composition).unwrap()
}
