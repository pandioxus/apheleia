use apheleia_core::buffer::Buffer;

use crate::{nodedata::NodeData, types::NodeId};

pub struct Canvas<'w> {
    id: NodeId,
    nodedata: NodeData,
    buffer: &'w mut Buffer,
}
impl<'w> Canvas<'w> {
    pub fn new(id: NodeId, nodedata: NodeData, buffer: &'w mut Buffer) -> Self {
        Canvas {
            id,
            nodedata,
            buffer,
        }
    }
}
