use std::{marker::PhantomData, slice};
use std::collections::HashSet;

use crate::types::traits::SystemParam;
use crate::{types::NodeId, world::World};

pub trait QueryType {
    type Item<'w>;
    fn match_ids(world: &World) -> Option<Vec<NodeId>>;
    unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Option<Self::Item<'w>>;
}

macro_rules! impl_world_query {
    ( $($query:ident),+ ) => {
        impl<$($query: QueryType),+> QueryType for ($($query,)*) {
            type Item<'w> = ($($query::Item<'w>),*);

            fn match_ids(world: &World) -> Option<Vec<NodeId>> {
                let mut sets: Vec<HashSet<NodeId>> = Default::default();
                $(
                    if let Some(ids) = $query::match_ids(world) {
                        sets.push(ids.into_iter().collect());
                    }
                )+

                if sets.is_empty() {
                    return None;
                }
                Some(sets[0]
                    .iter()
                    .filter(|id| sets[1..].iter().all(|s| s.contains(id)))
                    .copied()
                    .collect())
            }

            unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Option<Self::Item<'w>> {
                unsafe {
                $(
                    let $query = $query::fetch(world, id)?;
                )+

                Some( ($($query),+) )
                }
            }
        }
    };
}

impl_world_query!(Q0, Q1);
impl_world_query!(Q0, Q1, Q2);
impl_world_query!(Q0, Q1, Q2, Q3);
impl_world_query!(Q0, Q1, Q2, Q3, Q4);
impl_world_query!(Q0, Q1, Q2, Q3, Q4, Q5);
impl_world_query!(Q0, Q1, Q2, Q3, Q4, Q5, Q6);


pub trait QueryFilter {
    fn matches(world: &World, id: NodeId) -> bool;
}

impl QueryFilter for () {
    fn matches(_world: &World, _id: NodeId) -> bool {
        true
    }
}


macro_rules! impl_query_filter {
    ( $($filter:ident),+ ) => {
        impl< $($filter: QueryFilter),+ > QueryFilter for ( $($filter,)* ) {
            fn matches(world: &World, id: NodeId) -> bool {
                $($filter::matches(world, id))&&+
            }
        }
    };
}

impl_query_filter!(F0, F1);
impl_query_filter!(F0, F1, F2);
impl_query_filter!(F0, F1, F2, F3);

pub struct Query<'w, Q: QueryType, F: QueryFilter = ()> {
    world: *mut World,
    ids: Vec<NodeId>,
    _marker: PhantomData<(&'w (), Q, F)>,
}

impl<'w, Q: QueryType, F: QueryFilter> Query<'w, Q, F> {
    pub(crate) fn new(world: &'w mut World) -> Self {
        let mut ids = Q::match_ids(world)
            .unwrap_or_else(|| world.get_registered_nodes().to_vec());
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

pub struct QueryIter<'a, Q: QueryType> {
    world: *mut World,
    ids: slice::Iter<'a, NodeId>,

    _marker: PhantomData<(&'a mut World, Q)>,
}
impl<'a, Q: QueryType> QueryIter<'a, Q> {
    pub(crate) fn new(world: *mut World, ids: slice::Iter<'a, NodeId>) -> Self {
        Self {
            world,
            ids,
            _marker: PhantomData,
        }
    }
}

impl<'a, Q: QueryType> Iterator for QueryIter<'a, Q> {
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

impl<Q: QueryType + 'static, F: QueryFilter + 'static> SystemParam for Query<'static, Q, F> {
    unsafe fn fetch(world: *mut World) -> Option<Self> {
        let world = unsafe { &mut *world };
        let query = Self::new(world);

        if query.is_empty() {
            return None;
        }
        Some(query)
    }
}
