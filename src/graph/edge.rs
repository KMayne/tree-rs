use uuid::Uuid;

#[derive(Copy, Clone)]
pub enum EdgeType {
    Undirected,
    Directional,
    Bidirectional,
}

pub struct Edge {
    pub(crate) id: Uuid,
    pub(crate) from_node: Uuid,
    pub(crate) to_node: Uuid,
    pub(crate) edge_type: EdgeType,
}
