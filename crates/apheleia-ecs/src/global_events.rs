use std::{
    any::{Any, TypeId},
    collections::HashSet,
    fmt::Debug,
};

use rustc_hash::FxHashMap;

use crate::{tags::TagTrait};

pub trait GlobalEvent: Any + Debug + 'static {}

#[derive(Default)]
pub struct GlobalEventRegistry {
    events: FxHashMap<TypeId, HashSet<TypeId>>,
}
impl GlobalEventRegistry {
    pub fn mark<T: TagTrait, G: GlobalEvent>(&mut self) {
        self.events
            .entry(TypeId::of::<T>())
            .and_modify(|set| {
                set.insert(TypeId::of::<G>());
            })
            .or_default();
    }

    pub fn is_marked<T: TagTrait, G: GlobalEvent>(&self) -> bool {
        let Some(events) = self.events.get(&TypeId::of::<T>()) else {
            return false;
        };

        events.contains(&TypeId::of::<G>())
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}
