mod object;
mod painter;

use super::Renderer;

use crate::*;
pub use object::*;
use painter::*;

/// Drawing interface for the AppBrain.
pub struct UI {
    renderer: Renderer,
    painters: Painters,
}

impl UI {
    pub(crate) fn new(renderer: Renderer) -> Self {
        Self {
            painters: Painters::new(&renderer),
            renderer,
        }
    }

    pub(crate) fn render(&mut self) {
        self.painters.render(&mut self.renderer);
    }
    pub(crate) fn resize(&mut self, new_size: u32x2) {
        if self.renderer.resize(new_size) {
            self.painters.resize(new_size);
        }
    }

    pub(crate) fn request_redraw(&self) {
        self.renderer.window.request_redraw();
    }
}
