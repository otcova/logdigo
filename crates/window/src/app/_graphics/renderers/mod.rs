//! Expected Use
//! ```
//! let atlas = DynamicAtlas::new(...);
//! let images = ImageRenderer::new(...);
//! let wires = WireRenderer::new(...);
//!
//! let block_a = BlockShape { ... };
//! let block_a_texture = atlas.add_block(&block_a);
//!
//! let block_image = Image { block_a_texture, ... };
//! let image_handle = images.add_image(block_image);
//!
//! let wire = Wire { ... };
//! let wire_handle = wires.add_wire(wire);
//!
//! block_a_texture.render();
//! image_handle.render();
//! wires.render();
//!
//! images.get_mut(image_handle).position += Vec2::splat(0.1);
//!
//! block_a_texture.render(); // This won't do anything as the shape is already rendered
//! image_handle.render(); // This will update a buffer and make a draw call
//! wires.render(); // This will only do a draw call
//!
//!
//! atlas.remove_block(block_a);
//! images.remove_image(image_handle);
//! wires.remove_wire(wire_handle);
//!
//!
//! ```

mod dynamic_atlas;
