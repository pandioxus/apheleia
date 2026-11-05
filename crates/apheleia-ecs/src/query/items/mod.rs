pub mod extension;
pub mod node;
pub mod canvas;

use std::collections::HashSet;

use crate::{types::NodeId, world::World};

pub trait QueryItem {
    type Item<'w>;
    fn match_ids(world: &World) -> Option<Vec<NodeId>>;
    unsafe fn fetch<'w>(world: *mut World, id: NodeId) -> Option<Self::Item<'w>>;
}

macro_rules! impl_world_query {
    ( $($query:ident),+ ) => {
        impl<$($query: QueryItem),+> QueryItem for ($($query,)*) {
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
