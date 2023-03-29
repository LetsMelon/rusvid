use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pixel([u8; 4]);

impl std::fmt::Debug for Pixel {
    #[cfg_attr(coverage_nightly, no_coverage)]
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
    #[cfg_attr(coverage_nightly, no_coverage)]
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
    pub const FULL: Pixel = Pixel::new_raw([255; 4]);
    pub const WHITE: Pixel = Self::FULL;
    pub const BLACK: Pixel = Pixel::new_raw([0, 0, 0, 255]);

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::new_raw([r, g, b, a])
    }

    pub fn from_hex_string(hex_string: &str) -> Option<Self> {
        #[inline(always)]
        fn hex_to_u8(value: &str) -> Option<u8> {
            u8::from_str_radix(value, 16).ok()
        }

        #[inline(always)]
        fn from_rgb(value: &str) -> Option<Pixel> {
            let r = hex_to_u8(&value[0..2])?;
            let g = hex_to_u8(&value[2..4])?;
            let b = hex_to_u8(&value[4..6])?;

            Some(Pixel::new(r, g, b, 255))
        }

        #[inline(always)]
        fn from_rgba(value: &str) -> Option<Pixel> {
            let mut c = from_rgb(value)?;

            c.set_a(hex_to_u8(&value[6..8])?);

            Some(c)
        }

        match hex_string.len() {
            6 => from_rgb(hex_string),
            7 => from_rgb(&hex_string[1..hex_string.len()]),
            8 => from_rgba(hex_string),
            9 => from_rgba(&hex_string[1..hex_string.len()]),
            _ => None,
        }
    }

    pub const fn new_raw(values: [u8; 4]) -> Self {
        Pixel(values)
    }

    pub fn to_raw(&self) -> [u8; 4] {
        self.0
    }

    /// `0xAARRGGBB`
    pub fn to_raw_packed(&self) -> u32 {
        let mut result = 0;

        result |= (self.get_a() as u32) << 24;
        result |= (self.get_r() as u32) << 16;
        result |= (self.get_g() as u32) << 8;
        result |= self.get_b() as u32;

        result
    }

    pub fn get_r(&self) -> u8 {
        self[0]
    }
    pub fn get_r_mut(&mut self) -> &mut u8 {
        &mut self[0]
    }
    pub fn set_r(&mut self, value: u8) {
        self[0] = value;
    }

    pub fn get_g(&self) -> u8 {
        self[1]
    }
    pub fn get_g_mut(&mut self) -> &mut u8 {
        &mut self[1]
    }
    pub fn set_g(&mut self, value: u8) {
        self[1] = value;
    }

    pub fn get_b(&self) -> u8 {
        self[2]
    }
    pub fn get_b_mut(&mut self) -> &mut u8 {
        &mut self[2]
    }
    pub fn set_b(&mut self, value: u8) {
        self[2] = value;
    }

    pub fn get_a(&self) -> u8 {
        self[3]
    }
    pub fn get_a_mut(&mut self) -> &mut u8 {
        &mut self[3]
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
            .with_get_set(
                "r",
                |pixel: &mut Pixel| -> INT { pixel.get_r() as INT },
                |pixel: &mut Pixel, value: INT| pixel.set_r(value as u8),
            )
            .with_get_set(
                "g",
                |pixel: &mut Pixel| -> INT { pixel.get_g() as INT },
                |pixel: &mut Pixel, value: INT| pixel.set_g(value as u8),
            )
            .with_get_set(
                "b",
                |pixel: &mut Pixel| -> INT { pixel.get_b() as INT },
                |pixel: &mut Pixel, value: INT| pixel.set_b(value as u8),
            )
            .with_get_set(
                "a",
                |pixel: &mut Pixel| -> INT { pixel.get_a() as INT },
                |pixel: &mut Pixel, value: INT| pixel.set_a(value as u8),
            )
            .with_indexer_get(
                |pixel: &mut Pixel, idx: INT| -> core::result::Result<INT, Box<EvalAltResult>> {
                    match idx {
                        0..=3 => Ok(pixel[idx as usize] as INT),
                        _ => Err(
                            EvalAltResult::ErrorIndexNotFound(idx.into(), Position::NONE).into(),
                        ),
                    }
                },
            );
    }
}

