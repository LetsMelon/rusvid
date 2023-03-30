macro_rules! as_item {
    ($i:item) => {
        $i
    };
}

macro_rules! generate_ease_struct {
    ($struct_name:ident) => {
        #[doc = concat!("Wrapper struct around [easer::functions::", stringify!($struct_name), "]. See <https://easings.net/> for graphs.")]
        #[derive(std::fmt::Debug, serde::Serialize, serde::Deserialize)]
        pub struct $struct_name;

        impl $struct_name {
            pub const fn new() -> Self {
                Self {}
            }
        }

        impl crate::animation::Function for $struct_name {
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
            #[derive(std::fmt::Debug, serde::Serialize, serde::Deserialize)]
            pub enum FunctionType { $($x),* }
        }

        impl FunctionType {
            paste::paste! {
                $(
                    #[allow(non_upper_case_globals)]
                    const [<CONST_  $x >]: $x = $x::new();
                )*
            }

            #[inline(always)]
            fn get_struct(&self) -> Box<&'static dyn crate::animation::Function> {
                match self {
                    $(
                        Self::$x => Box::new(
                            paste::paste! {
                                & Self::[<CONST_ $x>]
                            }
                        ),
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
