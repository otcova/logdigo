use crate::*;

pub struct BlockBuilder {
    pub position: i32x2,
    pub size: u16x2,
    pub color: Color,
}

pub struct BlockHandle {
    id: ObjectId,
}

impl BlockBuilder {
    pub fn build(self, ui: &mut UI) -> BlockHandle {
        let id = ui.painters.new_object_id();
        ui.painters.block.insert(id, self);
        BlockHandle { id }
    }
}

impl BlockHandle {
    pub fn id(&self) -> ObjectId {
        self.id
    }

    pub fn delete(self, _ui: &mut UI) {}
}
