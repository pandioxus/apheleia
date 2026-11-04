use crate::{types::{NodeId, traits::LocalEvent}, world::World};

impl World {
    #[inline]
    pub fn mark_localevent<T: LocalEvent>(&mut self, id: NodeId) {
        self.local_events.mark::<T>(id);
    }

    #[inline]
    pub fn get_nodes_with_localevent<T: LocalEvent>(&mut self) -> &std::collections::HashSet<u16> {
        self.local_events.get_nodes::<T>()
    }

    #[inline]
    pub fn clear_localevents(&mut self) {
        self.local_events.clear();
    }
}
