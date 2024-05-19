mod camera;
mod image;

pub use camera::*;
pub use image::*;

use std::ops::{Range, RangeBounds};

pub struct BindGroupLayouts {
    camera: wgpu::BindGroupLayout,
    image: wgpu::BindGroupLayout,
}

impl BindGroupLayouts {
    pub fn new(device: &wgpu::Device) -> Self {
        BindGroupLayouts {
            camera: camera::create_layout(device),
            image: ImagePipeline::create_layout(device),
        }
    }
}

pub trait ModelPipeline {
    type Buffer: InstanceBuffer;

    fn new(context: &WgpuContext) -> Self;
    fn render(
        &self,
        pass: &mut wgpu::RenderPass,
        slices: [wgpu::BufferSlice; Self::Buffer::ARRAYS],
    );
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

macro_rules! vec_instance_buffer {
    (struct $Buffer:ident {
        $vec:ident: Vec<$Instance:ident>,
    }) => {
        #[derive(Default)]
        struct $Buffer {
            $vec: Vec<$Instance>,
        }

        impl InstanceBuffer for $Buffer {
            type Instance = $Instance;
            const ARRAYS: usize = 1;

            fn push(&mut self, instance: $Instance) -> [Range<u32>; 1] {
                let index = self.$vec.len() as u32;
                self.$vec.push(instance);
                [index..index + 1]
            }

            fn swap_drain(&mut self, _: u32, range: Range<u32>) {
                let range = range.start as usize..range.end as usize;
                swap_drain(&mut self.$vec, range);
            }

            fn clear(&mut self) {
                self.$vec.clear();
            }

            fn bytes_of(&self, array_index: u32) -> &[u8] {
                bytemuck::cast_slice(&self.$vec)
            }
        }
    };
}
use vec_instance_buffer;

use super::WgpuContext;
