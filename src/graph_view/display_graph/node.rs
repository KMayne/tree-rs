use druid::Rect;

use crate::graph::node::{Node, NodeId};

#[derive(Debug)]
pub(crate) struct DisplayNode {
    pub id: NodeId,
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