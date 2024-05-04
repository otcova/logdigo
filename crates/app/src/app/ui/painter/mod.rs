mod block;
mod wire;

use crate::*;
use block::*;
use wire::*;

pub struct Painters {
    pub main_camera: Camera2dBuffer,
    pub block: BlockPainter,
    pub wire: WirePainter,
    old_id: ObjectId,
}

impl Painters {
    pub fn new(renderer: &Renderer) -> Self {
        Self {
            main_camera: Camera2dBuffer::new(renderer),
            block: BlockPainter::new(renderer),
            wire: WirePainter::new(renderer),
            old_id: 0,
        }
    }

    pub fn new_object_id(&mut self) -> ObjectId {
        self.old_id += 1;
        self.old_id
    }

    pub fn resize(&mut self, new_size: u32x2) {
        self.main_camera.resize(new_size);
    }

    pub fn render(&mut self, encoder: &mut RendererEncoder, renderer: &mut Renderer) {
        self.main_camera.update_buffer(encoder, renderer);
        self.block.update_buffers(encoder, renderer);
        self.wire.update_buffers(encoder, renderer);

        let mut surface = encoder.surface_texture_target();
        let mut pass = surface.render_pass();

        self.block.render(&mut pass, renderer, &self.main_camera);
        self.wire.render(&mut pass, renderer, &self.main_camera);
    }
}
