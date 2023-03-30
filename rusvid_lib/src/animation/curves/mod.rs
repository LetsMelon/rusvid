use std::fmt::Debug;

use crate::animation::Function;

mod third_party;

pub use third_party::*;

#[derive(Debug, Clone, Copy)]
pub enum EaseType {
    In,
    Out,
    InOut,
}

impl Default for EaseType {
    fn default() -> Self {
        EaseType::In
    }
}

// TODO generate enum in `third_party.rs` + make `Function::new()` const
#[derive(Debug)]
pub enum FunctionType {
    Back,
    Bounce,
    Circ,
    Cubic,
    Elastic,
    Expo,
    Linear,
    Quad,
    Quart,
    Quint,
    Sine,
}

impl FunctionType {
    fn get_struct(&self) -> Box<dyn Function> {
        match self {
            FunctionType::Back => Box::new(Back::new()),
            FunctionType::Bounce => Box::new(Bounce::new()),
            FunctionType::Circ => Box::new(Circ::new()),
            FunctionType::Cubic => Box::new(Cubic::new()),
            FunctionType::Elastic => Box::new(Elastic::new()),
            FunctionType::Expo => Box::new(Expo::new()),
            FunctionType::Linear => Box::new(Linear::new()),
            FunctionType::Quad => Box::new(Quad::new()),
            FunctionType::Quart => Box::new(Quart::new()),
            FunctionType::Quint => Box::new(Quint::new()),
            FunctionType::Sine => Box::new(Sine::new()),
        }
    }

    pub fn delta_ease_in(&self, delta: f32) -> f32 {
        let function = self.get_struct();

        function.delta_ease_in(delta)
    }

    pub fn delta_ease_out(&self, delta: f32) -> f32 {
        let function = self.get_struct();

        function.delta_ease_out(delta)
    }

    pub fn delta_ease_in_out(&self, delta: f32) -> f32 {
        let function = self.get_struct();

        function.delta_ease_in_out(delta)
    }

    pub fn delta(&self, ease: EaseType, delta: f32) -> f32 {
        match ease {
            EaseType::In => self.delta_ease_in(delta),
            EaseType::Out => self.delta_ease_out(delta),
            EaseType::InOut => self.delta_ease_in_out(delta),
        }
    }
}
