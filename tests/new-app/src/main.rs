use std::io;

use apheleia_app::app::Apheleia;
use apheleia_core::style::{Style, color::Color};
use apheleia_ecs::{
    canvas::Canvas,
    system_params::query::Query,
    types::{NodeId, SystemStage},
};
use apheleia_types::size::Size;

fn render_test(query: Query<(NodeId, Canvas)>) {
    for (id, mut canvas) in query.iter() {
        canvas.write_text(
            format!("hello {}", id).as_str(),
            (0, 0),
            0,
            Some(Style {
                bg: Color::Rgba {
                    r: 200,
                    g: 0,
                    b: 0,
                    a: 255,
                },
                ..Default::default()
            }),
        );
    }
}
fn another_render_test(query: Query<(NodeId, Canvas)>) {
    for (id, mut canvas) in query.iter() {
        canvas.write_text(
            format!("WEE", id).as_str(),
            (0, 0),
            1,
            Some(Style {
                bg: Color::Rgba { r: 0, g: 155, b: 10, a: 100 },
                ..Default::default()
            }),
        );
    }
}

fn main() -> io::Result<()> {
    Apheleia::new()
        .create_child(|builder| builder.set_size(Size { w: 10, h: 10 }))
        .add_system(SystemStage::Render, render_test)
        .add_system(SystemStage::Render, another_render_test)
        .run()
}
