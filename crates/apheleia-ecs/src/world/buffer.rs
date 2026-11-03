use apheleia_core::buffer::Buffer;

use crate::world::World;

impl World {
    pub fn get_buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn get_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
}
