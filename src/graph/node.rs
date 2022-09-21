use druid::kurbo::{Point, Rect, Size};
use uuid::Uuid;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct NodeId(pub Uuid);

pub struct Node {
    pub(crate) id: NodeId,
    pub(crate) text: String,
    pub(crate) rect: Rect,
}

impl Node {
    const DEFAULT_SIZE: Size = Size { width: 100f64, height: 60f64 };

    pub(crate) fn new(center: Point, size: Option<Size>) -> Self {
        Node {
            id: NodeId(Uuid::new_v4()),
            text: String::new(),
            rect: Rect::from_center_size(center, size.unwrap_or(Node::DEFAULT_SIZE)),
        }
    }
}
