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
