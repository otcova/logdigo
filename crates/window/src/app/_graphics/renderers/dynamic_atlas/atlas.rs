use super::BlockShape;
use digolog_math::*;
use etagere::*;
use wgpu::naga::{FastHashMap, FastHashSet};

pub struct BlockAtlas {
    atlas: AtlasAllocator,

    /// All the BlockShapes that the texture atlas holds.
    atlas_blocks: FastHashMap<BlockShape, BlockAllocation>,
}

struct BlockAllocation {
    allocated: Allocation,
    /// Number of block instances that use this shape, and therefore
    /// share the same texture atlas rectangle.
    references: u32,
}

pub struct TextureAtlasRect {
    pub pos: Vec2<u16>,
    pub size: Vec2<u16>,
}

impl BlockAtlas {
    pub fn new(initial_size: Vec2<u32>) -> Self {
        Self {
            atlas: AtlasAllocator::new(size2(initial_size.x as i32, initial_size.y as i32)),
            atlas_blocks: FastHashMap::default(),
        }
    }

    /// Returns the rect of the shape if it already exited.
    /// It also will increse the shader reference count.
    pub fn share_shape(&mut self, shape: &BlockShape) -> Option<TextureAtlasRect> {
        let alloc = self.atlas_blocks.get_mut(shape)?;
        alloc.references += 1;
        Some(alloc.allocated.into())
    }

    /// Will allocate a new rectangle for the shape.
    /// If the shape does not fit, it will return None.
    ///
    /// # Panic
    /// If the shape already was allocated it will panic!
    pub fn allocate_shape(&mut self, shape: BlockShape) -> Option<TextureAtlasRect> {
        let size = size2(shape.size.x as i32, shape.size.y as i32);
        if let Some(alloc) = self.atlas.allocate(size) {
            let previous = self.atlas_blocks.insert(shape, alloc.into());
            if previous.is_some() {
                panic!("The shape already was allocated");
            }
            Some(alloc.into())
        } else {
            None
        }
    }

    /// Will decrees the shared reference count.
    /// If zero, it will deallocate the shape from the atlas.
    ///
    /// # Error
    /// If the shape is not allocated
    pub fn deref_block(&mut self, shape: &BlockShape) {
        if let Some(alloc) = self.atlas_blocks.get_mut(shape) {
            alloc.references -= 1;
            if alloc.references == 0 {
                self.atlas.deallocate(alloc.allocated.id);
                self.atlas_blocks.remove(shape);
            }
        } else {
            // This is not a fatal error.
            // TODO: Handle this case better.
            eprintln!("ERROR deleting already deallocated block");
        }
    }
}

impl From<Allocation> for BlockAllocation {
    fn from(texture: Allocation) -> Self {
        Self {
            allocated: texture,
            references: 1,
        }
    }
}

impl From<Allocation> for TextureAtlasRect {
    fn from(alloc: Allocation) -> Self {
        let pos = alloc.rectangle.min;
        let size = alloc.rectangle.size();
        Self {
            pos: Vec2 {
                x: pos.x as u16,
                y: pos.y as u16,
            },
            size: Vec2 {
                x: size.width as u16,
                y: size.height as u16,
            },
        }
    }
}
