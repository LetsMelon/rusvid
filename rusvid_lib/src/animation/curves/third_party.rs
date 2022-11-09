macro_rules! generate_ease_struct {
    ($struct_name:ident) => {
        #[doc = concat!("Wrapper struct around [easer::functions::", stringify!($struct_name), "] that implements [crate::animation::curves::Function]. See <https://easings.net/> for graphs.")]
        #[derive(std::fmt::Debug)]
        pub struct $struct_name {
            start_frame: usize,
            end_frame: usize,
            start: crate::types::Point,
            end: crate::types::Point,

            ease_type: crate::animation::curves::EaseType,

            d_x: f64,
            d_y: f64,
            d_t: f64,
        }

        impl crate::animation::curves::Function for $struct_name {
            fn new(
                start_frame: usize,
                end_frame: usize,
                start: crate::types::Point,
                end: crate::types::Point,
            ) -> anyhow::Result<Self>
            where
                Self: Sized,
            {
                let delta = end - start;

                Ok(Self {
                    start_frame,
                    end_frame,
                    start,
                    end,
                    ease_type: crate::animation::curves::EaseType::default(),
                    d_x: delta.x,
                    d_y: delta.y,
                    d_t: (end_frame - start_frame) as f64,
                })
            }

            fn calc_ease_in(&self, frame_number: usize) -> crate::types::Point {
                use easer::functions::Easing;

                let frame_number = (frame_number - self.start_frame) as f64;

                let x = easer::functions::$struct_name::ease_in(
                    frame_number,
                    self.start.x,
                    self.d_x,
                    self.d_t,
                );
                let y = easer::functions::$struct_name::ease_in(
                    frame_number,
                    self.start.y,
                    self.d_y,
                    self.d_t,
                );
                crate::types::Point::new(x, y)
            }

            fn calc_ease_out(&self, frame_number: usize) -> crate::types::Point {
                use easer::functions::Easing;

                let frame_number = (frame_number - self.start_frame) as f64;

                let x = easer::functions::$struct_name::ease_out(
                    frame_number,
                    self.start.x,
                    self.d_x,
                    self.d_t,
                );
                let y = easer::functions::$struct_name::ease_out(
                    frame_number,
                    self.start.y,
                    self.d_y,
                    self.d_t,
                );
                crate::types::Point::new(x, y)
            }

            fn calc_ease_in_out(&self, frame_number: usize) -> crate::types::Point {
                use easer::functions::Easing;

                let frame_number = (frame_number - self.start_frame) as f64;

                let x = easer::functions::$struct_name::ease_in_out(
                    frame_number,
                    self.start.x,
                    self.d_x,
                    self.d_t,
                );
                let y = easer::functions::$struct_name::ease_in_out(
                    frame_number,
                    self.start.y,
                    self.d_y,
                    self.d_t,
                );
                crate::types::Point::new(x, y)
            }

            fn start_frame(&self) -> usize {
                self.start_frame
            }

            fn end_frame(&self) -> usize {
                self.end_frame
            }

            fn delta_frame(&self) -> usize {
                self.d_t as usize
            }

            fn start(&self) -> crate::types::Point {
                self.start
            }

            fn end(&self) -> crate::types::Point {
                self.end
            }

            fn set_ease_type(&mut self, ease_type: crate::animation::curves::EaseType) {
                self.ease_type = ease_type;
            }

            fn calc_raw(&self, frame_number: usize) -> crate::types::Point {
                match &self.ease_type {
                    crate::animation::curves::EaseType::In => self.calc_ease_in(frame_number),
                    crate::animation::curves::EaseType::Out => self.calc_ease_out(frame_number),
                    crate::animation::curves::EaseType::InOut => self.calc_ease_in_out(frame_number),
                }
            }

            fn delta_raw(&self, frame_number: usize) -> crate::types::Point {
                self.calc_raw(frame_number) - self.calc_raw(frame_number - 1)
            }
        }
    };
}

