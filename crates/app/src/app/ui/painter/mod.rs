mod block;
mod wire;

use crate::*;
use block::*;
use wire::*;

pub struct Painters {
    pub main_camera: Camera2dBuffer,
    pub block: BlockPainter,
    pub wire: WirePainter,
    main_render_phase: RenderPhase,
    old_id: ObjectId,
}

impl Painters {
    pub fn new(renderer: &Renderer) -> Self {
        let main_camera = Camera2dBuffer::new(renderer);
        let block = BlockPainter::new(renderer);
        let wire = WirePainter::new(renderer);

        let main_render_phase = renderer.create_phase(|bundle| {
            main_camera.bundle_render(bundle);
            block.bundle_render(bundle, renderer);
            wire.bundle_render(bundle, renderer);
        });

        Self {
            main_camera,
            block,
            wire,
            main_render_phase,
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

    pub fn render(&mut self, renderer: &mut Renderer) {
        self.main_camera.update_buffer(renderer);
        self.block.update_buffers(renderer);
        self.wire.update_buffers(renderer);

        renderer.render([&self.main_render_phase]);
    }
}
