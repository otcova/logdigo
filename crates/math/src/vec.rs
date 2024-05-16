use bytemuck::{Pod, Zeroable};
use derive_more::*;

#[repr(C, packed)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Zeroable, Pod, Debug, Constructor, Add, Sub)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

#[repr(C, packed)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Zeroable, Pod, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Copy> From<[T; 2]> for Vec2<T> {
    fn from(v: [T; 2]) -> Vec2<T> {
        Vec2 { x: v[0], y: v[1] }
    }
}

impl<T> From<Vec2<T>> for [T; 2] {
    fn from(v: Vec2<T>) -> [T; 2] {
        [v.x, v.y]
    }
}

impl<T: Copy> Vec2<T> {
    pub fn splat(v: T) -> Self {
        Self { x: v, y: v }
    }
}

impl<T> From<Vec3<T>> for [T; 3] {
    fn from(v: Vec3<T>) -> [T; 3] {
        [v.x, v.y, v.z]
    }
}

impl<T: Copy> From<[T; 3]> for Vec3<T> {
    fn from(v: [T; 3]) -> Vec3<T> {
        Vec3 {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}

impl<T: Copy> Vec3<T> {
    pub fn splat(v: T) -> Self {
        Self { x: v, y: v, z: v }
    }
}
