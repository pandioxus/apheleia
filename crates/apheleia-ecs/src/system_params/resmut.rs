use std::ops::{Deref, DerefMut};

use crate::types::traits::{Resource, SystemParam};

pub struct ResMut<'w, T: Resource + 'static> {
    res: &'w mut T,
}
impl<'w, T: Resource + 'static> ResMut<'w, T> {
    pub(crate) fn new(res: &'w mut T) -> Self {
        Self { res }
    }
}

impl<'w, T: Resource + 'static> Deref for ResMut<'w, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.res
    }
}
impl<'w, T: Resource + 'static> DerefMut for ResMut<'w, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.res
    }
}

impl<T: Resource + 'static> SystemParam for ResMut<'static, T> {
    unsafe fn fetch<'w>(world: *mut crate::world::World) -> Option<Self> {
        let world = unsafe {
            &mut *world
        };

        if let Some(res) = world.resource_get_mut::<T>() {
            return Some(ResMut::new(res));
        }
        None
    }
}
