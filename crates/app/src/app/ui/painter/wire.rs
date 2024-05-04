use self::{app::WireBuilder, ui::BlockBuilder};
use crate::*;
use digolog_math::*;
use std::collections::HashMap;

pub struct WirePainter {
    indexes: HashMap<ObjectId, LineObject>,
    lines: LinesBatch,
}

struct LineObject {
    line: InstanceId,
}

impl WirePainter {
    pub fn new(renderer: &Renderer) -> Self {
        Self {
            indexes: HashMap::new(),
            lines: LinesBatch::new(&renderer),
        }
    }

    pub fn update_buffers(&mut self, encoder: &mut RendererEncoder, renderer: &mut Renderer) {
        self.lines.update_buffers(encoder, renderer);
    }

    pub fn render<'a>(
        &'a mut self,
        render_pass: &mut RenderPass<'a>,
        renderer: &'a Renderer,
        camera: &'a Camera2dBuffer,
    ) {
        self.lines.render(render_pass, renderer, camera);
    }

    pub fn insert(&mut self, id: ObjectId, wire: WireBuilder) {
        let line = self.lines.push(LineInstance {
            position_a: wire.position_a.cast().into(),
            position_b: wire.position_b.cast().into(),
            color: *wire.color,
        });

        self.indexes.insert(id, LineObject { line });
    }
}
