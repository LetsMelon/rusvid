use std::ops::{Index, IndexMut};

pub type BITDEPTH = u8;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pixel([BITDEPTH; 4]);

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

impl Pixel {
    pub const ZERO: Pixel = Pixel::new_raw([0; 4]);

    pub fn new(r: BITDEPTH, g: BITDEPTH, b: BITDEPTH, a: BITDEPTH) -> Self {
        Self::new_raw([r, g, b, a])
    }

    pub const fn new_raw(values: [BITDEPTH; 4]) -> Self {
        Pixel(values)
    }

    pub fn to_raw(&self) -> [BITDEPTH; 4] {
        self.0
    }
}

impl Index<usize> for Pixel {
    type Output = BITDEPTH;

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
    type Output = BITDEPTH;

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
    type Item = BITDEPTH;
    type IntoIter = std::array::IntoIter<Self::Item, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<[BITDEPTH; 4]> for Pixel {
    fn from(value: [BITDEPTH; 4]) -> Self {
        Self::new_raw(value)
    }
}

impl From<[BITDEPTH; 3]> for Pixel {
    fn from(value: [BITDEPTH; 3]) -> Self {
        Self::new(value[0], value[1], value[2], 255)
    }
}
