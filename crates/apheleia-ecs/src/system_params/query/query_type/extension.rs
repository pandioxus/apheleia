use crate::{system_params::query::QueryType, types::{NodeId, traits::Extension}, world::World};

/// Implement [`QueryType`] for reference to [`Extension`]
impl<E: Extension> QueryType for &E {
    type Item<'w> = &'w E;

    fn match_ids(world: &World) -> Option<Vec<NodeId>> {
        Some(world.get_nodeids_with_extension::<E>().to_vec())
    }

    unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Option<Self::Item<'w>> {
        let world = unsafe { &*world };
        world.get_extension::<E>(id)
    }
}

/// Implement [`QueryType`] for mutable referece to [`Extension`]
impl<E: Extension> QueryType for &mut E {
    type Item<'w> = &'w mut E;

    fn match_ids(world: &World) -> Option<Vec<NodeId>> {
        Some(world.get_nodeids_with_extension::<E>().to_vec())
    }

    unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Option<Self::Item<'w>> {
        let world = unsafe { &mut *world };
        world.get_extension_mut::<E>(id)
    }
}

