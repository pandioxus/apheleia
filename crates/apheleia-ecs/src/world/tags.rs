use std::collections::HashSet;

use crate::{tags::TagTrait, types::NodeId, world::World};

impl World {
    #[inline]
    pub fn tag_node<T: TagTrait>(&mut self, id: NodeId) {
        self.tags_registry.tag_node::<T>(id);
    }

    #[inline]
    pub fn is_node_tagged<T: TagTrait>(&self, id: NodeId) -> bool {
        self.tags_registry.is_node_tagged::<T>(id)
    }

    #[inline]
    pub fn get_nodes_tagged<T: TagTrait>(&self) -> &HashSet<NodeId> {
        self.tags_registry.get_nodes::<T>()
    }

    #[inline]
    pub fn clear_tags(&mut self) {
        self.tags_registry.clear();
    }
}
