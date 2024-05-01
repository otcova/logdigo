use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Pod, Zeroable)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const WHITE: Self = Color::gray(255);
    pub const GRAY: Self = Color::gray(127);
    pub const BLACK: Self = Color::gray(0);
    pub const RED: Self = Color { r: 255, g: 0, b: 0 };
    pub const GREEN: Self = Color { r: 0, g: 255, b: 0 };
    pub const BLUE: Self = Color { r: 0, g: 0, b: 255 };

    pub const fn gray(l: u8) -> Color {
        Color { r: l, g: l, b: l }
    }
}

/// Used to convert #fff into #ffffff
/// This function takes 0xf and returns 0xff
fn repeat_hex(num: u8) -> u8 {
    (num << 4) | num
}

impl Color {
    /// Takes a hexadecimal color without the initial '#'
    /// If present, ignores the alpha component
    pub fn from_hex(color: &str) -> Color {
        match color.len() {
            3 | 4 => Color {
                r: repeat_hex(u8::from_str_radix(&color[0..1], 16).unwrap()),
                g: repeat_hex(u8::from_str_radix(&color[0..1], 16).unwrap()),
                b: repeat_hex(u8::from_str_radix(&color[0..1], 16).unwrap()),
            },
            6 | 8 => Color {
                r: u8::from_str_radix(&color[0..2], 16).unwrap(),
                g: u8::from_str_radix(&color[2..4], 16).unwrap(),
                b: u8::from_str_radix(&color[4..6], 16).unwrap(),
            },
            _ => panic!("Invalid hexadecimal color string {:?}", color),
        }
    }
}

impl From<&str> for Color {
    fn from(color: &str) -> Color {
        if color.starts_with('#') {
            Color::from_hex(&color[1..])
        } else {
            match color {
                "white" => Color::WHITE,
                "gray" => Color::GRAY,
                "black" => Color::BLACK,
                "red" => Color::RED,
                "green" => Color::GREEN,
                "blue" => Color::BLUE,
                _ => panic!("Invalid color string {:?}", color),
            }
        }
    }
}

impl Into<[u8; 3]> for Color {
    fn into(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

impl Into<[u8; 4]> for Color {
    fn into(self) -> [u8; 4] {
        [self.r, self.g, self.b, 255]
    }
}
