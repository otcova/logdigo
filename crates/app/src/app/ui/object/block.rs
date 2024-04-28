use super::*;

pub struct BlockBuilder {
    pub color: wgpu::Color,
}

pub struct BlockHandle {
    id: ObjectId,
}

pub(crate) struct BlockObject {}

impl ObjectBuilder for BlockBuilder {
    type Handle = BlockHandle;

    fn build(self, ui: &mut UI) -> Self::Handle {
        let block = BlockObject {};

        let id = ui.objects.new_id();
        ui.objects.blocks.insert(id, block);
        BlockHandle { id }
    }
}

impl ObjectHandle for BlockHandle {
    fn id(&self) -> ObjectId {
        self.id
    }
    fn delete(self, ui: &mut UI) {
        ui.objects.blocks.remove(&self.id);
    }
}
