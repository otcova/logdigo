use bytemuck::{Pod, Zeroable};
use derive_more::*;
use std::ops::*;

pub trait VecComponent: Copy + Clone + PartialEq + Add + Sub + Pod + Zeroable {}
impl<T: Copy + Clone + PartialEq + Add + Sub + Pod + Zeroable> VecComponent for T {}

#[repr(C, packed)]
#[derive(Debug, Copy, Clone, PartialEq, Add, Sub, Pod, Zeroable)]
pub struct Vec2<T: VecComponent> {
    pub x: T,
    pub y: T,
}

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

impl<T: VecComponent> Vec2<T> {
    pub fn splat(v: T) -> Vec2<T> {
        Vec2 { x: v, y: v }
    }
}

impl<T: VecComponent, U: From<T>> Into<[U; 2]> for Vec2<T> {
    fn into(self) -> [U; 2] {
        [self.x.into(), self.y.into()]
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
