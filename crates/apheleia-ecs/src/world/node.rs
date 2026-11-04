use tree_ds::prelude::{Node, NodeRemovalStrategy};

use crate::{types::NodeId, world::World};

impl World {
    #[inline]
    pub fn node_create(&mut self) -> NodeId {
        let id = self.nodeid_gen.next();
        self.registered_nodeids.push(id);
        println!("Created ID: {}", id);
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

    pub fn relate_node_with_parent(&mut self, child: NodeId, parent: NodeId) {
        assert!(self.relations.get_node_by_id(&parent).is_some());
        if self.relations.get_node_by_id(&child).is_none() {
            self.relations
                .add_node(Node::new(child, None), Some(&parent))
                .unwrap();
        }

        // Retain children, and move the subtree along with the child if it has any
        // assert!(self.relations.get_node_by_id(&child).is_some());
        // let subtree = self
        //     .relations
        //     .get_subtree(&child, None)
        //     .expect("Couldn't get subtree");
        // self.relations
        //     .remove_node(&child, NodeRemovalStrategy::RemoveNodeAndChildren)
        //     .expect("Couldn't remove node from relations");
        // self.relations
        //     .add_subtree(&parent, subtree)
        //     .expect("Couldn't add subtree to relations");
    }
}
