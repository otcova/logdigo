use bytemuck::{Pod, Zeroable};
use digolog_math::*;
use guillotiere::{Allocation, AtlasAllocator, Size};
use wgpu::Extent3d;

pub struct Atlas3d {
    /// All the layers of the atlas. Bigger and less layers are priorized.
    atlas: Vec<AtlasAllocator>,
    /// Max width(x), height(y) and depth(z) of the textures
    max_dimension_size: u16,
}

pub struct AtlasHandle {
    allocation: Allocation,
    layer: u16,
}

pub struct AllocId {
    id: guillotiere::AllocId,
    layer: u16,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct TextureRect {
    pub rect: Rect<u16>,
    pub layer: u16,
}

impl Atlas3d {
    const MIN_SIZE: u16 = 256;
    const MAX_SIZE: u16 = 1 << 15; // To prevent overflow when using u16

    /// max_dimension_size will be clamped with MIN_SIZE and MAX_SIZE
    pub fn new(max_dimension_size: u16) -> Self {
        let initial_size = Size::splat(Self::MIN_SIZE as i32);
        Self {
            atlas: vec![AtlasAllocator::new(initial_size)],
            max_dimension_size: max_dimension_size.clamp(Self::MIN_SIZE, Self::MAX_SIZE),
        }
    }

    pub fn add(&mut self, size: Vec2<u16>) -> Option<AtlasHandle> {
        let isize = Size::new(size.x as i32, size.y as i32);

        for layer in 0..self.atlas.len() as u16 {
            if let Some(allocation) = self.atlas[layer as usize].allocate(isize) {
                return Some(AtlasHandle { allocation, layer });
            }
        }

        None
    }

    pub fn size(&self) -> Vec3<u16> {
        let size = self.atlas[0].size();
        Vec3 {
            x: size.width as u16,
            y: size.height as u16,
            z: self.atlas.len() as u16,
        }
    }

    pub fn remove(&mut self, id: AllocId) {
        self.atlas[id.layer as usize].deallocate(id.id);
    }

    /// Fast function that resizes the atlas.
    pub fn grow(&mut self, min_size_increment: Vec2<u16>) {
        let new_size = self.next_size(min_size_increment);
        let atlas_size = Size::new(new_size.x as i32, new_size.y as i32);

        for atlas in &mut self.atlas {
            atlas.grow(atlas_size);
        }

        // Is expected for this loop to only do 0 or 1 iterations
        while self.atlas.len() < new_size.z as usize {
            self.atlas.push(AtlasAllocator::new(atlas_size));
        }
    }

    /// Returns the next size after grow
    fn next_size(&self, min_size_increment: Vec2<u16>) -> Vec3<u16> {
        let current_size = self.size();

        let mut new_size = current_size.x;
        let target_size = new_size + min_size_increment.x.max(min_size_increment.y);

        if target_size > self.max_dimension_size {
            let size = Vec3 {
                x: self.max_dimension_size,
                y: self.max_dimension_size,
                z: current_size.z + 1,
            };
        }

        // Is expected for this loop do 1 iteration
        while new_size < target_size {
            new_size *= 2;
        }

        Vec3 {
            x: new_size,
            y: new_size,
            z: 1,
        }
    }
}

impl AtlasHandle {
    pub fn rect(&self) -> TextureRect {
        let min = self.allocation.rectangle.min;
        let max = self.allocation.rectangle.max;
        TextureRect {
            rect: Rect {
                min: Vec2::new(min.x as u16, min.y as u16),
                max: Vec2::new(max.x as u16, max.y as u16),
            },
            layer: self.layer,
        }
    }

    pub fn id(&self) -> AllocId {
        AllocId {
            id: self.allocation.id,
            layer: self.layer,
        }
    }
}
