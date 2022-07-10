use crate::renderer::build_command;
use anyhow::Result;
use debug_ignore::DebugIgnore;
use resvg::render;
use std::fs;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::rc::Rc;
use tiny_skia::{Pixmap, Transform};
use usvg::{AspectRatio, FitTo, PathData, Size, Svg, Transform as UsvgTransform, Tree, ViewBox};

use crate::metrics::{MetricsSize, MetricsVideo};
use crate::resolution::Resolution;

#[derive(Debug)]
pub struct Composition {
    /// The resolution of the composition
    resolution: Resolution,

    /// The fixed framerate of the composition in `frames per seconds`
    pub framerate: u8,

    /// The duration of the composition in seconds
    pub duration: u16,

    pub name: String,

    rtree: DebugIgnore<Tree>,
}

#[derive(Debug)]
pub struct CompositionBuilder {
    resolution: Resolution,
    framerate: u8,
    duration: u16,
    name: String,
}

impl Default for CompositionBuilder {
    fn default() -> Self {
        let res = Resolution::default();

        CompositionBuilder {
            resolution: res,
            framerate: 30,
            duration: 10,
            name: "UNKNOWN".to_string(),
        }
    }
}

impl CompositionBuilder {
    fn create_tree_from_resolution(resolution: Resolution) -> Tree {
        let size = Size::new(resolution.width() as f64, resolution.height() as f64).unwrap();

        Tree::create(Svg {
            size,
            view_box: ViewBox {
                rect: size.to_rect(0.0, 0.0),
                aspect: AspectRatio::default(),
            },
        })
    }

    pub fn build(self) -> Composition {
        Composition {
            resolution: self.resolution,
            framerate: self.framerate,
            duration: self.duration,
            name: self.name,
            rtree: DebugIgnore(CompositionBuilder::create_tree_from_resolution(
                self.resolution,
            )),
        }
    }

    pub fn framerate(mut self, framerate: u8) -> Self {
        self.framerate = framerate;
        self
    }

    pub fn resolution(mut self, resolution: Resolution) -> Self {
        self.resolution = resolution;
        self
    }

    pub fn duration(mut self, duration: u16) -> Self {
        self.duration = duration;
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
}

impl Composition {
    pub fn builder() -> CompositionBuilder {
        CompositionBuilder::default()
    }

    pub fn resolution(&self) -> Resolution {
        self.resolution
    }

    pub fn rtree(&self) -> &Tree {
        self.rtree.deref()
    }

    pub fn rtree_mut(&mut self) -> &mut Tree {
        self.rtree.deref_mut()
    }

    pub fn render_single(&self, path: &Path) -> Result<()> {
        let pixmap_size = self.rtree().svg_node().size.to_screen_size();

        let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height())
            .expect("Error while creating pixmap");
        render(
            self.rtree(),
            FitTo::Original,
            Transform::default(),
            pixmap.as_mut(),
        )
        .expect("Error while rendering");

        pixmap.save_png(path)?;

        Ok(())
    }

    pub fn render(
        &self,
        out_path: &Path,
        tmp_path: &Path,
        mut box_position: Rc<PathData>,
    ) -> Result<()> {
        if tmp_path.exists() {
            fs::remove_dir_all(tmp_path)?;
        }
        fs::create_dir(tmp_path)?;

        let frames = self.frames();
        for i in 0..frames {
            println!("{:03}/{:03}", i + 1, frames);

            let filename = format!("{}.png", i + 1);
            let file_path = tmp_path.join(Path::new(&filename));
            self.render_single(file_path.as_path())?;

            // TODO: make safe
            // Test 1:
            // let mut reference_position = box_position.borrow_mut();
            // reference_position.transform(UsvgTransform::new_translate(5.0, 4.0));
            unsafe {
                let pd = Rc::get_mut_unchecked(&mut box_position);
                pd.transform(UsvgTransform::new_translate(5.0, 4.0));
            }
        }

        let mut command = build_command(tmp_path, out_path, self.framerate)?;

        if out_path.exists() {
            fs::remove_file(out_path)?;
        }

        command.output()?;
        println!("Saved as: {}", out_path.to_str().unwrap());

        Ok(())
    }
}

impl Default for Composition {
    fn default() -> Self {
        Composition::builder().build()
    }
}

impl MetricsVideo for Composition {
    fn frames(&self) -> usize {
        (self.framerate as u16 * self.duration) as usize
    }

    fn pixels(&self) -> usize {
        self.frames() * self.resolution.pixels()
    }
}

impl MetricsSize for Composition {
    fn bytes(&self) -> usize {
        let frames = self.frames();
        let per_frame_bytes = self.resolution.bytes();

        frames * per_frame_bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::composition::Composition;
    use crate::resolution::Resolution;

    #[test]
    fn takes_arguments_and_build_composition() {
        let comp = Composition::builder()
            .name("test")
            .resolution(Resolution::HD)
            .duration(15)
            .framerate(5)
            .build();

        assert_eq!(comp.resolution, Resolution::HD);
        assert_eq!(comp.framerate, 5);
        assert_eq!(comp.duration, 15);
        assert_eq!(comp.name, "test".to_string());
    }
}
