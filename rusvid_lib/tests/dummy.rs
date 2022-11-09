use anyhow::Result;
use rusvid_core::plane::Plane;
use rusvid_lib::composition::Composition;
use rusvid_lib::renderer::Renderer;

#[derive(Default)]
pub struct DummyRender {}

impl Renderer for DummyRender {
    fn render(&mut self, _: Composition) -> Result<()> {
        todo!()
    }

    fn out_path(&self) -> &std::path::Path {
        todo!()
    }

    fn tmp_dir_path(&self) -> &std::path::Path {
        todo!()
    }
}

impl DummyRender {
    pub fn render_frame(&self, composition: &Composition) -> Result<Plane> {
        self.render_single(composition)
    }
}
