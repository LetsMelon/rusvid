use anyhow::{bail, Result};
use image::{Rgba, RgbaImage};
use itertools::Itertools;

use crate::effect::{EffectLogic, Element, ID};

#[derive(Debug)]
pub struct BoxBlur {
    kernel_x: u32,
    kernel_y: u32,

    abs_d_x: i32,
    abs_d_y: i32,

    id: Option<ID>,
}

impl BoxBlur {
    #[inline(always)]
    pub fn new(kernel_size: u32) -> Result<Self> {
        Self::new_asymmetric(kernel_size, kernel_size)
    }

    #[inline(always)]
    pub fn new_with_id(kernel_size: u32, id: impl Into<ID>) -> Result<Self> {
        let mut obj = Self::new(kernel_size)?;
        obj.id = Some(id.into());

        Ok(obj)
    }

    #[inline(always)]
    pub fn new_asymmetric(kernel_x: u32, kernel_y: u32) -> Result<Self> {
        if kernel_x < 2 {
            bail!("kernel_x must be bigger 1 ({})", kernel_x);
        } else if kernel_x % 2 != 1 {
            bail!("kernel_x must be odd ({})", kernel_x);
        }
        if kernel_y < 2 {
            bail!("kernel_y must be bigger 1 ({})", kernel_y);
        } else if kernel_y % 2 != 1 {
            bail!("kernel_y must be odd ({})", kernel_y);
        }

        Ok(BoxBlur {
            kernel_x,
            kernel_y,
            abs_d_x: kernel_x.div_floor(2) as i32,
            abs_d_y: kernel_y.div_floor(2) as i32,
            id: None,
        })
    }

    #[inline(always)]
    pub fn new_asymmetric_with_id(kernel_x: u32, kernel_y: u32, id: impl Into<ID>) -> Result<Self> {
        let mut obj = Self::new_asymmetric(kernel_x, kernel_y)?;
        obj.id = Some(id.into());

        Ok(obj)
    }
}

impl Element for BoxBlur {
    fn id(&self) -> Option<&ID> {
        self.id.as_ref()
    }
}

impl EffectLogic for BoxBlur {
    fn apply(&self, original: image::RgbaImage) -> Result<RgbaImage> {
        let mut result = original.clone();

        for x in (self.abs_d_x as u32)..(result.width() - self.abs_d_x as u32) {
            for y in (self.abs_d_y as u32)..(result.height() - self.abs_d_y as u32) {
                let x = x as i32;
                let y = y as i32;

                let count = (self.kernel_x * self.kernel_y) as u32;
                let sum = ((self.abs_d_y * -1)..=self.abs_d_y)
                    .cartesian_product((self.abs_d_x * -1)..=self.abs_d_x)
                    .map(|(i_x, i_y)| *original.get_pixel((x + i_x) as u32, (y + i_y) as u32))
                    .fold([0_u32; 4], |mut acc, val| {
                        acc[0] += val[0] as u32;
                        acc[1] += val[1] as u32;
                        acc[2] += val[2] as u32;
                        acc[3] += val[3] as u32;

                        acc
                    });

                *result.get_pixel_mut(x as u32, y as u32) = Rgba([
                    (sum[0] / count) as u8,
                    (sum[1] / count) as u8,
                    (sum[2] / count) as u8,
                    (sum[3] / count) as u8,
                ]);
            }
        }

        Ok(result)
    }
}