use crate::{query::filter::QueryFilter, types::NodeId, world::World};

impl QueryFilter for () {
    fn matches(_world: &World, _id: NodeId) -> bool {
        true
    }
}
