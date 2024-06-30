mod block;
mod wire;

use crate::*;
use block::*;
use derive_more::*;
use wire::*;

pub struct RenderContext {
    pub main_camera: Camera2dBuffer,
    pub block: BlockPainter,
    pub wire: WirePainter,
}
impl RenderContext {
    fn create_render_phase(&self, renderer: &Renderer) -> RenderPhase {
        renderer.create_phase(|bundle| {
            self.main_camera.bundle_render(bundle);
            self.block.bundle_render(bundle, renderer);
            self.wire.bundle_render(bundle, renderer);
        })
    }

    /// If the new buffers are allocated, the RenderPhase needs to be rebuilt.
    fn update_buffers(&mut self, renderer: &mut Renderer) -> Option<RenderPhase> {
        self.main_camera.update_buffer(renderer);

        let status = [
            self.block.update_buffers(renderer),
            self.wire.update_buffers(renderer),
        ];

        if !BufferUpdateStatus::all_done(&status) {
            return Some(self.create_render_phase(renderer));
        }

        None
    }
}

#[derive(Deref, DerefMut)]
pub struct Painters {
    #[deref]
    #[deref_mut]
    render_context: RenderContext,
    main_render_phase: RenderPhase,
    old_id: ObjectId,
}

impl Painters {
    pub fn new(renderer: &Renderer) -> Self {
        let render_context = RenderContext {
            main_camera: Camera2dBuffer::new(renderer),
            block: BlockPainter::new(renderer),
            wire: WirePainter::new(renderer),
        };

        let main_render_phase = render_context.create_render_phase(renderer);

        Self {
            render_context,
            main_render_phase,
            old_id: 0,
        }
    }

    pub fn new_object_id(&mut self) -> ObjectId {
        self.old_id += 1;
        self.old_id
    }

    pub fn resize(&mut self, new_size: Vec2<u32>) {
        self.render_context.main_camera.resize(new_size);
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        if let Some(new_phase) = self.render_context.update_buffers(renderer) {
            self.main_render_phase = new_phase;
        }

        renderer.render([&self.main_render_phase]);
    }
}
