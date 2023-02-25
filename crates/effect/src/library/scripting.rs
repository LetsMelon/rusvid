use anyhow::Result;
use itertools::*;
use rhai::{Dynamic, Engine, Func, OptimizationLevel, INT};
use rusvid_core::pixel::Pixel;
use rusvid_core::plane::Plane;

use crate::{EffectLogic, Element, ID};

pub struct ScriptingEffect {
    id: Option<String>,

    entry_point: String,
    script: &'static str,
}

impl std::fmt::Debug for ScriptingEffect {
    #[cfg_attr(coverage_nightly, no_coverage)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScriptingEffect")
            .field("id", &self.id)
            .field("entry_point", &self.entry_point)
            .field("script", &"TOO_LARGE")
            .finish()
    }
}

impl ScriptingEffect {
    pub fn new_with_id(
        entry_point: impl Into<String>,
        script: &'static str,
        id: impl Into<String>,
    ) -> Self {
        let mut effect = Self::new(entry_point, script);
        effect.id = Some(id.into());

        effect
    }

    pub fn new(entry_point: impl Into<String>, script: &'static str) -> Self {
        ScriptingEffect {
            id: None,
            entry_point: entry_point.into(),
            script,
        }
    }
}

impl Element for ScriptingEffect {
    fn id(&self) -> Option<&ID> {
        self.id.as_ref()
    }

    fn name(&self) -> &str {
        "scripting"
    }
}

impl EffectLogic for ScriptingEffect {
    fn apply(&self, original: Plane) -> Result<Plane> {
        let width = original.width();
        let height = original.height();

        let mut engine = Engine::new();
        engine.set_optimization_level(OptimizationLevel::Full);
        engine.build_type::<Pixel>();

        engine.register_fn("width", move || -> INT { width.clone() as INT });
        engine.register_fn("height", move || -> INT { height.clone() as INT });
        engine.register_fn("get_pixel", move |x: INT, y: INT| -> Dynamic {
            if x < 0 || y < 0 {
                return Dynamic::UNIT;
            }

            let p = original.pixel(x as u32, y as u32);

            match p {
                Some(value) => Dynamic::from_array(vec![
                    Dynamic::from(value[0] as INT),
                    Dynamic::from(value[1] as INT),
                    Dynamic::from(value[2] as INT),
                    Dynamic::from(value[3] as INT),
                ]),
                None => Dynamic::UNIT,
            }
        });

        let function =
            Func::<(INT, INT), Pixel>::create_from_script(engine, self.script, &self.entry_point)?;

        let data = (0..(width as INT))
            .cartesian_product(0..(height as INT))
            .map(|(x, y)| function(x, y).unwrap())
            .collect_vec();

        Ok(Plane::from_data_unchecked(width, height, data))
    }
}

#[cfg(test)]
mod tests {
    use rusvid_core::pixel::Pixel;
    use rusvid_core::plane::Plane;

    use super::ScriptingEffect;
    use crate::EffectLogic;

    #[test]
    fn simple_script() {
        const SCRIPT: &'static str = "
fn my_function(x, y) {
    pixel(255, 0, 0, 255)
}
";

        let size = 2;

        let plane =
            Plane::from_data(size, size, vec![Pixel::ZERO; (size * size) as usize]).unwrap();

        let effect = ScriptingEffect::new("my_function", SCRIPT);

        let effect_result = effect.apply(plane).unwrap();

        assert_eq!(
            *effect_result.pixel(0, 0).unwrap(),
            Pixel::new(255, 0, 0, 255)
        );
    }

    mod get_pixel {
        use rusvid_core::pixel::Pixel;
        use rusvid_core::plane::Plane;

        use crate::library::ScriptingEffect;
        use crate::EffectLogic;

        #[test]
        fn just_works() {
            const SCRIPT: &'static str = "
fn my_function(x, y) {
    let p = pixel_raw(get_pixel(x, y));

    if (p.r == 255) {
        p.r = 0
    }
    if (p.g == 255) {
        p.g = 0
    }
    if (p.b == 255) {
        p.b = 0
    }

    p
}
";

            let plane = Plane::from_data(
                2,
                2,
                vec![
                    Pixel::new(255, 100, 100, 255),
                    Pixel::new(10, 255, 10, 255),
                    Pixel::new(15, 15, 255, 255),
                    Pixel::new(40, 40, 40, 255),
                ],
            )
            .unwrap();

            let effect = ScriptingEffect::new("my_function", SCRIPT);

            let effect_result = effect.apply(plane).unwrap();

            assert_eq!(
                effect_result.pixel(0, 0).unwrap(),
                &Pixel::new(0, 100, 100, 255)
            );
            assert_eq!(
                effect_result.pixel(1, 0).unwrap(),
                &Pixel::new(15, 15, 0, 255)
            );
            assert_eq!(
                effect_result.pixel(0, 1).unwrap(),
                &Pixel::new(10, 0, 10, 255)
            );
            assert_eq!(
                effect_result.pixel(1, 1).unwrap(),
                &Pixel::new(40, 40, 40, 255)
            );
        }
    }
}
