use rusvid_lib::composition::Composition;
use rusvid_lib::figures::circle::circle;
use rusvid_lib::figures::rect::rect;
use rusvid_lib::figures::triangle::equilateral_triangle;
use rusvid_lib::renderer::ffmpeg::FfmpegRenderer;
use rusvid_lib::renderer::png::PngRender;
use rusvid_lib::renderer::Renderer;
use rusvid_lib::resolution::Resolution;
use rusvid_lib::usvg::{
    BaseGradient, Color, LinearGradient, NodeKind, Opacity, Paint, Path, SpreadMethod, Stop,
    StopOffset, Stroke, StrokeWidth, Transform, Units,
};
use rusvid_lib::utils::color_from_hex;
use std::path::PathBuf;

use rusvid_lib::renderer::raw::RawRender;
use std::rc::Rc;

fn main() {
    let mut composition = Composition::builder()
        .resolution(Resolution::FourK)
        .framerate(60)
        .duration(5)
        .build();

    composition.add_to_defs(NodeKind::LinearGradient(LinearGradient {
        id: "lg1".into(),
        x1: 0.0,
        y1: 0.0,
        x2: 1.0,
        y2: 0.0,
        base: BaseGradient {
            units: Units::ObjectBoundingBox,
            transform: Transform::default(),
            spread_method: SpreadMethod::Pad,
            stops: vec![
                Stop {
                    offset: StopOffset::new(0.0),
                    color: Color::new_rgb(0, 255, 0),
                    opacity: Opacity::new(1.0),
                },
                Stop {
                    offset: StopOffset::new(1.0),
                    color: Color::new_rgb(0, 0, 255),
                    opacity: Opacity::new(1.0),
                },
            ],
        },
    }));
    composition.add_to_defs(NodeKind::LinearGradient(LinearGradient {
        id: "lg2".into(),
        x1: 0.0,
        y1: 0.0,
        x2: 1.0,
        y2: 0.0,
        base: BaseGradient {
            units: Units::ObjectBoundingBox,
            transform: Transform::default(),
            spread_method: SpreadMethod::Pad,
            stops: vec![
                Stop {
                    offset: StopOffset::new(0.0),
                    color: color_from_hex("9796f0".to_string()).unwrap(),
                    opacity: Opacity::new(1.0),
                },
                Stop {
                    offset: StopOffset::new(1.0),
                    color: color_from_hex("fbc7d4".to_string()).unwrap(),
                    opacity: Opacity::new(1.0),
                },
            ],
        },
    }));

    composition.add_to_root(NodeKind::Path(Path {
        stroke: Some(Stroke {
            paint: Paint::Link("lg2".into()),
            width: StrokeWidth::new(10.0),
            ..Stroke::default()
        }),
        rendering_mode: Default::default(),
        data: Rc::new(circle(700.0, 850.0, 600.0)),
        ..Path::default()
    }));

    let mut path = equilateral_triangle(400.0, 400.0, 350.0);
    path.transform(Transform::new_rotate(2.5));
    composition.add_to_root(NodeKind::Path(Path {
        fill: composition.fill_with_link("lg1"),
        data: Rc::new(path),
        ..Path::default()
    }));

    let position = Rc::new(rect(
        20.0,
        20.0,
        composition.resolution().width() as f64 / 2.0,
        composition.resolution().height() as f64 / 3.0,
    ));

    composition.add_to_root(NodeKind::Path(Path {
        fill: match composition.fill_with_link("lg1") {
            None => None,
            Some(mut f) => {
                f.opacity = Opacity::new(0.75);
                Some(f)
            }
        },
        data: Rc::clone(&position),
        ..Path::default()
    }));

    let out_path = PathBuf::from("out.mp4");
    let tmp_path = PathBuf::from("./out");

    // TODO add builder pattern for video- & image-render
    let mut renderer = FfmpegRenderer::new(out_path, tmp_path.clone());
    renderer.set_image_render(Box::new(RawRender::new()));
    renderer.render(composition, position).unwrap()
}
