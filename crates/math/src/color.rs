use crate::*;
use bytemuck::{Pod, Zeroable};
use derive_more::*;
use std::simd::*;

#[repr(C)]
#[derive(Debug, Into, From, Deref, Copy, Clone, PartialEq, Eq, Pod, Zeroable)]
pub struct Color(u8x4);

/// Used to convert #fff into #ffffff
/// This function takes [0xf, 0x3] and returns [0xff, 0x33]
fn repeat_hex(num: u8x4) -> u8x4 {
    (num << 4) | num
}

impl Color {
    /// Takes a hexadecimal color without the initial '#'
    pub fn from_hex(color: &str) -> Color {
        match color.len() {
            3 => repeat_hex(u8x4::from([
                u8::from_str_radix(&color[0..1], 16).unwrap(),
                u8::from_str_radix(&color[1..2], 16).unwrap(),
                u8::from_str_radix(&color[2..3], 16).unwrap(),
                0,
            ])),
            4 => repeat_hex(u8x4::from([
                u8::from_str_radix(&color[0..1], 16).unwrap(),
                u8::from_str_radix(&color[1..2], 16).unwrap(),
                u8::from_str_radix(&color[2..3], 16).unwrap(),
                u8::from_str_radix(&color[3..4], 16).unwrap(),
            ])),
            6 => u8x4::from([
                u8::from_str_radix(&color[0..2], 16).unwrap(),
                u8::from_str_radix(&color[2..4], 16).unwrap(),
                u8::from_str_radix(&color[4..6], 16).unwrap(),
                0,
            ]),
            8 => u8x4::from([
                u8::from_str_radix(&color[0..2], 16).unwrap(),
                u8::from_str_radix(&color[2..4], 16).unwrap(),
                u8::from_str_radix(&color[4..6], 16).unwrap(),
                u8::from_str_radix(&color[7..8], 16).unwrap(),
            ]),
            _ => panic!("Invalid hexadecimal color string {:?}", color),
        }
        .into()
    }
}

impl From<&str> for Color {
    fn from(color: &str) -> Color {
        if color.starts_with('#') {
            Color::from_hex(&color[1..])
        } else {
            match color {
                "white" => [255, 0, 0, 255],
                "gray" => [127, 127, 127, 255],
                "black" => [0, 0, 0, 255],
                "red" => [255, 0, 0, 255],
                "green" => [0, 255, 0, 255],
                "blue" => [0, 0, 255, 255],
                _ => panic!("Invalid color string {:?}", color),
            }
            .into()
        }
    }
}

impl From<[u8; 4]> for Color {
    fn from(value: [u8; 4]) -> Self {
        Color(value.into())
    }
}
