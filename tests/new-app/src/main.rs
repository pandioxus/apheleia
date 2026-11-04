use std::io;

use apheleia_app::app::Apheleia;
use apheleia_ecs::{
    canvas::Canvas,
    system_params::query::Query,
    types::{NodeId, SystemStage},
};
use apheleia_types::size::Size;

fn render_test(query: Query<(NodeId, Canvas)>) {
    for (id, mut canvas) in query.iter() {
        canvas.write_text(format!("hello {}", id).as_str(), (0, 0), 0, None);
    }
}

fn main() -> io::Result<()> {
    Apheleia::new()
        .add_system(SystemStage::Render, render_test)
        .create_child(|builder| builder.set_size(Size { w: 10, h: 10 }))
        .run()
}
