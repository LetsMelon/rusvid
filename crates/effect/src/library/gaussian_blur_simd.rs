use core::arch::aarch64::{vld1q_f64, vmulq_n_f64, vst1q_f64};

use anyhow::Result;
use itertools::Itertools;
use log::info;
use rusvid_core::plane::Plane;

use crate::library::gaussian_blur::{gaussian_function, kernel_size};
use crate::{EffectLogic, Element, ID};

#[derive(Debug)]
pub struct GaussianBlurSimd {
    kernel: i32,

    abs_d: i32,
    weights: Vec<f64>,

    id: Option<ID>,
}

impl GaussianBlurSimd {
    #[inline(always)]
    pub fn new(stdev: f64) -> Self {
        let kernel = kernel_size(stdev);
        let abs_d = kernel.div_floor(2);

        info!(
            "GaussianBlur: stdev: {:.2}, kernel: {}, abs_d: {}",
            stdev, kernel, abs_d
        );

        let weights = ((-abs_d)..=abs_d)
            .cartesian_product((-abs_d)..=abs_d)
            .map(|(x, y)| gaussian_function(stdev.abs(), x, y))
            .collect::<Vec<f64>>();

        GaussianBlurSimd {
            kernel,
            abs_d,
            weights,
            id: None,
        }
    }

    #[inline(always)]
    pub fn new_with_id(stdev: f64, id: impl Into<ID>) -> Self {
        let mut obj = Self::new(stdev);
        obj.id = Some(id.into());

        obj
    }
}

impl Element for GaussianBlurSimd {
    fn id(&self) -> Option<&ID> {
        self.id.as_ref()
    }
}

impl EffectLogic for GaussianBlurSimd {
    fn apply(&self, original: Plane) -> Result<Plane> {
        let mut result = Plane::new(original.width(), original.height())?;

        for x in (self.kernel)..(result.width() as i32 - self.kernel) {
            for y in (self.kernel)..(result.height() as i32 - self.kernel) {
                let sum = ((-self.abs_d)..=self.abs_d)
                    .cartesian_product((-self.abs_d)..=self.abs_d)
                    .map(|(i_x, i_y)| {
                        let cord_x = (x + i_x) as u32;
                        let cord_y = (y + i_y) as u32;

                        let c = original.pixel_unchecked(cord_x, cord_y);

                        let weight = self.weights
                            [((i_y + self.abs_d) * self.kernel + (i_x + self.abs_d)) as usize];

                        let rg_packed = unsafe {
                            vmulq_n_f64(vld1q_f64([c[0] as f64, c[1] as f64].as_ptr()), weight)
                        };
                        let ba_packed = unsafe {
                            vmulq_n_f64(vld1q_f64([c[2] as f64, c[3] as f64].as_ptr()), weight)
                        };

                        let mut values = [0.0, 0.0, 0.0, 0.0];
                        unsafe {
                            vst1q_f64(values.as_mut_ptr().offset(0), rg_packed);
                            vst1q_f64(values.as_mut_ptr().offset(2), ba_packed);
                        };

                        values
                    })
                    .fold([0.0; 4], |mut acc, c| {
                        acc[0] += c[0];
                        acc[1] += c[1];
                        acc[2] += c[2];
                        acc[3] += c[3];

                        acc
                    });

                result.put_pixel_unchecked(
                    x as u32,
                    y as u32,
                    [sum[0] as u8, sum[1] as u8, sum[2] as u8, sum[3] as u8],
                );
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use std::hint::black_box;

    use rusvid_core::holder::gradient::base::BaseGradient;
    use rusvid_core::holder::gradient::linear::LinearGradient;
    use rusvid_core::holder::gradient::stop::Stop;
    use rusvid_core::holder::likes::color_like::ColorLike;
    use rusvid_core::holder::likes::path_like::PathLike;
    use rusvid_core::holder::likes::types_like::TypesLike;
    use rusvid_core::holder::object::Object;
    use rusvid_core::holder::svg_holder::{SvgHolder, SvgItem};
    use rusvid_core::plane::Plane;
    use rusvid_core::point::Point;

    use crate::library::{GaussianBlur, GaussianBlurSimd};
    use crate::EffectLogic;

    const SIZE: u32 = 300;

    fn setup() -> Plane {
        let mut svg = SvgHolder::new();

        let rect_size = Point::new_symmetric((SIZE as f64) * 0.3);
        let rect_pos = Point::new(100.0, 50.0);
        let triangle = SvgItem::new(
            vec![
                PathLike::Move(rect_pos),
                PathLike::Line(rect_size * Point::new(1.0, 0.0) + rect_pos),
                PathLike::Line(rect_size * Point::new(1.0, 1.0) + rect_pos),
                PathLike::Line(rect_size * Point::new(0.0, 1.0) + rect_pos),
                PathLike::Close,
            ],
            Some(ColorLike::LinearGradient(LinearGradient::new(
                BaseGradient::new(vec![
                    Stop::new([2, 0, 36, 255], 0.0),
                    Stop::new([9, 9, 121, 255], 0.35),
                    Stop::new([0, 212, 255, 255], 1.0),
                ]),
            ))),
        );
        svg.add_item(triangle);

        let heart = SvgItem::new(
            vec![
                PathLike::Move(Point::new(100.0, 100.0)),
                PathLike::Line(Point::new(150.0, 50.0)),
                PathLike::CurveTo(
                    Point::new(100.0, 25.0),
                    Point::new(169.0, 11.0),
                    Point::new(119.0, -13.0),
                ),
                PathLike::CurveTo(
                    Point::new(50.0, 50.0),
                    Point::new(80.0, -13.0),
                    Point::new(30.0, 11.0),
                ),
                PathLike::Close,
            ],
            Some(ColorLike::LinearGradient(LinearGradient::new(
                BaseGradient::new_from_colors(vec![[255, 0, 0, 255], [255, 100, 0, 255]]),
            ))),
        );
        svg.add_item(heart);

        let object = Object::new(TypesLike::Svg(svg));
        object.render(SIZE, SIZE).unwrap()
    }

    #[test]
    fn just_works() {
        let effect_simd = GaussianBlurSimd::new(3.0);
        let effect = GaussianBlur::new(3.0);

        for i in 0..10 {
            print!("{i}");
            let out = effect_simd.apply(black_box(setup()));
            assert!(out.is_ok());
            let out = effect.apply(black_box(setup()));
            assert!(out.is_ok());
        }
    }
}

// cargo flamegraph --unit-test crate_name -- test::in::package::with::multiple:crate
// cargo test --package rusvid_effect --lib -- library::gaussian_blur_simd::tests::just_works
// -o my_flamegraph.svg
