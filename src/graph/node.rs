use druid_shell::kurbo::{Point, Rect, Size};
use uuid::Uuid;

pub struct Node {
    pub(crate) id: Uuid,
    pub(crate) text: String,
    pub(crate) rect: Rect,
}

impl Node {
    const DEFAULT_SIZE: Size = Size { width: 100f64, height: 60f64 };

    pub(crate) fn new(center: Point, size: Option<Size>) -> Self {
        Node {
            id: Uuid::new_v4(),
            text: String::new(),
            rect: Rect::from_center_size(center, size.unwrap_or(Node::DEFAULT_SIZE)),
        }
    }
}
