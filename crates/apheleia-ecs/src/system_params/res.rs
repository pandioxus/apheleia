use std::ops::Deref;

use crate::types::traits::{Resource, SystemParam};

pub struct Res<'w, T: Resource + 'static> {
    res: &'w T,
}
impl<'w, T: Resource + 'static> Res<'w, T> {
    pub(crate) fn new(res: &'w T) -> Self {
        Self { res }
    }
}

impl<'w, T: Resource + 'static> Deref for Res<'w, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.res
    }
}

impl<T: Resource + 'static> SystemParam for Res<'static, T> {
    unsafe fn fetch<'w>(world: *mut crate::world::World) -> Option<Self> {
        let world = unsafe {
            &* world
        };

        if let Some(res) = world.resource_get::<T>() {
            return Some(Res::new(res));
        }
        None
    }
}
