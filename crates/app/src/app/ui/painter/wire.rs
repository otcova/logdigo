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
            lines: LinesBatch::new(renderer),
        }
    }

    pub fn update_buffers(&mut self, renderer: &mut Renderer) {
        self.lines.update_buffers(renderer);
    }

    pub fn bundle_render<'a>(
        &'a self,
        bundle: &mut RenderBundleEncoder<'a>,
        renderer: &'a Renderer,
    ) {
        self.lines.bundle_render(bundle, renderer);
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
