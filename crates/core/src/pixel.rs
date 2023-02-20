use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pixel([u8; 4]);

impl std::fmt::Debug for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pixel")
            .field("r", &self.0[0])
            .field("g", &self.0[1])
            .field("b", &self.0[2])
            .field("a", &self.0[3])
            .finish()
    }
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pixel")
            .field("r", &self.0[0])
            .field("g", &self.0[1])
            .field("b", &self.0[2])
            .field("a", &self.0[3])
            .finish()
    }
}

impl Pixel {
    pub const ZERO: Pixel = Pixel::new_raw([0; 4]);

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::new_raw([r, g, b, a])
    }

    pub const fn new_raw(values: [u8; 4]) -> Self {
        Pixel(values)
    }

    pub fn to_raw(&self) -> [u8; 4] {
        self.0
    }

    pub fn get_r(&self) -> u8 {
        self[0]
    }
    fn get_r_scripting(&mut self) -> u8 {
        self[0]
    }
    pub fn set_r(&mut self, value: u8) {
        self[0] = value;
    }

    pub fn get_g(&self) -> u8 {
        self[1]
    }
    fn get_g_scripting(&mut self) -> u8 {
        self[1]
    }
    pub fn set_g(&mut self, value: u8) {
        self[1] = value;
    }

    pub fn get_b(&self) -> u8 {
        self[2]
    }
    fn get_b_scripting(&mut self) -> u8 {
        self[2]
    }
    pub fn set_b(&mut self, value: u8) {
        self[2] = value;
    }

    pub fn get_a(&self) -> u8 {
        self[3]
    }
    fn get_a_scripting(&mut self) -> u8 {
        self[3]
    }
    pub fn set_a(&mut self, value: u8) {
        self[3] = value;
    }
}

impl Index<usize> for Pixel {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0..=3 => &self.0[index],
            // TODO implement better panic message
            _ => panic!("out of range"),
        }
    }
}

impl IndexMut<usize> for Pixel {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0..=3 => &mut self.0[index],
            // TODO implement better panic message
            _ => panic!("out of range"),
        }
    }
}

impl Index<&'_ str> for Pixel {
    type Output = u8;

    fn index(&self, index: &'_ str) -> &Self::Output {
        match index {
            "r" => &self[0],
            "g" => &self[1],
            "b" => &self[2],
            "a" => &self[3],
            // TODO implement better panic message
            _ => panic!("i have no idea!"),
        }
    }
}

impl IntoIterator for Pixel {
    type Item = u8;
    type IntoIter = std::array::IntoIter<Self::Item, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<[u8; 4]> for Pixel {
    fn from(value: [u8; 4]) -> Self {
        Self::new_raw(value)
    }
}

impl From<[u8; 3]> for Pixel {
    fn from(value: [u8; 3]) -> Self {
        Self::new(value[0], value[1], value[2], 255)
    }
}

#[cfg(feature = "scripting")]
impl rhai::CustomType for Pixel {
    fn build(mut builder: rhai::TypeBuilder<Self>) {
        use rhai::{Dynamic, EvalAltResult, Position, INT};

        let fct_pixel =
            |r: INT, g: INT, b: INT, a: INT| Self::new(r as u8, g as u8, b as u8, a as u8);

        builder
            .with_name("Pixel")
            .with_fn("pixel", fct_pixel.clone())
            .with_fn("pixel_raw", move |values: Vec<Dynamic>| {
                let r = values[0].clone_cast::<INT>().max(0).min(255);
                let g = values[1].clone_cast::<INT>().max(0).min(255);
                let b = values[2].clone_cast::<INT>().max(0).min(255);
                let a = values[3].clone_cast::<INT>().max(0).min(255);

                fct_pixel(r, g, b, a)
            })
            .with_fn("to_string", |p: &mut Pixel| -> String {
                format!("{p}").to_string()
            })
            .with_fn("to_debug", |p: &mut Pixel| -> String {
                format!("{p:?}").to_string()
            })
            .with_get_set("r", Self::get_r_scripting, Self::set_r)
            .with_get_set("g", Self::get_g_scripting, Self::set_g)
            .with_get_set("b", Self::get_b_scripting, Self::set_b)
            .with_get_set("a", Self::get_a_scripting, Self::set_a)
            .with_indexer_get(
                |pixel: &mut Pixel, idx: i64| -> core::result::Result<u8, Box<EvalAltResult>> {
                    match idx {
                        0..=3 => Ok(pixel[idx as usize]),
                        _ => Err(
                            EvalAltResult::ErrorIndexNotFound(idx.into(), Position::NONE).into(),
                        ),
                    }
                },
            );
    }
}
