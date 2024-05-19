mod objects;

use super::WgpuContext;
use digolog_math::Vec2;
use std::sync::Arc;
use winit::window::Window;

pub use objects::*;

pub struct UI {
    context: WgpuContext,
    objects: Objects,
}

impl UI {
    pub fn add<T: UIObject>(&mut self, object: T) -> Handle<T> {
        self.objects.add(object)
    }

    pub(crate) async fn new(window: Arc<Window>) -> Self {
        let context = WgpuContext::new(window).await;
        let objects = Objects::new(&context);
        Self { context, objects }
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
