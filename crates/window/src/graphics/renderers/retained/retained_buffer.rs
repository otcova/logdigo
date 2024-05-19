use smallvec::{smallvec, SmallVec};
use wgpu::naga::FastHashMap;

use crate::graphics::models::*;
use std::ops::Range;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Id {
    id: u64,
}

pub struct ModifiedBytes<'a> {
    pub offset: wgpu::BufferAddress,
    pub data: &'a [u8],
}

pub struct RetainedInstanceBuffer<M>
where
    M: ModelPipeline,
    [(); M::Buffer::ARRAYS]: Sized,
{
    instances: M::Buffer,
    /// Relation: instance Id -> elements indexes
    indexes: FastHashMap<Id, [SmallVec<Range<u32>, 1>; M::Buffer::ARRAYS]>,
    bigger_id: u64,
    /// Relation: elements indexes -> instance Id
    ids: [Vec<Id>; M::Buffer::ARRAYS],

    /// Range of elements that have changed and need to be uploaded to the gpu.
    modified_range: [Range<u32>; M::Buffer::ARRAYS],
}

impl<M> RetainedInstanceBuffer<M>
where
    M: ModelPipeline,
    [(); M::Buffer::ARRAYS]: Sized,
{
    pub fn new() -> Self {
        Self {
            instances: Default::default(),
            indexes: Default::default(),
            bigger_id: 0,
            ids: std::array::from_fn(|_| vec![]),
            modified_range: std::array::from_fn(|_| 0..0),
        }
    }

    pub fn push(&mut self, instance: <M::Buffer as InstanceBuffer>::Instance) -> Id {
        let id = self.new_id();

        // Push instance
        let new_indexes = self.instances.push(instance);

        // Update Modified Range
        for (array_i, range) in new_indexes.iter().enumerate() {
            self.set_modified_indexes(array_i as u32, range.clone());
        }

        // Update Relation `instance indexes -> Id`
        for array_i in 0..new_indexes.len() {
            for _ in new_indexes[array_i].clone() {
                self.ids[array_i].push(id);
            }
        }

        // Update Relation `Id -> instance indexes`
        let new_indexes = new_indexes.map(|i| smallvec![i]);
        self.indexes.insert(id, new_indexes);

        id
    }

    pub fn remove(&mut self, handle: Id) {
        let Some(indexes) = self.indexes.remove(&handle) else {
            // WARN: Removing invalid handle
            return;
        };

        for array_i in 0..indexes.len() {
            for elements in indexes[array_i].clone() {
                let range = elements.start as usize..elements.end as usize;
                swap_drain(&mut self.ids[array_i], range);
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

    fn new_id(&mut self) -> Id {
        self.bigger_id += 1;
        Id { id: self.bigger_id }
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
