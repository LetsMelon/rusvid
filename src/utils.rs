use usvg::Color;

pub fn color_from_hex(hex_color: String) -> Option<Color> {
    if !(hex_color.len() != 6 || hex_color.len() != 8) {
        return None;
    }
    let r = u8::from_str_radix(&hex_color[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex_color[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex_color[4..6], 16).ok()?;

    Some(Color::new_rgb(r, g, b))
}
