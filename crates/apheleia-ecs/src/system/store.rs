use std::any::TypeId;

use crate::{system::definitions::{IntoSystem, System}, types::SystemStage, world::World};

pub(crate) struct SystemStore {
    systems: Vec<Vec<Box<dyn System>>>,
    registered_systems: Vec<TypeId>,
}
impl Default for SystemStore {
    fn default() -> Self {
        let systems = vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![]];

        Self {
            systems,
            registered_systems: Default::default(),
        }
    }
}
impl SystemStore {
    /// Register the given function the store and return the [`SystemId`]
    pub fn add_system<Params: 'static>(
        &mut self,
        stage: SystemStage,
        system: impl IntoSystem<Params>,
    ) {
        let system = system.into_system();
        let system_typeid = system.id();

        if self.registered_systems.contains(&system_typeid) {
            return;
        }

        self.registered_systems.push(system_typeid);

        let stage = &mut self.systems[stage as usize];
        stage.push(system);
    }

    pub fn add_system_boxed(&mut self, stage: SystemStage, system: Box<dyn System>) {
        let system_typeid = system.id();

        if self.registered_systems.contains(&system_typeid) {
            return;
        }
        self.registered_systems.push(system_typeid);

        let stage = &mut self.systems[stage as usize];
        stage.push(system);
    }

    pub fn run_stage(&mut self, stage: SystemStage, world: *mut World) {
        let systems = &mut self.systems[stage as usize];
        for system in systems {
            system.run(world);
        }
    }
}
