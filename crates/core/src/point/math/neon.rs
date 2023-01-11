use std::simd::f64x2;

use crate::point::Point;

// impl std::ops::Add<Point> for Point {
//     type Output = Point;
//     fn add(self, rhs: Point) -> Self::Output {
//         println!("Printed from neon add");
//         Point::from_raw(self.raw().add(rhs.raw()))
//     }
// }

/*
use core::arch::aarch64::*;
fn simd_add_point_to_point(p1: Point, p2: Point) -> Point {
    let x1 = p1.x();
    let y1 = p1.y();
    let x2 = p2.x();
    let y2 = p2.y();

    let p1_packed = unsafe { vld1q_f64([x1, y1].as_ptr()) };
    let p2_packed = unsafe { vld1q_f64([x2, y2].as_ptr()) };

    let p_packed = unsafe { vaddq_f64(p1_packed, p2_packed) };

    let low = unsafe { vget_low_f64(p_packed) };
    let high = unsafe { vget_high_f64(p_packed) };

    println!("{:?}", (low, high));

    todo!()
}
 */

/*
use packed_simd::f64x2;
fn simd_add_point_to_point(p1: Point, p2: Point) -> Point {
    let p1_packed = f64x2::new(p1.x(), p1.y());
    let p2_packed = f64x2::new(p2.x(), p2.y());

    let p_packed = p1_packed + p2_packed;

    Point::new(p_packed.extract(0), p_packed.extract(1))
}
*/

fn simd_add_point_to_point(p1: Point, p2: Point) -> Point {
    let p1_packed = f64x2::from_array([p1.x(), p1.y()]);
    let p2_packed = f64x2::from_array([p2.x(), p2.y()]);

    let p_packed = p1_packed + p2_packed;
    let p_raw = p_packed.to_array();

    Point::new(p_raw[0], p_raw[1])
}

#[cfg(test)]
mod tests {
    use super::simd_add_point_to_point;
    use crate::point::Point;

    #[test]
    fn just_works() {
        assert_eq!(simd_add_point_to_point(Point::ZERO, Point::ONE), Point::ONE)
    }
}
