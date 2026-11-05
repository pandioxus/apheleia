use crate::{global_events::GlobalEvent, tags::TagTrait, world::World};

impl World {
    #[inline]
    pub fn mark_global_event<T: TagTrait, G: GlobalEvent>(&mut self) {
        self.global_events.mark::<T, G>();
    }

    #[inline]
    pub fn is_tag_marked_globalevent<T: TagTrait, G: GlobalEvent>(&self) -> bool {
        self.global_events.is_marked::<T, G>()
    }

    #[inline]
    pub fn clear_globalevents(&mut self) {
        self.global_events.clear();
    }
}
