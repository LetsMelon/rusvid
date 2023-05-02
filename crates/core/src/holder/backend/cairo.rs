use cairo::{Antialias, Context, Format, ImageSurface};

use crate::holder::backend::Backend;
use crate::holder::object::Object;
use crate::holder::utils::ApplyToCairoContext;
use crate::pixel::Pixel;
use crate::plane::{Plane, PlaneError, PlaneResult, SIZE};

#[derive(Debug)]
pub struct CairoBackend {}

impl Backend for CairoBackend {
    fn name() -> &'static str {
        "cairo"
    }

    fn render(object: &Object, width: SIZE, height: SIZE) -> PlaneResult<Plane> {
        let surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32)
            .map_err(|err| PlaneError::Backend(format!("{err:?}")))?;
        let context =
            Context::new(&surface).map_err(|err| PlaneError::Backend(format!("{err:?}")))?;

        context.set_source_rgb(1.0, 1.0, 1.0);
        context.paint().unwrap();
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

/*
pub fn test_my_thing(width: SIZE, height: SIZE) -> Result<Plane, PlaneError> {
    let surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32).unwrap();
    let context = Context::new(&surface).unwrap();

    context.set_source_rgb(1.0, 1.0, 1.0);
    context.paint().unwrap();

    context.set_line_width(5.0);
    context.set_antialias(Antialias::Best);
    context.set_dash(vec![4.0, 4.0].as_slice(), 0.0);

    roundedrec_moonlight(&context, 50.0, 30.0, 100.0, 120.0, 25.0, 25.0);

    context.set_source_rgb(1.0, 0.0, 0.0);
    context.stroke_preserve().unwrap();
    context.set_source_rgb(0.5, 0.75, 0.36);
    context.fill().unwrap();

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

fn roundedrec_moonlight(
    context: &Context,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    radius_x: f64,
    radius_y: f64,
) {
    const ARC_TO_BEZIER: f64 = 0.55228475;

    let radius_x = if radius_x > w - radius_x {
        w / 2.0
    } else {
        radius_x
    };
    let radius_y = if radius_y > h - radius_y {
        h / 2.0
    } else {
        radius_y
    };

    let c1 = ARC_TO_BEZIER * radius_x;
    let c2 = ARC_TO_BEZIER * radius_y;

    context.new_path();
    context.move_to(x + radius_x, y);
    context.rel_line_to(w - 2.0 * radius_x, 0.0);
    context.rel_curve_to(c1, 0.0, radius_x, c2, radius_x, radius_y);
    context.rel_line_to(0.0, h - 2.0 * radius_y);
    context.rel_curve_to(0.0, c2, c1 - radius_x, radius_y, -radius_x, radius_y);
    context.rel_line_to(-w + 2.0 * radius_x, 0.0);
    context.rel_curve_to(-c1, 0.0, -radius_x, -c2, -radius_x, -radius_y);
    context.rel_line_to(0.0, -h + 2.0 * radius_y);
    context.rel_curve_to(0.0, -c2, radius_x - c1, -radius_y, radius_x, -radius_y);
    context.close_path();
}
 */
