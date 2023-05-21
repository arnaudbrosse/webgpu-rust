pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Color {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Color {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        if hex.len() != 7 || !hex.starts_with('#') {
            return None;
        }

        let red = u8::from_str_radix(&hex[1..3], 16).ok()?;
        let green = u8::from_str_radix(&hex[3..5], 16).ok()?;
        let blue = u8::from_str_radix(&hex[5..7], 16).ok()?;

        Some(Color::new(red, green, blue, 255))
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}
