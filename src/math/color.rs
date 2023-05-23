pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }

    pub fn from_hex(hex: &str) -> Result<Self, std::num::ParseFloatError> {
        let hex = hex.trim_start_matches('#');

        let r = u8::from_str_radix(&hex[0..2], 16)? as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16)? as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16)? as f32 / 255.0;

        let a = if hex.len() >= 8 {
            u8::from_str_radix(&hex[6..8], 16)? as f32 / 255.0
        } else {
            1.0
        };

        Ok(Color::new(r, g, b, a))
    }
}
