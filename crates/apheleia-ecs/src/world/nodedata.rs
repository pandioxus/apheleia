use crate::{nodedata::NodeData, types::NodeId, world::World};

impl World {
    #[inline]
    pub fn nodedata_write(&mut self, id: NodeId, data: NodeData) {
        self.nodedata.write_data(id, data);
    }

    #[inline]
    pub fn nodedata_remove(&mut self, id: NodeId) {
        self.nodedata.remove(id);
    }

    #[inline]
    pub fn get_nodedata(&self, id: NodeId) -> Option<&NodeData> {
        self.nodedata.get(id)
    }

    #[inline]
    pub fn get_nodedata_mut(&mut self, id: NodeId) -> Option<&mut NodeData> {
        self.nodedata.get_mut(id)
    }
}
