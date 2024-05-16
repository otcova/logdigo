use crate::Vec2;
use bytemuck::{Pod, Zeroable};
use std::ops::*;

#[repr(C, packed)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Zeroable, Pod, Debug)]
pub struct Rect<T> {
    pub min: Vec2<T>,
    pub max: Vec2<T>,
}

impl<T: Sub<Output = T> + Copy> Rect<T> {
    pub fn size(self) -> Vec2<T> {
        self.max - self.min
    }
}
