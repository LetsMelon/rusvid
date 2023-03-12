use rusvid_core::pixel::Pixel;

const MULTIPLIER_RED: f32 = 0.299;
const MULTIPLIER_GREEN: f32 = 0.587;
const MULTIPLIER_BLUE: f32 = 0.114;

#[inline(always)]
pub fn transform(source: &Pixel) -> Pixel {
    let grayscale_value = (source.get_r() as f32 * MULTIPLIER_RED
        + source.get_g() as f32 * MULTIPLIER_GREEN
        + source.get_b() as f32 * MULTIPLIER_BLUE) as u8;

    Pixel::new(grayscale_value, grayscale_value, grayscale_value, source[3])
}

#[cfg(test)]
mod tests {
    use rusvid_core::pixel::Pixel;

    use super::transform;

    #[test]
    fn just_works() {
        let source = Pixel::new(255, 0, 0, 255);
        let result = transform(&source);
        assert!(!(source.get_r() == source.get_g() && source.get_r() == source.get_b()));
        assert!((result.get_r() == result.get_g() && result.get_r() == result.get_b()));
        assert_eq!(source.get_a(), result.get_a());
        assert_eq!(result.get_r(), 76);

        let source = Pixel::new(0, 255, 0, 255);
        let result = transform(&source);
        assert!(!(source.get_r() == source.get_g() && source.get_r() == source.get_b()));
        assert!((result.get_r() == result.get_g() && result.get_r() == result.get_b()));
        assert_eq!(source.get_a(), result.get_a());
        assert_eq!(result.get_r(), 149);

        let source = Pixel::new(0, 0, 255, 255);
        let result = transform(&source);
        assert!(!(source.get_r() == source.get_g() && source.get_r() == source.get_b()));
        assert!((result.get_r() == result.get_g() && result.get_r() == result.get_b()));
        assert_eq!(source.get_a(), result.get_a());
        assert_eq!(result.get_r(), 29);
    }
}
