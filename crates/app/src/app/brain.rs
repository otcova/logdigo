pub trait AppBrain: Send + 'static {
    /// Run once the window is ready to be used.
    fn init(&mut self);
}
