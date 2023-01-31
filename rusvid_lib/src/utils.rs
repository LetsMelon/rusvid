use resvg::usvg::Color;

fn hex_to_u8(value: &str) -> Option<u8> {
    u8::from_str_radix(value, 16).ok()
}

pub fn rgb_from_hex(hex_color: String) -> Option<Color> {
    let mut color = hex_color;
    if color.len() == 7 {
        color = color[1..color.len()].to_string();
    } else if color.len() != 6 {
        return None;
    }

    let r = hex_to_u8(&color[0..2])?;
    let g = hex_to_u8(&color[2..4])?;
    let b = hex_to_u8(&color[4..6])?;

    Some(Color::new_rgb(r, g, b))
}

/*
TODO use own struct to save color information, `usvg::Color` is only rgb and not rgba
pub fn rgba_from_hex(hex_color: String) -> Option<Color> {
    let mut color = hex_color;
    if color.len() == 9 {
        color = color[1..color.len()].to_string();
    } else if color.len() != 8 {
        return None;
    }

    let r = hex_to_u8(&color[0..2])?;
    let g = hex_to_u8(&color[2..4])?;
    let b = hex_to_u8(&color[4..6])?;
    let a = hex_to_u8(&color[6..8])?;

    Some(todo!())
}
*/

pub fn color_from_hex(hex_color: String) -> Option<Color> {
    match hex_color.len() {
        6 | 7 => rgb_from_hex(hex_color),
        8 | 9 => None, // see comment at `fn rgba_from_hex`
        _ => None,
    }
}

// TODO replace with crate approx
#[cfg(test)]
pub(crate) fn equal_delta(v1: f64, v2: f64, delta: f64) -> bool {
    let diff = (v1 - v2).abs();
    diff <= delta.abs()
}

#[cfg(test)]
mod tests {
    mod color {
        use resvg::usvg::Color;

        use crate::utils::color_from_hex;

        #[test]
        fn without_hashtag() {
            assert_eq!(
                color_from_hex("000000".to_string()).unwrap(),
                Color::new_rgb(0, 0, 0)
            );
            assert_eq!(
                color_from_hex("ffFFff".to_string()).unwrap(),
                Color::new_rgb(255, 255, 255)
            );

            assert_eq!(color_from_hex("000".to_string()), Option::None);
            assert_eq!(color_from_hex("000F0FFF".to_string()), Option::None);
        }

        #[test]
        fn with_hashtag() {
            assert_eq!(
                color_from_hex("#000000".to_string()).unwrap(),
                Color::new_rgb(0, 0, 0)
            );
            assert_eq!(
                color_from_hex("#ffFFff".to_string()).unwrap(),
                Color::new_rgb(255, 255, 255)
            );

            assert_eq!(color_from_hex("#000".to_string()), Option::None);
            assert_eq!(color_from_hex("#000F0FFF".to_string()), Option::None);
        }
    }
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
}
