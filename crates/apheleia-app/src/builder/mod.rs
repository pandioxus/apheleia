use apheleia_ecs::{
    nodedata::NodeData,
    types::{NodeId, traits::Extension},
    world_commands::{
        WorldCommand,
        node::{CreateNode, NodeAddExtension, NodeSetData, NodeSubCommand},
    },
};
use apheleia_types::{point::Point, size::Size};

use crate::app::{self, Apheleia};

pub struct NodeBuilder<'w> {
    app: &'w mut Apheleia,

    id: NodeId,
    parent_id: NodeId,
    nodedata: NodeData,
    node_subcommands: Vec<Box<dyn NodeSubCommand>>,
    child_commands: Vec<Box<dyn WorldCommand>>,
}
impl<'w> NodeBuilder<'w> {
    pub fn new(parent_id: NodeId, app: &'w mut Apheleia) -> NodeBuilder<'w> {
        let id = app.create_node();
        Self {
            app,

            id,
            parent_id,
            nodedata: NodeData {
                local_position: Point { x: 0, y: 0 },
                global_position: None,

                size: Size { w: 0, h: 0 },
            },
            node_subcommands: Default::default(),
            child_commands: Default::default(),
        }
    }

    pub fn set_position(mut self, position: Point<i32>) -> Self {
        self.nodedata.local_position = position;
        self
    }

    pub fn set_size(mut self, size: Size<u16>) -> Self {
        self.nodedata.size = size;
        self
    }

    pub fn add_extension<T: Extension>(mut self, extension: T) -> Self {
        self.node_subcommands.push(NodeAddExtension::new(extension));
        self
    }

    pub fn create_child(mut self, f: impl FnOnce(NodeBuilder) -> NodeBuilder) -> Self {
        let builder = f(NodeBuilder::new(self.id, self.app));
        self.child_commands.append(&mut builder.build());
        self
    }

    pub fn build(mut self) -> Vec<Box<dyn WorldCommand>> {
        self.node_subcommands.push(NodeSetData::new(self.nodedata));

        vec![CreateNode::new(
            self.parent_id,
            self.node_subcommands,
            Some(self.child_commands),
        )]
    }
}
