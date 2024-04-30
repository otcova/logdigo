mod block;

use crate::*;
use block::*;

pub struct Painters {
    pub main_camera: Camera2d,
    pub block: BlockPainter,
    old_id: ObjectId,
}

impl Painters {
    pub fn new(renderer: &Renderer) -> Self {
        Self {
            main_camera: Camera2d::new(renderer),
            block: BlockPainter::new(renderer),
            old_id: 0,
        }
    }

    pub fn new_object_id(&mut self) -> ObjectId {
        self.old_id += 1;
        self.old_id
    }

    pub fn render(&mut self, encoder: &mut RendererEncoder, renderer: &mut Renderer) {
        self.main_camera.update_buffer(encoder, renderer);
        self.block.update_buffers(encoder, renderer);

        let mut surface = encoder.surface_texture_target();
        let mut pass = surface.render_pass();

        self.block.render(&mut pass, renderer, &self.main_camera);
    }
}
