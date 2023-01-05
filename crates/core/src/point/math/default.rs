use crate::point::Point;

macro_rules! implement_math_operator {
    ($operant:ident, $fct:ident, Point for Point) => {
        paste::paste! {
            #[inline]
            pub fn [< $fct _point_for_point >] (p1: Point, p2: Point) -> Point {
                use std::ops::$operant;
                Point::from_raw(p1.raw().$fct(p2.raw()))
            }
        }

        impl std::ops::$operant<Point> for Point {
            type Output = Point;
            #[inline(always)]
            fn $fct(self, rhs: Self) -> Self::Output {
                paste::paste! {
                    [< $fct _point_for_point >](self, rhs)
                }
            }
        }
    };
    ($operant:ident, $fct:ident, f64 for Point) => {
        paste::paste! {
            #[inline]
            pub fn [< $fct _f64_for_point >] (p: Point, value: f64) -> Point {
                use std::ops::$operant;
                Point::from_raw(p.raw().$fct(value))
            }
        }

        impl std::ops::$operant<f64> for Point {
            type Output = Point;
            #[inline(always)]
            fn $fct(self, rhs: f64) -> Self::Output {
                paste::paste! {
                    [< $fct _f64_for_point >](self, rhs)
                }
            }
        }
    };
    ($operant:ident, $fct:ident, Point for f64) => {
        paste::paste! {
            #[inline]
            pub fn [< $fct _point_for_f64 >] (p: Point, value: f64) -> Point {
                use std::ops::$operant;
                Point::new(value.$fct(p.x()), value.$fct(p.y()))
            }
        }

        impl std::ops::$operant<Point> for f64 {
            type Output = Point;
            #[inline(always)]
            fn $fct(self, rhs: Self::Output) -> Self::Output {
                paste::paste! {
                    [< $fct _point_for_f64 >](rhs, self)
                }
            }
        }
    };
    ($operant:ident, $fct:ident, Point for assign $other_type:ident) => {
        concat_idents::concat_idents!(method_name = $fct, _assign {
            impl std::ops::$operant<$other_type> for Point {
                #[inline]
                fn method_name(&mut self, other: $other_type) {
                    use std::ops::*;

                    *self = self.$fct(other);
                }
            }
        });
    };
}

implement_math_operator!(Add, add, f64 for Point);
implement_math_operator!(Add, add, Point for f64);
implement_math_operator!(Add, add, Point for Point);
implement_math_operator!(AddAssign, add, Point for assign f64);
implement_math_operator!(AddAssign, add, Point for assign Point);

implement_math_operator!(Div, div, f64 for Point);
implement_math_operator!(Div, div, Point for f64);
implement_math_operator!(Div, div, Point for Point);
implement_math_operator!(DivAssign, div, Point for assign f64);
implement_math_operator!(DivAssign, div, Point for assign Point);

implement_math_operator!(Mul, mul, f64 for Point);
implement_math_operator!(Mul, mul, Point for f64);
implement_math_operator!(Mul, mul, Point for Point);
implement_math_operator!(MulAssign, mul, Point for assign f64);
implement_math_operator!(MulAssign, mul, Point for assign Point);

implement_math_operator!(Rem, rem, f64 for Point);
implement_math_operator!(Rem, rem, Point for f64);
implement_math_operator!(Rem, rem, Point for Point);
implement_math_operator!(RemAssign, rem, Point for assign f64);
implement_math_operator!(RemAssign, rem, Point for assign Point);

implement_math_operator!(Sub, sub, f64 for Point);
implement_math_operator!(Sub, sub, Point for f64);
implement_math_operator!(Sub, sub, Point for Point);
implement_math_operator!(SubAssign, sub, Point for assign f64);
implement_math_operator!(SubAssign, sub, Point for assign Point);
