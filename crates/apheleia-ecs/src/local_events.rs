use std::{any::{Any, TypeId}, collections::HashSet, fmt::Debug};

use rustc_hash::FxHashMap;

use crate::types::NodeId;

pub trait LocalEvent: Any + Debug + 'static {}

#[derive(Default)]
pub struct LocalEventRegistry {
    events: FxHashMap<TypeId, HashSet<NodeId>>,
}
impl LocalEventRegistry {
    pub fn mark<T: LocalEvent>(&mut self, id: NodeId) {
        self.events
            .entry(TypeId::of::<T>())
            .and_modify(|set| {
                set.insert(id);
            }).or_default();
    }

    pub fn get_nodes<T: LocalEvent>(&mut self) -> &HashSet<NodeId> {
        self.events.get(&TypeId::of::<T>()).unwrap()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}
