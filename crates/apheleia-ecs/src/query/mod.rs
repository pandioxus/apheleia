pub mod filter;
pub mod items;

use std::{marker::PhantomData, slice};

use crate::query::filter::QueryFilter;
use crate::query::items::QueryItem;
use crate::{types::NodeId, world::World};

pub struct Query<'w, Q: QueryItem, F: QueryFilter = ()> {
    world: *mut World,
    ids: Vec<NodeId>,
    _marker: PhantomData<(&'w (), Q, F)>,
}

impl<'w, Q: QueryItem, F: QueryFilter> Query<'w, Q, F> {
    pub(crate) fn new(world: &'w mut World) -> Self {
        let mut ids = Q::match_ids(world).unwrap_or_else(|| world.get_registered_nodes().to_vec());
        ids.retain(|id| F::matches(world, *id));

        Self {
            world,
            ids,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    pub fn iter(&self) -> QueryIter<'_, Q> {
        QueryIter::new(self.world, self.ids.iter())
    }
}

pub struct QueryIter<'a, Q: QueryItem> {
    world: *mut World,
    ids: slice::Iter<'a, NodeId>,

    _marker: PhantomData<(&'a mut World, Q)>,
}
impl<'a, Q: QueryItem> QueryIter<'a, Q> {
    pub(crate) fn new(world: *mut World, ids: slice::Iter<'a, NodeId>) -> Self {
        Self {
            world,
            ids,
            _marker: PhantomData,
        }
    }
}

impl<'a, Q: QueryItem> Iterator for QueryIter<'a, Q> {
    type Item = Q::Item<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.ids.len() > 0 {
            let id = *self.ids.next().unwrap();
            let world = unsafe { &mut *self.world };

            if let Some(query) = unsafe { Q::fetch(world, id) } {
                return Some(query);
            }
        }
        None
    }
}
