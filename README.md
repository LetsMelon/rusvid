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
use rusvid_lib::usvg::*;
use rusvid_lib::utils::color_from_hex;
use std::path::PathBuf;
use std::rc::Rc;

fn main() {
    let mut composition = Composition::builder()
        .resolution(Resolution::FourK)
        .framerate(60)
        .duration(5)
        .build();

    let mut layer = Layer::new(composition.resolution());
    layer
        .add_to_defs(NodeKind::LinearGradient(LinearGradient {
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
        }))
        .unwrap();

    let circle_position = animation::Points::Point2d(700.0, 850.0);
    layer
        .add_to_root(NodeKind::Path(Path {
            id: "circle".to_string(),
            stroke: Some(Stroke {
                paint: Paint::Link("lg2".into()),
                width: StrokeWidth::new(10.0),
                ..Stroke::default()
            }),
            rendering_mode: Default::default(),
            data: Rc::new(figures::circle(
                circle_position.x(),
                circle_position.y(),
                600.0,
            )),
            ..Path::default()
        }))
        .unwrap();
    layer.add_animation(animation::PositionAnimation::new(
        "circle".to_string(),
        animation::functions::S::new(
            0,
            90,
            circle_position,
            animation::Points::Point2d(
                composition.resolution().width() as f64 / 2.0,
                composition.resolution().height() as f64 / 2.0,
            ),
        )
        .unwrap(),
    ));
    composition.add_layer(layer);

    let out_path = PathBuf::from("out.mp4");
    let tmp_path = PathBuf::from("./out");

    let mut renderer = FfmpegRenderer::new(out_path, tmp_path);
    renderer.set_image_render(PngRender::new());
    renderer.render(composition).unwrap()
}
```

after that render the animation with `cargo run -r`.

It is recommended to always run the program in release-mode.
