// use std::marker::PhantomData;
//
// use crate::{extension::traits::Extension, query::filter::QueryFilter, types::NodeId, world::World};
//
// pub struct With<T: Extension> {
//     marker: PhantomData<T>,
// }
// impl<T: Extension> QueryFilter for With<T> {
//     fn matches(world: &World, id: NodeId) -> bool {
//         world.get_extension::<T>(id).is_some()
//     }
// }
//
// pub struct Without<T: Extension> {
//     marker: PhantomData<T>,
// }
// impl<T: Extension> QueryFilter for Without<T> {
//     fn matches(world: &World, id: NodeId) -> bool {
//         world.get_extension::<T>(id).is_none()
//     }
// }
