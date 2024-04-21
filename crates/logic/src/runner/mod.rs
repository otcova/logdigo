use crate::*;
use derive_more::*;

pub struct ChapterRunner {
    chapter: Chapter,

    /// Used to generate new outputs.
    /// It should only be when a tick is completed.
    old_outputs: Vec<u8>,

    /// A buffer to store the new state.
    /// It should only be read to move the data to `old_outputs` when a tick is completed.
    new_outputs: Vec<u8>,
}

pub struct BlockPanelIndex(usize);

#[derive(Deref, DerefMut)]
pub struct BlockOutputMut<'a>(&'a mut [u8]);

#[derive(Deref)]
pub struct BlockInput<'a>(&'a [u8]);

pub enum BlockLogic {
    /// A stateless rust function. (example: Logic Gates)
    Builtin(fn(BlockInput, BlockOutputMut)),
}
