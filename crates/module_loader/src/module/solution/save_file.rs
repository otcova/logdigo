use crate::*;
use std::path::*;

pub struct SolutionSaveFile {
    pub blocks: Vec<Block>,
}

impl SolutionSaveFile {
    pub fn from_path(p: impl AsRef<Path>) -> Option<Self> {
        todo!()
    }
}
