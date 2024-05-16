use digolog_math::*;
use guillotiere::{Allocation, AtlasAllocator, Size};
use wgpu::Extent3d;

pub struct Atlas3d {
    /// All the layers of the atlas. Bigger and less layers are priorized.
    atlas: Vec<AtlasAllocator>,
    /// Max width(x), height(y) and depth(z) of the textures
    max_dimension_size: u32,
}

pub struct AtlasHandle {
    allocation: Allocation,
    layer: u32,
}

impl Atlas3d {
    const MIN_SIZE: u32 = 256;
    const MAX_SIZE: u32 = 1 << 24;

    /// max_dimension_size will be clamped with MIN_SIZE and MAX_SIZE
    pub fn new(max_dimension_size: u32) -> Self {
        let initial_size = Size::splat(Self::MIN_SIZE as i32);
        Self {
            atlas: vec![AtlasAllocator::new(initial_size)],
            max_dimension_size: max_dimension_size.clamp(Self::MIN_SIZE, Self::MAX_SIZE),
        }
    }

    pub fn add(&mut self, size: Vec2<u32>) -> Option<AtlasHandle> {
        let isize = Size::new(size.x as i32, size.y as i32);

        for layer in 0..self.atlas.len() as u32 {
            if let Some(allocation) = self.atlas[layer as usize].allocate(isize) {
                return Some(AtlasHandle { allocation, layer });
            }
        }

        None
    }

    pub fn size(&self) -> Vec3<u32> {
        let size = self.atlas[0].size();
        Vec3 {
            x: size.width as u32,
            y: size.height as u32,
            z: self.atlas.len() as u32,
        }
    }

    pub fn remove(&mut self, handle: AtlasHandle) {
        self.atlas[handle.layer as usize].deallocate(handle.allocation.id);
    }

    /// Fast function that resizes the atlas.
    pub fn grow(&mut self, min_size_increment: Vec2<u32>) {
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
    fn next_size(&self, min_size_increment: Vec2<u32>) -> Vec3<u32> {
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
    pub fn rect(&self) -> Rect<u32> {
        let min = self.allocation.rectangle.min;
        let max = self.allocation.rectangle.max;
        Rect {
            min: Vec2 {
                x: min.x as u32,
                y: min.y as u32,
            },
            max: Vec2 {
                x: max.x as u32,
                y: max.y as u32,
            },
        }
    }
}