generate_ease_struct!(Back);
generate_ease_struct!(Bounce);
generate_ease_struct!(Circ);
generate_ease_struct!(Cubic);
generate_ease_struct!(Elastic);
generate_ease_struct!(Expo);
generate_ease_struct!(Linear);
generate_ease_struct!(Quad);
generate_ease_struct!(Quart);
generate_ease_struct!(Quint);
generate_ease_struct!(Sine);

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod start_is_start_value_and_end_is_end_value {
        use approx::assert_abs_diff_eq;

        use crate::animation::prelude::*;
        use crate::types::Point;

        const FRAME_START: usize = 10;
        const FRAME_END: usize = 30;
        const POS_START: Point = Point::ZERO;
        const POS_END: Point = Point::new(10.0, 25.0);
        const DELTA: f64 = 0.1;

        macro_rules! simple_test_ease_function {
            ($name:ident, $function:ident) => {
                let function = $name::new(FRAME_START, FRAME_END, POS_START, POS_END).unwrap();
                let function_name = stringify!($function);

                let fct: Box<dyn Fn(usize) -> Point> = match function_name {
                    "ease_in" => Box::new(|frame: usize| function.calc_ease_in(frame)),
                    "ease_out" => Box::new(|frame: usize| function.calc_ease_out(frame)),
                    "ease_in_out" => Box::new(|frame: usize| function.calc_ease_in_out(frame)),
                    _ => panic!("Undefined function: {}", function_name),
                };

                assert_abs_diff_eq!(fct(FRAME_START), POS_START, epsilon = DELTA);
                assert_abs_diff_eq!(fct(FRAME_END), POS_END, epsilon = DELTA);
            };
        }

        #[test]
        fn ease_in() {
            simple_test_ease_function!(Back, ease_in);
            simple_test_ease_function!(Bounce, ease_in);
            simple_test_ease_function!(Circ, ease_in);
            simple_test_ease_function!(Cubic, ease_in);
            simple_test_ease_function!(Elastic, ease_in);
            simple_test_ease_function!(Expo, ease_in);
            simple_test_ease_function!(Linear, ease_in);
            simple_test_ease_function!(Quad, ease_in);
            simple_test_ease_function!(Quart, ease_in);
            simple_test_ease_function!(Quint, ease_in);
            simple_test_ease_function!(Sine, ease_in);
        }

        #[test]
        fn ease_out() {
            simple_test_ease_function!(Back, ease_out);
            simple_test_ease_function!(Bounce, ease_out);
            simple_test_ease_function!(Circ, ease_out);
            simple_test_ease_function!(Cubic, ease_out);
            simple_test_ease_function!(Elastic, ease_out);
            simple_test_ease_function!(Expo, ease_out);
            simple_test_ease_function!(Linear, ease_out);
            simple_test_ease_function!(Quad, ease_out);
            simple_test_ease_function!(Quart, ease_out);
            simple_test_ease_function!(Quint, ease_out);
            simple_test_ease_function!(Sine, ease_out);
        }

        #[test]
        fn ease_in_out() {
            simple_test_ease_function!(Back, ease_in_out);
            simple_test_ease_function!(Bounce, ease_in_out);
            simple_test_ease_function!(Circ, ease_in_out);
            simple_test_ease_function!(Cubic, ease_in_out);
            simple_test_ease_function!(Elastic, ease_in_out);
            simple_test_ease_function!(Expo, ease_in_out);
            simple_test_ease_function!(Linear, ease_in_out);
            simple_test_ease_function!(Quad, ease_in_out);
            simple_test_ease_function!(Quart, ease_in_out);
            simple_test_ease_function!(Quint, ease_in_out);
            simple_test_ease_function!(Sine, ease_in_out);
        }
    }

    mod implements_debug {
        use crate::animation::prelude::*;

        macro_rules! test_debug {
            ($name:ident) => {
                let function =
                    $name::new(0, 100, crate::types::Point::ZERO, crate::types::Point::ONE)
                        .unwrap();
                let _ = format!("{:?}", function);
                assert!(true);
            };
        }

        #[test]
        fn just_works() {
            test_debug!(Back);
            test_debug!(Bounce);
            test_debug!(Circ);
            test_debug!(Cubic);
            test_debug!(Elastic);
            test_debug!(Expo);
            test_debug!(Linear);
            test_debug!(Quad);
            test_debug!(Quart);
            test_debug!(Quint);
            test_debug!(Sine);
        }
    }
}
