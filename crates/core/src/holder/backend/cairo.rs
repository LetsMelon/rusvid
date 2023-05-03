use cairo::{Antialias, Context, Format, ImageSurface};

use crate::holder::backend::Backend;
use crate::holder::object::Object;
use crate::holder::utils::ApplyToCairoContext;
use crate::pixel::Pixel;
use crate::plane::{Plane, PlaneError, PlaneResult, SIZE};

#[derive(Debug)]
pub struct CairoBackend {}

impl Backend for CairoBackend {
    fn name(&self) -> &'static str {
        "cairo"
    }

    fn render(&self, object: &Object, width: SIZE, height: SIZE) -> PlaneResult<Plane> {
        let surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32)
            .map_err(|err| PlaneError::Backend(format!("{err:?}")))?;
        let context =
            Context::new(&surface).map_err(|err| PlaneError::Backend(format!("{err:?}")))?;

        context.set_antialias(Antialias::Best);

        object
            .apply(&context)
            .map_err(|err| PlaneError::Backend(format!("{err:?}")))?;

        drop(context);

        let data = surface.take_data().unwrap();

        Plane::from_data(
            width,
            height,
            data.chunks_exact(4)
                .map(|raw| Pixel::new(raw[2], raw[1], raw[0], raw[3]))
                .collect::<Vec<_>>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::holder::likes::color_like::ColorLike;
    use crate::holder::likes::path_like::PathLike;
    use crate::holder::likes::types_like::TypesLike;
    use crate::holder::object::Object;
    use crate::holder::svg_holder::SvgHolder;
    use crate::holder::svg_item::SvgItem;
    use crate::pixel::Pixel;
    use crate::point::Point;

    #[test]
    fn just_works() {
        let mut svg = SvgHolder::new();

        let rect_size = Point::new_symmetric(150.0);
        let rect_pos = Point::new(100.0, 50.0);
        let mut rect = SvgItem::new(
            vec![
                PathLike::Move(rect_pos),
                PathLike::Line(rect_size * Point::new(1.0, 0.0) + rect_pos),
                PathLike::Line(rect_size * Point::new(1.0, 1.0) + rect_pos),
                PathLike::Line(rect_size * Point::new(0.0, 1.0) + rect_pos),
                PathLike::Close,
            ],
            Some(ColorLike::Color(Pixel::new(255, 100, 125, 255))),
        );
        rect.stroke = None;

        svg.add_item(rect);

        let object = Object::new(TypesLike::Svg(svg));

        let plane = object.render(300, 300).unwrap();
        plane.save_as_png("my_special_output.png").unwrap();
    }
}
