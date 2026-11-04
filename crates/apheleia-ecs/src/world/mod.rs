pub mod commands;
pub mod extensions;
pub mod node;
pub mod nodedata;
pub mod resource;
pub mod systems;
pub mod local_events;

use std::{cell::RefCell, rc::Rc};

use apheleia_core::buffer::Buffer;
use apheleia_types::id_gen::IdGenerator;
use tree_ds::prelude::{Node, Tree};

use crate::{
    stores::{
        extensions::ExtensionStore, local_events::LocalEventRegistry, nodedata::NodeDataStore, resource::ResourceStore, systems::SystemStore
    },
    types::NodeId,
    world_commands::WorldCommand,
};

pub struct World {
    pub is_running: bool,
    pub terminal_size: (u16, u16),
    pub buffer: Rc<RefCell<Buffer>>,

    nodeid_gen: IdGenerator<NodeId>,
    registered_nodeids: Vec<NodeId>,
    pub relations: Tree<NodeId, NodeId>,

    nodedata: NodeDataStore,
    resource_store: ResourceStore,
    extension_store: ExtensionStore,
    system_store: SystemStore,

    local_events: LocalEventRegistry,

    world_commands: Vec<Box<dyn WorldCommand>>,
}
impl World {
    pub fn new(terminal_size: (u16, u16)) -> Self {
        let mut relations = Tree::<NodeId, NodeId>::new(None);
        relations.add_node(Node::new(0, None), None).ok();

        Self {
            is_running: true,

            terminal_size,
            buffer: Rc::new(RefCell::new(Buffer::new(terminal_size))),

            nodeid_gen: IdGenerator::new(1), // 0 is reserved for terminal 'root'
            registered_nodeids: Default::default(),
            relations,

            nodedata: Default::default(),
            resource_store: Default::default(),
            extension_store: Default::default(),
            system_store: Default::default(),

            local_events: Default::default(),

            world_commands: Default::default(),
        }
    }
}
