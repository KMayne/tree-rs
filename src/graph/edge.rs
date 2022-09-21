use uuid::Uuid;
use crate::graph::node::NodeId;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct EdgeId(pub Uuid);

#[derive(Copy, Clone)]
pub enum EdgeType {
    Undirected,
    Directional,
    Bidirectional,
}

#[derive(Clone)]
pub struct Edge {
    pub(crate) id: EdgeId,
    pub(crate) from_node_id: NodeId,
    pub(crate) to_node_id: NodeId,
    pub(crate) edge_type: EdgeType,
}
