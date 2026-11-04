pub mod world_commands;
pub mod nodedata;
pub mod stores;
pub mod world;
pub mod system_params;
pub mod canvas;

pub mod types {
    pub type NodeId = u16;

    #[derive(PartialEq, Eq)]
    pub enum SystemStage {
        Event = 0,

        PreUpdate = 1,
        Update = 2,
        PostUpdate = 3,

        PreRender = 4,
        Render = 5,
        PostRender = 6
    }


    pub mod traits {
        use std::{
            any::Any, fmt::Debug, sync::{OnceLock, atomic::AtomicUsize}
        };

        use crate::world::World;

        pub trait Resource: Any + Debug + 'static {}

        static COMPONENT_ID_COUNT: AtomicUsize = AtomicUsize::new(0);
        pub trait Extension: Any + Debug + 'static {
            fn id() -> usize
            where
                Self: Sized,
            {
                static ID: OnceLock<usize> = OnceLock::new();
                *ID.get_or_init(|| {
                    COMPONENT_ID_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
                })
            }
        }

        pub trait SystemParam: Sized + 'static {
            unsafe fn fetch<'w>(world: *mut World) -> Option<Self>;
        }
    }
}
