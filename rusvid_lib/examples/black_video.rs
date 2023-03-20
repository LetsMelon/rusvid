use rusvid_lib::prelude::*;

fn main() {
    let resolution = Resolution::HD;

    let mut composition = Composition::builder()
        .resolution(resolution)
        .framerate(24)
        .duration(2)
        .build();

    let _ = composition.create_layer(LayerType::Svg).unwrap();

    let mut renderer = EmbeddedRenderer::new("black_video.mp4");
    renderer.render(composition).unwrap();
}
