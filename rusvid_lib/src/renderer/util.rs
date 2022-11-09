use anyhow::Result;
use rusvid_core::plane::{Pixel, Plane};

use crate::effect::EffectLogic;

#[inline]
pub fn combine_renders(width: u32, height: u32, images: Vec<Plane>) -> Result<Plane> {
    let images_as_data = images
        .iter()
        .map(|i| i.as_data())
        .collect::<Vec<&Vec<Pixel>>>();

    let data = (0..((width * height) as usize))
        .map(|i| {
            images_as_data.iter().fold([0_u8; 4], |acc, value| {
                let value = value[i];

                match (acc[3], value[3]) {
                    (0, 0) => acc, // both colors are fully transparent -> do nothing
                    (_, 0) => acc, // new color is fully transparent -> do nothing
                    // old color is transparent and the new color overrides it completely
                    (0, _) => value,
                    // mix both colors into a new one
                    (255, 255) => {
                        // TODO add flag if the layer should override the old one or "merge", if merge then use calculation from beneath match closure
                        value
                    }
                    // mix both colors into a new one
                    (_, _) => {
                        let bg_r = (acc[0] as f64) / 255.0;
                        let bg_g = (acc[1] as f64) / 255.0;
                        let bg_b = (acc[2] as f64) / 255.0;
                        let bg_a = (acc[3] as f64) / 255.0;

                        let fg_r = (value[0] as f64) / 255.0;
                        let fg_g = (value[1] as f64) / 255.0;
                        let fg_b = (value[2] as f64) / 255.0;
                        let fg_a = (value[3] as f64) / 255.0;

                        let mix_a = 1.0 - (1.0 - fg_a) * (1.0 - bg_a);
                        let mix_r = fg_r * fg_a / mix_a + bg_r * bg_a * (1.0 - fg_a) / mix_a;
                        let mix_g = fg_g * fg_a / mix_a + bg_g * bg_a * (1.0 - fg_a) / mix_a;
                        let mix_b = fg_b * fg_a / mix_a + bg_b * bg_a * (1.0 - fg_a) / mix_a;

                        [
                            (mix_r * 255.0) as u8,
                            (mix_g * 255.0) as u8,
                            (mix_b * 255.0) as u8,
                            (mix_a * 255.0) as u8,
                        ]
                    }
                }
            })
        })
        .collect();

    Ok(Plane::from_data_unchecked(width, height, data))
}

#[inline]
pub fn apply_effects(original: Plane, effects: &Vec<Box<dyn EffectLogic>>) -> Result<Plane> {
    let mut back = original;

    for effect in effects {
        back = effect.apply(back)?;
    }

    Ok(back)
}