#[cfg(test)]
mod tests {
    use super::Pixel;

    #[test]
    fn to_raw_and_packed() {
        let p = Pixel::new(255, 0, 0, 0);
        assert_eq!(p.to_raw_packed(), 0x00FF0000);
        assert_eq!(p.to_raw(), [255, 0, 0, 0]);

        let p = Pixel::new(0, 255, 0, 0);
        assert_eq!(p.to_raw_packed(), 0x0000FF00);
        assert_eq!(p.to_raw(), [0, 255, 0, 0]);

        let p = Pixel::new(0, 0, 255, 0);
        assert_eq!(p.to_raw_packed(), 0x000000FF);
        assert_eq!(p.to_raw(), [0, 0, 255, 0]);

        let p = Pixel::new(0, 0, 0, 255);
        assert_eq!(p.to_raw_packed(), 0xFF000000);
        assert_eq!(p.to_raw(), [0, 0, 0, 255]);
    }

    mod getter_setter {
        macro_rules! generate_test_case {
            ($name:ident) => {
                paste::paste! {
                    #[test]
                    fn $name() {
                        use crate::pixel::Pixel;

                        let mut p = Pixel::ZERO;

                        assert_eq!(Pixel::[<get_ $name >](&p), 0);

                        Pixel::[<set_ $name >](&mut p, 100);
                        assert_eq!(Pixel::[<get_ $name >](&p), 100);
                    }
                }
            };
        }

        generate_test_case!(r);
        generate_test_case!(g);
        generate_test_case!(b);
        generate_test_case!(a);
    }

    mod index {
        mod str {
            use crate::pixel::Pixel;

            #[test]
            fn rgba() {
                let p = Pixel::new(100, 200, 50, 75);

                assert_eq!(p["r"], 100);
                assert_eq!(p["g"], 200);
                assert_eq!(p["b"], 50);
                assert_eq!(p["a"], 75);
            }

            #[test]
            fn can_panic() {
                let result = std::panic::catch_unwind(|| {
                    let p = Pixel::ZERO;
                    p["c"];
                });

                assert!(result.is_err());
            }
        }

        mod usize {
            use crate::pixel::Pixel;

            #[test]
            fn by_index() {
                let p = Pixel::new(100, 200, 50, 75);

                assert_eq!(p[0], 100);
                assert_eq!(p[1], 200);
                assert_eq!(p[2], 50);
                assert_eq!(p[3], 75);
            }

            #[test]
            fn by_index_mut() {
                let mut p = Pixel::ZERO;

                assert_eq!(p[0], 0);
                assert_eq!(p[1], 0);
                assert_eq!(p[2], 0);
                assert_eq!(p[3], 0);

                p[0] = 100;
                p[1] = 200;
                p[2] = 50;
                p[3] = 75;

                assert_eq!(p[0], 100);
                assert_eq!(p[1], 200);
                assert_eq!(p[2], 50);
                assert_eq!(p[3], 75);
            }

            #[test]
            fn can_panic() {
                let result = std::panic::catch_unwind(|| {
                    let p = Pixel::ZERO;
                    let _ = p[4];
                });
                assert!(result.is_err());

                let result = std::panic::catch_unwind(|| {
                    let mut p = Pixel::ZERO;
                    p[4] = 100;
                });
                assert!(result.is_err());
            }
        }
    }

    mod iterator {
        use crate::pixel::Pixel;

        #[test]
        fn into_iter() {
            let p = Pixel::new(50, 25, 25, 100);

            assert_eq!(p.into_iter().map(|x| x as usize).sum::<usize>(), 200);
        }
    }

    mod from {
        use crate::pixel::Pixel;

        #[test]
        fn three_items() {
            assert_eq!(
                Into::<Pixel>::into([100, 200, 50]),
                Pixel::new(100, 200, 50, 255)
            );
        }

        #[test]
        fn four_items() {
            assert_eq!(
                Into::<Pixel>::into([100, 200, 50, 25]),
                Pixel::new(100, 200, 50, 25)
            );
        }
    }

    #[cfg(feature = "scripting")]
    mod scripting {
        use rhai::*;

