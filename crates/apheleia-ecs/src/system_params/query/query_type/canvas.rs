use crate::{canvas::Canvas, nodedata::NodeData, system_params::query::QueryType};

impl QueryType for Canvas {
    type Item<'w> = Canvas;

    fn match_ids(world: &crate::world::World) -> Option<Vec<crate::types::NodeId>> {
        None
    }

    unsafe fn fetch<'w>(world: *mut crate::world::World, id: crate::types::NodeId) -> Option<Self::Item<'w>> {
        let world = unsafe {
            &mut *world
        };

        if let Some(data) = world.get_nodedata(id) {
            return Some(Canvas::new(id, data.clone(), world.buffer.clone()));
        }
        None
    }
}
