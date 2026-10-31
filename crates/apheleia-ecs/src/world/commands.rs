use std::mem::take;

use crate::{world_commands::WorldCommand, world::World};

impl World {
    #[inline]
    pub fn insert_command(&mut self, command: Box<dyn WorldCommand>) {
        self.world_commands.push(command);
    }

    #[inline]
    pub fn execute_commands(&mut self) {
        let commands = take(&mut self.world_commands);
        for command in commands {
            command.execute(self);
        }
    }
}
