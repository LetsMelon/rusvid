use std::fs::File;

mod wrapper;

use rusvid_lib::prelude::FrameRenderer;
use rusvid_lib::renderer::Renderer;
use wrapper::*;

fn main() {
    let comp = Composition {
        framerate: 24,
        resolution: Resolution {
            width: 1920,
            height: 1080,
        },
        layers: vec![
            Layer {
                objects: vec![Object {
                    name: "obj_1".to_string(),
                    path: "M0,0 L100,0 L100,100 L0,100 Z".to_string(),
                    color: "200,10,95,255".to_string(),
                }],
            },
            Layer {
                objects: vec![Object {
                    name: "obj_2".to_string(),
                    path: "M200,200 L300,200 L300,300 L200,300 Z".to_string(),
                    color: "0,255,100,255".to_string(),
                }],
            },
        ],
    };

    let file = File::create("out.yaml").unwrap();
    serde_yaml::to_writer(file, &comp).unwrap();

    let mut renderer =
        FrameRenderer::new_with_file_type("./out", rusvid_lib::prelude::FrameImageFormat::Bmp);
    renderer.render(comp.translate()).unwrap();
}
