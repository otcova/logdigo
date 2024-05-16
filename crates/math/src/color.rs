use bytemuck::{Pod, Zeroable};
use derive_more::*;

#[repr(C)]
#[derive(Debug, Into, Copy, Clone, Hash, PartialEq, Eq, Pod, Zeroable)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    /// Used to convert #fff into #ffffff.
    /// This function takes [0xf, 0x3, 0x1, 0xf] and returns [0xff, 0x33, 0x11, 0xff].
    ///
    /// # Panic
    /// If the color is not of the format [0x0_, 0x0_, 0x0_, 0x0_],
    /// the function could panic.
    fn repeat_hex(self) -> Color {
        let raw_color: u32 = self.into();
        ((raw_color << 4) | raw_color).into()
    }

    /// Takes a hexadecimal color without the initial '#'
    pub fn from_hex(color: &str) -> Color {
        match color.len() {
            3 => Color {
                r: u8::from_str_radix(&color[0..1], 16).unwrap(),
                g: u8::from_str_radix(&color[1..2], 16).unwrap(),
                b: u8::from_str_radix(&color[2..3], 16).unwrap(),
                a: 0,
            }
            .repeat_hex(),
            4 => Color {
                r: u8::from_str_radix(&color[0..1], 16).unwrap(),
                g: u8::from_str_radix(&color[1..2], 16).unwrap(),
                b: u8::from_str_radix(&color[2..3], 16).unwrap(),
                a: u8::from_str_radix(&color[3..4], 16).unwrap(),
            }
            .repeat_hex(),
            6 => Color {
                r: u8::from_str_radix(&color[0..2], 16).unwrap(),
                g: u8::from_str_radix(&color[2..4], 16).unwrap(),
                b: u8::from_str_radix(&color[4..6], 16).unwrap(),
                a: 0,
            },
            8 => Color {
                r: u8::from_str_radix(&color[0..2], 16).unwrap(),
                g: u8::from_str_radix(&color[2..4], 16).unwrap(),
                b: u8::from_str_radix(&color[4..6], 16).unwrap(),
                a: u8::from_str_radix(&color[7..8], 16).unwrap(),
            },
            _ => panic!("Invalid hexadecimal color string {:?}", color),
        }
    }
}

impl From<&str> for Color {
    fn from(color: &str) -> Color {
        if let Some(hex_code) = color.strip_prefix('#') {
            Color::from_hex(hex_code)
        } else {
            // match color {
            //     "white" => [255, 0, 0, 255],
            //     "gray" => [127, 127, 127, 255],
            //     "black" => [0, 0, 0, 255],
            //     "red" => [255, 0, 0, 255],
            //     "green" => [0, 255, 0, 255],
            //     "blue" => [0, 0, 255, 255],
            //     _ => panic!("Invalid color string {:?}", color),
            // }
            // .into()
            panic!("Invalid color string {:?}", color);
        }
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Color {
            r: value as u8,
            g: (value >> 8) as u8,
            b: (value >> 16) as u8,
            a: (value >> 24) as u8,
        }
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> u32 {
        color.r as u32 | (color.g as u32) << 8 | (color.b as u32) << 16 | (color.a as u32) << 24
    }
}
