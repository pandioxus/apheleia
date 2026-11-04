use std::{io, thread::sleep, time::Duration};

use apheleia_core::{terminal::Terminal};
use apheleia_ecs::{stores::systems::function_system::IntoSystem, types::{NodeId, SystemStage, traits::Resource}, world::World};

use crate::builder::{self, NodeBuilder};

pub struct Apheleia {
    world: World,
    terminal: Terminal,
}
impl Apheleia {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut term = Terminal::new();
        term.init().expect("Couldn't initialize terminal");

        let size = term.size().expect("Couuldn't get terminal size");

        Self {
            world: World::new(size),
            terminal: term,
        }
    }

    pub fn get_world(&self) -> &World {
        &self.world
    }
    pub fn get_world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn create_child(mut self, f: impl FnOnce(NodeBuilder) -> NodeBuilder) -> Self {
        let mut commands = f(NodeBuilder::new(0, &mut self)).build();
        self.world.append_commands(&mut commands);
        self
    }

    pub fn insert_resource<T: Resource>(mut self, resource: T) -> Self {
        self.world.resource_insert(resource);
        self
    }

    pub fn add_system<Params: 'static>(mut self, stage: SystemStage, system: impl IntoSystem<Params>) -> Self {
        self.world.system_register(stage, system);
        self
    }

    fn event(&mut self) {
        while let Ok(app_event) = self.terminal.events().try_recv() {
            match app_event {
                apheleia_core::events::CrosstermEvent::Event(event) => todo!(),
                apheleia_core::events::CrosstermEvent::Tick => todo!(),
            }
        }
    }

    fn update(&mut self) {
        self.world.system_run(SystemStage::PreUpdate);
        self.world.system_run(SystemStage::Update);
        self.world.system_run(SystemStage::PostUpdate);
    }

    fn render_flip(&mut self) -> io::Result<()> {
        self.world.system_run(SystemStage::PreRender);
        self.world.system_run(SystemStage::Render);
        self.world.system_run(SystemStage::PostRender);

        self.terminal.render_clear(&mut self.world.buffer.borrow_mut())?;

        Ok(())
    }

    fn render_update(&mut self) -> io::Result<()> {
        self.world.system_run(SystemStage::PreRender);
        self.world.system_run(SystemStage::Render);
        self.world.system_run(SystemStage::PostRender);

        self.terminal.render_update(&mut self.world.buffer.borrow_mut())?;

        Ok(())
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.world.execute_commands();

        self.render_flip()?;
        while self.world.is_running {
            self.event();
            self.update();
            self.render_update()?;

            sleep(Duration::from_millis(16)); // TODO: Temporary. Replace with delta time tracking
                                              // with solid 60fps
        }

        Ok(())
    }
}
