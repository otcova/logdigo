use smallvec::{smallvec, SmallVec};
use wgpu::naga::FastHashMap;

use crate::graphics::models::*;
use std::ops::Range;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Handle {
    id: u64,
}

pub struct ModifiedBytes<'a> {
    pub offset: wgpu::BufferAddress,
    pub data: &'a [u8],
}

pub struct RetainedInstanceBuffer<M>
where
    M: Model,
    [(); M::Buffer::ARRAYS]: Sized,
{
    instances: M::Buffer,
    /// Relation: instance Handle -> elements indexes
    indexes: FastHashMap<Handle, [SmallVec<Range<u32>, 1>; M::Buffer::ARRAYS]>,
    bigger_handle_id: u64,
    /// Relation: elements indexes -> instance Handle
    handles: [Vec<Handle>; M::Buffer::ARRAYS],

    /// Range of elements that have changed and need to be uploaded to the gpu.
    modified_range: [Range<u32>; M::Buffer::ARRAYS],
}

impl<M> RetainedInstanceBuffer<M>
where
    M: Model,
    [(); M::Buffer::ARRAYS]: Sized,
{
    pub fn new() -> Self {
        Self {
            instances: Default::default(),
            indexes: Default::default(),
            bigger_handle_id: 0,
            handles: std::array::from_fn(|_| vec![]),
            modified_range: std::array::from_fn(|_| 0..0),
        }
    }

    pub fn push(&mut self, instance: <M::Buffer as InstanceBuffer>::Instance) -> Handle {
        let handle = self.new_handle();

        // Push instance
        let new_indexes = self.instances.push(instance);

        for array_i in 0..new_indexes.len() {
            // Update Relation `instance indexes -> Handle`
            for _ in new_indexes[array_i].clone() {
                self.handles[array_i].push(handle);
            }

            // Update Relation `instance indexes -> Handle`
        }

        // Update Relation `Handle -> instance indexes`
        // let new_indexes = new_indexes.map(|range| smallvec![range]);
        let new_indexes = std::array::from_fn(|_| smallvec![]);
        self.indexes.insert(handle, new_indexes);

        handle
    }

    pub fn remove(&mut self, handle: Handle) {
        let Some(indexes) = self.indexes.remove(&handle) else {
            // WARN: Removing invalid handle
            return;
        };

        for array_i in 0..indexes.len() {
            for elements in indexes[array_i].clone() {
                let range = elements.start as usize..elements.end as usize;
                swap_drain(&mut self.handles[array_i], range);
                self.instances.swap_drain(array_i as u32, elements);
            }
        }
    }

    pub fn bytes_of(&self, array_index: u32) -> &[u8] {
        self.instances.bytes_of(array_index)
    }

    /// Returns the modified bytes since the last call of this function
    pub fn modified_bytes(&mut self) -> [ModifiedBytes; M::Buffer::ARRAYS] {
        let bytes = self.instances.bytes();
        let modified = std::array::from_fn(|i| {
            let start = self.modified_range[i].start as usize;
            let end = bytes[i].len().min(self.modified_range[i].end as usize);
            ModifiedBytes {
                offset: start as wgpu::BufferAddress,
                data: &bytes[i][start..end],
            }
        });
        self.modified_range.fill(0..0);
        modified
    }

    fn set_modified_indexes(&mut self, array_i: u32, updated: Range<u32>) {
        let array_range = &mut self.modified_range[array_i as usize];
        *array_range = combine_range(array_range.clone(), updated);
    }

    fn new_handle(&mut self) -> Handle {
        self.bigger_handle_id += 1;
        Handle {
            id: self.bigger_handle_id,
        }
    }
}

/// Returns the smaller range that includes the elements of `a` and `b`
/// # Examples:
/// ```
/// assert!(combine_range(3..6, 9..10) == 3..10);
/// assert!(combine_range(3..6, 9..9) == 3..6);
/// ```
fn combine_range(a: Range<u32>, b: Range<u32>) -> Range<u32> {
    if a.is_empty() {
        return b;
    }
    if b.is_empty() {
        return a;
    }
    u32::min(a.start, b.start)..u32::max(a.end, b.end)
}
