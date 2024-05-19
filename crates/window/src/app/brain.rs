use super::UI;
use crate::*;

pub trait AppBrain: Send + 'static {
    /// Run once the window is ready to be used.
    fn init(&mut self, ui: &mut UI);
}
