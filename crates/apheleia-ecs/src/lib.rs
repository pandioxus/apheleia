pub mod world_commands;
pub mod nodedata;
pub mod world;
pub mod canvas;
pub mod extension;
pub mod resource;
pub mod system;
pub mod local_events;
pub mod query;

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
}
