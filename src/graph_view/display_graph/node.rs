use druid::Rect;
use uuid::Uuid;
use crate::graph::node::Node;

#[derive(Debug)]
pub(crate) struct DisplayNode {
    pub id: Uuid,
    pub text: String,
    pub rect: Rect,
    pub selected: bool,
}

impl From<&Node> for DisplayNode {
    fn from(node: &Node) -> Self {
        DisplayNode {
            id: node.id,
            text: node.text.clone(),
            rect: node.rect,
            selected: false,
        }
    }
}