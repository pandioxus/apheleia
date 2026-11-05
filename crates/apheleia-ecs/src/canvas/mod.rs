use std::{cell::RefCell, rc::Rc};

use apheleia_core::{buffer::Buffer, style::Style};

use crate::{nodedata::definition::NodeData, types::NodeId};

pub struct Canvas {
    id: NodeId,
    nodedata: NodeData,
    buffer: Rc<RefCell<Buffer>>,
}
impl Canvas {
    pub fn new(id: NodeId, nodedata: NodeData, buffer: Rc<RefCell<Buffer>>) -> Self {
        Canvas {
            id,
            nodedata,
            buffer,
        }
    }

    // TODO: Temporary to get something drawn
    pub fn write_text(&mut self, text: &str, position: (u16, u16), z: i8, style: Option<Style>) {
        self.buffer.borrow_mut().write(text, position, z, style);
    }
}
