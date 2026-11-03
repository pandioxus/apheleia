use std::{thread::sleep, time::Duration};

use apheleia_core::{buffer::Buffer, terminal::Terminal};
use apheleia_ecs::world::{self, World};

pub struct Apheleia {
    world: World,
    terminal: Terminal,

    buffer: Buffer
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

            buffer: Buffer::new(size)
        }
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

    fn render_flip(&mut self) {
        self.world.system_run(apheleia_ecs::stores::systems::SystemStage::PreRender);
        self.world.system_run(apheleia_ecs::stores::systems::SystemStage::Render);
        self.world.system_run(apheleia_ecs::stores::systems::SystemStage::PostRender);

        self.terminal.render_clear(&mut self.buffer);
    }

    fn render_update(&mut self) {
        self.world.system_run(apheleia_ecs::stores::systems::SystemStage::PreRender);
        self.world.system_run(apheleia_ecs::stores::systems::SystemStage::Render);
        self.world.system_run(apheleia_ecs::stores::systems::SystemStage::PostRender);

        self.terminal.render_update(&mut self.buffer);
    }

    pub fn run(&mut self) {
        self.render_flip();
        while true {
            self.event();
            self.update();
            self.render_update();

            sleep(Duration::from_millis(16)); // TODO: Temporary. Replace with delta time tracking
                                              // with solid 60fps
        }
    }
}
