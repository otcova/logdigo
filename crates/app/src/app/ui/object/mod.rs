mod block;

use crate::*;
use std::collections::HashMap;

pub use block::*;

pub trait ObjectBuilder {
    type Handle: ObjectHandle;
    fn build(self, ui: &mut UI) -> Self::Handle;
}

pub trait ObjectHandle {
    fn id(&self) -> ObjectId;
    fn delete(self, ui: &mut UI);
}

pub(crate) type ObjectId = usize;

#[derive(Default)]
pub(crate) struct Objects {
    blocks: HashMap<ObjectId, BlockObject>,
    old_id: ObjectId,
}

impl Objects {
    fn new_id(&mut self) -> ObjectId {
        self.old_id += 1;
        self.old_id
    }
}
