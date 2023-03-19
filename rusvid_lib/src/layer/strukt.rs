use anyhow::Result;
use rusvid_core::holder::likes::TypesLike;
use rusvid_core::holder::object::Object;
use rusvid_core::holder::svg_holder::SvgHolder;
use rusvid_core::holder::transform::{Transform, TransformLogic};
use rusvid_core::holder::utils::random_id;
use rusvid_effect::EffectLogic;
use static_assertions::const_assert_eq;

use crate::animation::position_animation::PositionAnimation;
use crate::animation::{Animation, AnimationType};
use crate::resolution::Resolution;

pub enum LayerType {
    Svg,
    Image,
}

// These two enums must ALWAYS be in synch
const_assert_eq!(
    TypesLike::VARIANT_COUNT,
    std::mem::variant_count::<LayerType>()
);

#[derive(Debug)]
pub struct Layer {
    _name: String,

    // TODO why `pub`?
    pub object: Object,

    animations: Vec<AnimationType>,
    effects: Vec<Box<dyn EffectLogic>>,
}

impl Layer {
    pub fn new(layer_type: LayerType, _resolution: Resolution) -> Self {
        Self {
            _name: format!("layer_{}", random_id()),
            object: Object::new(match layer_type {
                LayerType::Svg => TypesLike::Svg(SvgHolder::new()),
                LayerType::Image => todo!(),
            }),

            animations: Vec::new(),
            effects: Vec::new(),
        }
    }

    pub fn update(&mut self, frame_count: usize) -> Result<()> {
        for animation in &self.animations {
            let id = animation.object_id();

            if animation.status_running(frame_count) {
                let transform = match animation {
                    AnimationType::Position(p_a) => Transform::Position(p_a.position(frame_count)),
                };

                self.object.transform_by_id(id, &transform)?;
            }
        }

        Ok(())
    }

    pub fn add_animation(&mut self, animation: AnimationType) {
        self.animations.push(animation)
    }

    pub fn add_position_animation(&mut self, animation: PositionAnimation) {
        self.add_animation(AnimationType::Position(animation))
    }

    pub fn add_effect<T: EffectLogic + 'static>(&mut self, effect: T) {
        self.effects.push(Box::new(effect))
    }
}
