//! A painter uses renderers to draw objects

mod block;

use crate::*;
use block::*;

pub struct Painters {
    block: BlockPainter,
    old_id: ObjectId,
}

impl Painters {
    pub fn new(renderer: &Renderer) -> Self {
        Self {
            block: BlockPainter::new(renderer),
            old_id: 0,
        }
    }

    pub fn new_object_id(&mut self) -> ObjectId {
        self.old_id += 1;
        self.old_id
    }

    pub fn render(&mut self, encoder: &mut RendererEncoder, renderer: &Renderer) {
        let mut surface = encoder.surface_texture_target();
        let mut render_pass = surface.render_pass();

        self.block.render(&mut render_pass, renderer);
    }
}
