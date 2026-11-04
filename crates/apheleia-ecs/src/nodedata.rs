use apheleia_types::{point::Point, size::Size};

#[derive(Debug, Clone)]
pub struct NodeData {
    pub local_position: Point<i32>,
    pub global_position: Option<Point<i32>>,

    pub size: Size<u16>,
}
