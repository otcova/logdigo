use derive_more::*;

#[derive(Debug, Copy, Clone, PartialEq, Add, Sub)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Vec2 {
    pub const ZERO: Self = Vec2 { x: 0.0, y: 0.0 };
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

impl Into<[f32; 2]> for Vec2 {
    fn into(self) -> [f32; 2] {
        [self.x, self.y]
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
