mod block_logic;
mod block_shape;
mod block_template;

use crate::*;
pub use block_logic::*;
pub use block_shape::*;
pub use block_template::*;

/// A placed block on a chapter
pub struct Block {
    // [Perfomance TODO] Share Block struct for multipl BlockPanel instances. (Change 'block' to be an index or a Rc<>)
    pub shape: BlockShape,
    pub pos: Vec2,
}
