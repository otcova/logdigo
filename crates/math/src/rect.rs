use crate::Vec2;
use bytemuck::{Pod, Zeroable};
use num_traits::Num;

#[repr(C, packed)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Zeroable, Pod, Debug)]
pub struct Rect<T>
where
    T: Num,
{
    pub min: Vec2<T>,
    pub max: Vec2<T>,
}

impl<T: Num> Rect<T> {
    pub fn size(self) -> Vec2<T> {
        self.max - self.min
    }
}

impl<T: Num + Copy> Rect<T> {
    pub fn from_size(pos: Vec2<T>, size: Vec2<T>) -> Self {
        Rect {
            min: pos,
            max: pos + size,
        }
    }
}