        use crate::pixel::Pixel;

        fn dummy_engine() -> Engine {
            let mut engine = Engine::new();
            engine.build_type::<Pixel>();

            engine
        }

        #[test]
        fn pixel() {
            let engine = dummy_engine();

            let result = engine.eval::<Pixel>("pixel(255, 0, 0, 255)").unwrap();
            assert_eq!(result, Pixel::new(255, 0, 0, 255));
        }

        #[test]
        fn pixel_raw() {
            let engine = dummy_engine();

            let result = engine.eval::<Pixel>("pixel_raw([0, 255, 0, 255])").unwrap();
            assert_eq!(result, Pixel::new(0, 255, 0, 255));

            let result = engine
                .eval::<Pixel>("pixel_raw([-100, 255, 300, 255])")
                .unwrap();
            assert_eq!(result, Pixel::new(0, 255, 255, 255));
        }

        #[test]
        fn to_string() {
            let engine = dummy_engine();

            let result = engine
                .eval::<String>("let p = pixel_raw([0, 255, 0, 255]);\np.to_string()")
                .unwrap();
            assert_eq!(result, "Pixel { r: 0, g: 255, b: 0, a: 255 }");
        }

        #[test]
        fn to_debug() {
            let engine = dummy_engine();

            let result = engine
                .eval::<String>("let p = pixel_raw([0, 255, 255, 255]);\np.to_debug()")
                .unwrap();
            assert_eq!(result, "Pixel { r: 0, g: 255, b: 255, a: 255 }");
        }

        mod getter_setter {
            use rhai::INT;

            use super::dummy_engine;

            fn generate_getter_script(value: &str) -> String {
                format!(
                    "
                let p = pixel(255, 205, 105, 55);
                p.{value}
                "
                )
            }

            fn generate_setter_script(rgba: &str, value: u8) -> String {
                format!(
                    "
                let p = pixel(0, 0, 0, 0);
                p.{rgba} = {value};
                p.{rgba}
                "
                )
            }

            #[test]
            fn get_rgba() {
                let engine = dummy_engine();

                let result = engine.eval::<INT>(&generate_getter_script("r")).unwrap();
                assert_eq!(result, 255);
                let result = engine.eval::<INT>(&generate_getter_script("g")).unwrap();
                assert_eq!(result, 205);
                let result = engine.eval::<INT>(&generate_getter_script("b")).unwrap();
                assert_eq!(result, 105);
                let result = engine.eval::<INT>(&generate_getter_script("a")).unwrap();
                assert_eq!(result, 55);
            }

            #[test]
            fn set_rgba() {
                let engine = dummy_engine();

                let result = engine
                    .eval::<INT>(&generate_setter_script("r", 55))
                    .unwrap();
                assert_eq!(result, 55);
                let result = engine
                    .eval::<INT>(&generate_setter_script("g", 105))
                    .unwrap();
                assert_eq!(result, 105);
                let result = engine
                    .eval::<INT>(&generate_setter_script("b", 155))
                    .unwrap();
                assert_eq!(result, 155);
                let result = engine
                    .eval::<INT>(&generate_setter_script("a", 255))
                    .unwrap();
                assert_eq!(result, 255);
            }
        }

        mod index {
            use rhai::INT;

            use super::dummy_engine;

            fn generate_index_getter_script(index: usize) -> String {
                format!(
                    "
                let p = pixel(255, 205, 105, 55);
                p[{index}]
                "
                )
            }

            #[test]
            fn get() {
                let engine = dummy_engine();

                let result = engine
                    .eval::<INT>(&&generate_index_getter_script(0))
                    .unwrap();
                assert_eq!(result, 255);
                let result = engine
                    .eval::<INT>(&generate_index_getter_script(1))
                    .unwrap();
                assert_eq!(result, 205);
                let result = engine
                    .eval::<INT>(&generate_index_getter_script(2))
                    .unwrap();
                assert_eq!(result, 105);
                let result = engine
                    .eval::<INT>(&generate_index_getter_script(3))
                    .unwrap();
                assert_eq!(result, 55);

                let result = engine.eval::<INT>(&generate_index_getter_script(4));
                assert!(result.is_err());
            }
        }
    }
}
