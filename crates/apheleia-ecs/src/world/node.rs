use crate::{types::NodeId, world::World};

impl World {
    #[inline]
    pub fn node_create(&mut self) -> NodeId {
        let id = self.nodeid_gen.next();
        self.registered_nodeids.push(id);
        id
    }

    #[inline]
    pub fn node_delete(&mut self, id: NodeId) {
        self.registered_nodeids.retain(|&x| x != id);
        self.nodeid_gen.free(id);
    }

    #[inline]
    pub fn get_registered_nodes(&self) -> &Vec<NodeId> {
        &self.registered_nodeids
    }
}
