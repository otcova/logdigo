use crate::*;
use derive_more::*;

pub enum BlockLogic {
    /// A rust function. (example: Logic Gates)
    Builtin(fn(BlockInput, BlockOutputMut)),
}

#[derive(Deref, DerefMut)]
pub struct BlockOutputMut<'a>(&'a mut [u8]);

#[derive(Deref)]
pub struct BlockInput<'a>(&'a [u8]);

