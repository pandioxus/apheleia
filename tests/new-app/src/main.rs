use apheleia_app::{
    app::Apheleia,
    builder::{self, NodeBuilder},
};
use apheleia_types::{point::Point, size::Size};

fn main() {
    let mut app = Apheleia::new();

    let commands = NodeBuilder::new(0, &mut app)
        .create_child(|builder| builder.set_position(Point { x: 10, y: 5 }))
        .create_child(|builder| {
            builder
                .set_size(Size { w: 10, h: 10 })
                .create_child(|builder| builder.set_position(Point { x: 10, y: 5 }))
        })
        .build();
    println!("{:#?}", commands);
}
