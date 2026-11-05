use crate::{
    query::{Query, filter::QueryFilter, items::QueryItem},
    system::definitions::SystemParam,
    world::World,
};

impl<Q: QueryItem + 'static, F: QueryFilter + 'static> SystemParam for Query<'static, Q, F> {
    unsafe fn fetch(world: *mut World) -> Option<Self> {
        let world = unsafe { &mut *world };
        let query = Self::new(world);

        if query.is_empty() {
            return None;
        }
        Some(query)
    }
}
