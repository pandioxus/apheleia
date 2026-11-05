use std::{any::{Any, TypeId}, collections::HashSet, fmt::Debug};

use rustc_hash::FxHashMap;

use crate::types::NodeId;

pub trait TagTrait: Any + Debug + 'static {}

#[derive(Default)]
pub struct TagRegistry {
    tags: FxHashMap<TypeId, HashSet<NodeId>>,
}
impl TagRegistry {
    pub fn tag_node<T: TagTrait>(&mut self, id: NodeId) {
        self.tags
            .entry(TypeId::of::<T>())
            .and_modify(|set| {
                set.insert(id);
            }).or_default();
    }

    pub fn is_node_tagged<T: TagTrait>(&self, id: NodeId) -> bool {
        let Some(nodes) = self.tags.get(&TypeId::of::<T>()) else {
            return false;
        };

        nodes.contains(&id)
    }

    pub fn get_nodes<T: TagTrait>(&self) -> &HashSet<NodeId> {
        self.tags.get(&TypeId::of::<T>()).unwrap()
    }

    pub fn clear(&mut self) {
        self.tags.clear();
    }
}
