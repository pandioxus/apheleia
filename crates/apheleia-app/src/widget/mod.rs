use apheleia_ecs::world_commands::WorldCommand;

pub trait Widget {
    fn build_commands(self) -> Vec<Box<dyn WorldCommand>>;
}
