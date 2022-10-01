use usvg::{Color, PathSegment};

use crate::animation::curves::Point;

pub fn color_from_hex(hex_color: String) -> Option<Color> {
    if !(hex_color.len() != 6 || hex_color.len() != 8) {
        return None;
    }
    let r = u8::from_str_radix(&hex_color[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex_color[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex_color[4..6], 16).ok()?;

    Some(Color::new_rgb(r, g, b))
}

pub fn equal_delta(v1: f64, v2: f64, delta: f64) -> bool {
    let diff = (v1 - v2).abs();
    diff <= delta.abs()
}

pub fn map(value: f64, low1: f64, high1: f64, low2: f64, high2: f64) -> f64 {
    low2 + (value - low1) * (high2 - low2) / (high1 - low1)
}

pub fn set_path(segments: &mut [PathSegment], cords: Point) {
    for seg in segments {
        match seg {
            PathSegment::MoveTo { x, y } => {
                apply_to(x, y, &cords);
            }
            PathSegment::LineTo { x, y } => {
                apply_to(x, y, &cords);
            }
            PathSegment::CurveTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => {
                apply_to(x1, y1, &cords);
                apply_to(x2, y2, &cords);
                apply_to(x, y, &cords);
            }
            PathSegment::ClosePath => {}
        }
    }
}

#[inline]
fn apply_to(x: &mut f64, y: &mut f64, cords: &Point) {
    *x = cords.x;
    *y = cords.y;
}

#[cfg(test)]
mod tests {
    mod equal_delta {
        use crate::utils::equal_delta;

        #[test]
        fn zero_delta() {
            assert!(equal_delta(10.0, 10.0, 0.0));
            assert_eq!(equal_delta(10.0, 10.1, 0.0), false);
        }

        #[test]
        fn small_delta() {
            assert!(equal_delta(0.00000001, 0.0, 0.0001));
            assert!(equal_delta(-0.00000001, 0.0, 0.0001));
            assert_eq!(equal_delta(2.0, 0.0, 0.0001), false);
        }

        #[test]
        fn big_delta() {
            assert!(equal_delta(10.0, 15.0, 6.5));
            assert!(equal_delta(10.0, 4.78, 6.5));
            assert_eq!(equal_delta(10.0, std::f64::consts::PI, 6.5), false);
        }
    }

    mod map {
        use crate::utils::map;

        #[test]
        fn just_works() {
            assert_eq!(map(10.0, 0.0, 100.0, 0.0, 1.0), 0.1);
            assert_eq!(map(10.0, 0.0, 10.0, 0.0, 1.0), 1.0);
        }
    }
}
