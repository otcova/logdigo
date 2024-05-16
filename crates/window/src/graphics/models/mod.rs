mod camera;

pub use camera::*;

use std::ops::{Range, RangeBounds};

pub trait Model {
    type Buffer: InstanceBuffer;

    fn render(pass: &mut wgpu::RenderPass, slices: [wgpu::BufferSlice; Self::Buffer::ARRAYS]);
}

pub trait InstanceBuffer: Default {
    type Instance: Copy;

    /// Amount of arrays that it has internally.
    const ARRAYS: usize;

    /// Push all the necesary elements to draw the instance into the arrays.
    /// Return the indexes of the elements that have been added.
    fn push(&mut self, instance: Self::Instance) -> [Range<u32>; Self::ARRAYS];

    /// `array_index`: index that represents an array of the InstanceBuffer.
    /// `elements`: range of elements from the array that will be removed.
    ///
    /// # Precondition
    /// `array_index` must be in 0..Self::ARRAYS
    fn swap_drain(&mut self, array_index: u32, elements: Range<u32>);

    /// Returns the arrays with the data of all the instances
    fn bytes_of(&self, array_index: u32) -> &[u8];

    /// Returns the arrays with the data of all the instances
    fn bytes(&self) -> [&[u8]; Self::ARRAYS] {
        std::array::from_fn(|array_index| self.bytes_of(array_index as u32))
    }

    /// Remove all the instances
    fn clear(&mut self);
}

/// Drains a range of elements from a vec replacing them by the elements of the back.
///
/// # Examples
/// ```
/// let mut v = vec![1, 2, 3, 4, 5, 6];
/// swap_drain(&mut v, 1..3);
/// assert_eq!(v, vec![1, 5, 6, 4]);
/// ```
/// ```
/// let mut v = vec![1, 2, 3, 4, 5, 6];
/// swap_drain(&mut v, 3..5);
/// assert_eq!(v, vec![1, 5, 6, 4]);
/// ```
pub fn swap_drain<T, R>(v: &mut Vec<T>, range: R)
where
    T: Copy,
    R: RangeBounds<usize>,
{
    let range = std::slice::range(range, ..v.len());

    let final_len = v.len() - range.len();
    let (dst, src) = v.split_at_mut(final_len);

    // Step 1: Move element from the back to fill the empty range
    let end = usize::min(range.end, dst.len());
    dst[range.start..end].copy_from_slice(src);

    // Step 2: Resize the vec
    v.truncate(final_len);
}
