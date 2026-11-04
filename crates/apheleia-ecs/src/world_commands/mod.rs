pub mod resource;
pub mod node;
pub mod extension;
pub mod nodedata;

use std::fmt::Debug;

use crate::world::World;

pub trait WorldCommand: Debug {
    fn execute(self: Box<Self>, world: &mut World);
}
