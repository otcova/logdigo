use super::*;

pub struct BlockBuilder {
    pub color: wgpu::Color,
}

pub struct BlockHandle {
    id: ObjectId,
}

impl BlockBuilder {
    pub fn build(self, ui: &mut UI) -> BlockHandle {
        let id = ui.painters.new_object_id();
        BlockHandle { id }
    }
}

impl BlockHandle {
    pub fn id(&self) -> ObjectId {
        self.id
    }

    pub fn delete(self, _ui: &mut UI) {}
}
