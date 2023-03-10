use std::f64::consts::{E, PI};

use itertools::Itertools;
use log::info;
use rusvid_core::pixel::Pixel;
use rusvid_core::plane::Plane;

use crate::error::EffectError;
use crate::{EffectLogic, Element, ID};

fn gaussian_function(stdev: f64, x: i32, y: i32) -> f64 {
    let pow_x = (x as f64).powf(2.0);
    let pow_y = (y as f64).powf(2.0);
    (1.0 / (2.0 * PI * stdev.powf(2.0))) * E.powf(-((pow_x + pow_y) / (2.0 * stdev.powf(2.0))))
}

fn kernel_size(stdev: f64) -> i32 {
    let mut kernel = (6.0 * stdev).ceil() as i32;

    if kernel % 2 == 0 {
        kernel += 1;
    }

    kernel
}

#[derive(Debug)]
/// Effect to apply a [gaussian blur](https://en.wikipedia.org/wiki/Gaussian_blur) effect on a [`Plane`].
pub struct GaussianBlur {
    stdev: f64,
    kernel: i32,

    abs_d: i32,
    weights: Vec<f64>,

    id: Option<ID>,
}

impl GaussianBlur {
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

        GaussianBlur {
            stdev,
            kernel,
            abs_d,
            weights,
            id: None,
        }
    }

    pub fn new_with_id(stdev: f64, id: impl Into<ID>) -> Self {
        let mut obj = Self::new(stdev);
        obj.id = Some(id.into());

        obj
    }

    pub fn kernel(&self) -> i32 {
        self.kernel
    }

    pub fn stdev(&self) -> f64 {
        self.stdev
    }
}

impl Element for GaussianBlur {
    fn id(&self) -> Option<&ID> {
        self.id.as_ref()
    }

    fn name(&self) -> &str {
        "gaussian blur"
    }
}

impl EffectLogic for GaussianBlur {
    fn apply(&self, original: Plane) -> Result<Plane, EffectError> {
        let mut result = Plane::new(original.width(), original.height())?;

        for x in (self.kernel)..(result.width() as i32 - self.kernel) {
            for y in (self.kernel)..(result.height() as i32 - self.kernel) {
                let sum = ((-self.abs_d)..=self.abs_d)
                    .cartesian_product((-self.abs_d)..=self.abs_d)
                    .map(|(i_x, i_y)| {
                        let cord_x = (x + i_x) as u32;
                        let cord_y = (y + i_y) as u32;
                        (*original.pixel_unchecked(cord_x, cord_y), i_x, i_y)
                    })
                    .fold([0.0; 4], |mut acc, val| {
                        // TODO check if it's the right index
                        let weight = self.weights
                            [((val.2 + self.abs_d) * self.kernel + (val.1 + self.abs_d)) as usize];
                        acc[0] += val.0[0] as f64 * weight;
                        acc[1] += val.0[1] as f64 * weight;
                        acc[2] += val.0[2] as f64 * weight;
                        acc[3] += val.0[3] as f64 * weight;

                        acc
                    });

                result.put_pixel_unchecked(
                    x as u32,
                    y as u32,
                    Pixel::new(sum[0] as u8, sum[1] as u8, sum[2] as u8, sum[3] as u8),
                );
            }
        }

        Ok(result)
    }
}
