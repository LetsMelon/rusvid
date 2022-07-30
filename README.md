# rusvid

Write and render svg-animations with Rust ✨

(no gui or cli, under active development)

## Dependencies

- Ffmpeg in path
- rustc 1.63.0-nightly

## Simple usage

Add `rusvid_lib` to `cargo.toml`

```toml
[dependencies]
rusvid_lib = "0.1.0"
```

and copy the following code into `main.rs`

```rust
use rusvid_lib::prelude::*;
use rusvid_lib::usvg::{
    BaseGradient, Color, LinearGradient, NodeKind, Opacity, Paint, Path, SpreadMethod, Stop,
    StopOffset, Stroke, StrokeWidth, Transform, Units,
};
use std::path::PathBuf;
use std::rc::Rc;

fn main() {
    let mut composition = Composition::builder()
        .resolution(Resolution::FHD)
        .framerate(30)
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

    let pixel_position = animation::Points::Point2d(20.0, 20.0);
    composition.add_to_root(NodeKind::Path(Path {
        id: "rect".to_string(),
        fill: match composition.fill_with_link("lg1") {
            None => None,
            Some(mut f) => {
                f.opacity = Opacity::new(0.75);
                Some(f)
            }
        },
        data: Rc::new(figures::rect(
            pixel_position.x(),
            pixel_position.y(),
            composition.resolution().width() as f64 / 2.0,
            composition.resolution().height() as f64 / 3.0,
        )),
        ..Path::default()
    }));

    let out_path = PathBuf::from("out.mp4");
    let tmp_path = PathBuf::from("./out");

    let animation_1 = animation::PositionAnimation::new(
        "rect".to_string(),
        animation::functions::Linear::new(0, 200, pixel_position, (1250.0, 500.0).into()).unwrap(),
    );

    let mut renderer = FfmpegRenderer::new(out_path, tmp_path);
    renderer.set_image_render(PngRender::new());
    composition.animations.add_animation(animation_1);
    renderer.render(composition).unwrap()
}
```

after that render the animation with `cargo run -r`.

It is recommended to always run the program in release-mode otherwise because it's to slow.
