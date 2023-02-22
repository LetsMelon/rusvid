use rusvid_core::pixel::Pixel;

#[inline(always)]
fn calculate_color_diff(c1: &Pixel, c2: &Pixel) -> u32 {
    c1[0].abs_diff(c2[0]) as u32
        + c1[1].abs_diff(c2[1]) as u32
        + c1[2].abs_diff(c2[2]) as u32
        + c1[3].abs_diff(c2[3]) as u32
}

#[inline(always)]
pub fn transform(source: &Pixel, color_palette: &[Pixel]) -> Pixel {
    let mut best_palette_color = color_palette[0];
    let mut distance = calculate_color_diff(source, &best_palette_color);
    for i in 1..color_palette.len() {
        let color_to_test = color_palette[i];
        let test_distance = calculate_color_diff(source, &color_to_test);

        if test_distance < distance {
            best_palette_color = color_to_test;
            distance = test_distance;
        }
    }

    best_palette_color
}

#[cfg(test)]
mod tests {
    use rusvid_core::pixel::Pixel;

    use super::{calculate_color_diff, transform};

    #[test]
    fn calculate_color_diff_test() {
        let p1 = Pixel::new(255, 100, 55, 10);
        let p2 = p1.clone();
        assert_eq!(calculate_color_diff(&p1, &p2), 0);

        let p1 = Pixel::new(255, 0, 0, 0);
        let p2 = Pixel::new(0, 0, 0, 0);
        assert_eq!(calculate_color_diff(&p1, &p2), 255);

        let p1 = Pixel::new(0, 0, 0, 0);
        let p2 = Pixel::new(0, 255, 0, 0);
        assert_eq!(calculate_color_diff(&p1, &p2), 255);

        let p1 = Pixel::new(0, 0, 255, 0);
        let p2 = Pixel::new(0, 0, 0, 0);
        assert_eq!(calculate_color_diff(&p1, &p2), 255);

        let p1 = Pixel::new(255, 0, 0, 255);
        let p2 = Pixel::new(255, 0, 0, 0);
        assert_eq!(calculate_color_diff(&p1, &p2), 255);

        let p1 = Pixel::new(255, 100, 55, 10);
        let p2 = Pixel::new(0, 0, 0, 0);
        assert_eq!(calculate_color_diff(&p1, &p2), 420);
    }

    #[test]
    fn transform_test() {
        let palette = vec![Pixel::BLACK, Pixel::WHITE];
        let p = Pixel::new(255, 0, 0, 255);
        let result = transform(&p, &palette);
        assert_eq!(result, Pixel::BLACK);

        let palette = vec![Pixel::BLACK, Pixel::WHITE];
        let p = Pixel::new(255, 255, 0, 255);
        let result = transform(&p, &palette);
        assert_eq!(result, Pixel::WHITE);
    }
}
