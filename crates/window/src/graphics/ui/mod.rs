use std::sync::Arc;

use super::WgpuContext;
use digolog_math::Vec2;
use winit::window::Window;

pub struct UI {
    context: WgpuContext,
}

impl UI {
    pub fn create_panel() -> PanelHandle {
        todo!()
    }
    pub fn create_block() -> BlockHandle {
        todo!()
    }
    pub fn create_wire() -> WireHandle {
        todo!()
    }

    pub(crate) async fn new(window: Arc<Window>) -> Self {
        let context = WgpuContext::new(window).await;
        Self { context }
    }
    /// Returns true if the size has changed
    pub(crate) fn resize(&mut self, new_size: Vec2<u32>) {
        self.context.resize_surface(new_size);
    }

    pub(crate) fn render(&self) {
        self.context.render(|render_pass| {
            //
        });
    }
}
