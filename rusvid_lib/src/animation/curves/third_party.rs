macro_rules! as_item {
    ($i:item) => {
        $i
    };
}

macro_rules! generate_ease_struct {
    ($struct_name:ident) => {
        #[doc = concat!("Wrapper struct around [easer::functions::", stringify!($struct_name), "]. See <https://easings.net/> for graphs.")]
        #[derive(std::fmt::Debug)]
        pub struct $struct_name {
            ease_type: crate::animation::curves::EaseType,
        }

        impl crate::animation::Function for $struct_name {
            fn new_with_ease_type(ease_type: crate::animation::curves::EaseType) -> Self {
                Self {
                    ease_type,
                }
            }

            fn get_ease_type(&self) -> &crate::animation::curves::EaseType {
                &self.ease_type
            }

            fn delta_ease_in(&self, delta: f32) -> f32 {
                use easer::functions::Easing;

                easer::functions::$struct_name::ease_in(delta, 0.0, 1.0, 1.0)
            }

            fn delta_ease_out(&self, delta: f32) -> f32 {
                use easer::functions::Easing;

                easer::functions::$struct_name::ease_out(delta, 0.0, 1.0, 1.0)
            }

            fn delta_ease_in_out(&self, delta: f32) -> f32 {
                use easer::functions::Easing;

                easer::functions::$struct_name::ease_in_out(delta, 0.0, 1.0, 1.0)
            }
        }
    };
    ($($x:ident),+ $(,)?) => (
        as_item! {
            #[derive(std::fmt::Debug)]
            pub enum FunctionType { $($x),* }
        }

        impl FunctionType {
            #[inline(always)]
            fn get_struct(&self) -> Box<dyn crate::animation::Function> {
                use crate::animation::Function;

                match self {
                    $(
                        Self::$x => Box::new($x::new()),
                    )*
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

            pub fn delta(&self, ease: crate::animation::curves::EaseType, delta: f32) -> f32 {
                match ease {
                    crate::animation::curves::EaseType::In => self.delta_ease_in(delta),
                    crate::animation::curves::EaseType::Out => self.delta_ease_out(delta),
                    crate::animation::curves::EaseType::InOut => self.delta_ease_in_out(delta),
                }
            }
        }

        $(
            generate_ease_struct!($x);
        )*
    );
}

generate_ease_struct!(Back, Bounce, Circ, Cubic, Elastic, Expo, Linear, Quad, Quart, Quint, Sine);
