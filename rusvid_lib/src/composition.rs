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

impl Composition {
    pub fn new(name: String, resolution: Resolution) -> Composition {
        Composition {
            name,
            rtree: DebugIgnore(Composition::create_tree_from_resolution(resolution)),
            resolution,
            ..Composition::default()
        }
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
        let exists = fs::try_exists(tmp_path)?;
        if exists {
            fs::remove_dir_all(tmp_path)?;
        }
        fs::create_dir(tmp_path)?;

        let frames = self.calculate_frames();
        for i in 0..frames {
            println!("{}/{}", i + 1, frames);

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

        let exists = fs::try_exists(out_path)?;
        if exists {
            fs::remove_file(out_path)?;
        }

        command.output()?;
        println!("Saved as: {}", out_path.to_str().unwrap());

        Ok(())
    }
}

impl Default for Composition {
    fn default() -> Self {
        let res = Resolution::default();

        Composition {
            resolution: res,
            framerate: 30,
            duration: 10,
            name: "UNKNOWN".to_string(),
            rtree: DebugIgnore(Composition::create_tree_from_resolution(res)),
        }
    }
}

// Metrics
impl Composition {
    pub fn calculate_frames(&self) -> usize {
        (self.framerate as u16 * self.duration) as usize
    }

    pub fn calculate_bytes(&self) -> usize {
        let frames = self.calculate_frames();
        let per_frame_bytes = self.resolution.calculate_bytes(3);

        frames * per_frame_bytes
    }
}
