use crate::graph::edge::EdgeId;
use crate::graph::node::NodeId;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub(crate) enum ElementId {
    Node(NodeId),
    Edge(EdgeId),
}