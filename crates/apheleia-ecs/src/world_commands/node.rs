use std::{fmt::Debug, mem::take};

use crate::{
    extension::traits::Extension, nodedata::definition::NodeData, types::NodeId, world::World, world_commands::WorldCommand
};

pub trait NodeSubCommand: Debug {
    fn execute(self: Box<Self>, id: NodeId, world: &mut World);
}

#[derive(Debug)]
pub struct CreateNode {
    parent_id: NodeId,
    id: NodeId,
    sub_commands: Vec<Box<dyn NodeSubCommand>>,
    child_commands: Option<Vec<Box<dyn WorldCommand>>>,
}
impl CreateNode {
    pub fn new(parent_id: NodeId, id: NodeId, sub_commands: Vec<Box<dyn NodeSubCommand>>, child_commands: Option<Vec<Box<dyn WorldCommand>>>) -> Box<Self> {
        Box::new(Self {
            parent_id,
            id,
            sub_commands,
            child_commands
        })
    }
}
impl WorldCommand for CreateNode {
    fn execute(mut self: Box<Self>, world: &mut crate::world::World) {
        let commands = take(&mut self.sub_commands);
        for command in commands {
            command.execute(self.id, world);
        }

        world.relate_node_with_parent(self.id, self.parent_id);

        let Some(commands) = take(&mut self.child_commands) else {
            return;
        };
        for command in commands  {
            command.execute(world);
        }
    }
}

#[derive(Debug)]
pub struct NodeSetData(pub NodeData);
impl NodeSetData {
    pub fn new(data: NodeData) -> Box<Self> {
        Box::new(Self(data))
    }
}
impl NodeSubCommand for NodeSetData {
    fn execute(self: Box<Self>, id: NodeId, world: &mut World) {
        world.nodedata_write(id, self.0);
    }
}

#[derive(Debug)]
pub struct NodeAddExtension<T: Extension>(pub T);
impl<T: Extension> NodeAddExtension<T> {
    pub fn new(ext: T) -> Box<Self> {
        Box::new(NodeAddExtension(ext))
    }
}
impl<T: Extension> NodeSubCommand for NodeAddExtension<T> {
    fn execute(self: Box<Self>, id: NodeId, world: &mut World) {
        world.add_extension(id, self.0);
    }
}
