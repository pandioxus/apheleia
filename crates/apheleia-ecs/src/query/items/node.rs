use crate::{nodedata::definition::NodeData, query::items::QueryItem, types::NodeId, world::World};

impl QueryItem for NodeData {
    type Item<'w> = &'w NodeData;

    fn match_ids(_world: &World) -> Option<Vec<NodeId>> {
        None
    }

    unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Option<Self::Item<'w>> {
        let world = unsafe { &*world };
        if let Some(data) = world.get_nodedata(id) {
            return Some(data);
        }
        None
    }
}

impl QueryItem for NodeId {
    type Item<'w> = NodeId;

    fn match_ids(_world: &World) -> Option<Vec<NodeId>> {
        None
    }

    unsafe fn fetch<'w>(_world: *mut World, id: NodeId) -> Option<Self::Item<'w>> {
        Some(id)
    }
}
