use std::{io, thread::sleep, time::Duration};

use apheleia_core::{terminal::Terminal};
use apheleia_ecs::{types::{NodeId, traits::Resource}, world::World};

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

    pub fn create_node(&mut self) -> NodeId {
        self.world.node_create()
    }

    pub fn insert_resource<T: Resource>(mut self, resource: T) -> Self {
        self.world.resource_insert(resource);
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
        self.world.system_run(apheleia_ecs::stores::systems::SystemStage::PreUpdate);
        self.world.system_run(apheleia_ecs::stores::systems::SystemStage::Update);
        self.world.system_run(apheleia_ecs::stores::systems::SystemStage::PostUpdate);
    }

    fn render_flip(&mut self) -> io::Result<()> {
        self.world.system_run(apheleia_ecs::stores::systems::SystemStage::PreRender);
        self.world.system_run(apheleia_ecs::stores::systems::SystemStage::Render);
        self.world.system_run(apheleia_ecs::stores::systems::SystemStage::PostRender);

        self.terminal.render_clear(self.world.get_buffer_mut())?;

        Ok(())
    }

    fn render_update(&mut self) -> io::Result<()> {
        self.world.system_run(apheleia_ecs::stores::systems::SystemStage::PreRender);
        self.world.system_run(apheleia_ecs::stores::systems::SystemStage::Render);
        self.world.system_run(apheleia_ecs::stores::systems::SystemStage::PostRender);

        self.terminal.render_update(self.world.get_buffer_mut())?;

        Ok(())
    }

    pub fn run(&mut self) -> io::Result<()> {
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
