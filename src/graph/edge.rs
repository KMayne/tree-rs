use uuid::Uuid;

pub enum EdgeType {
    Undirected,
    Directional,
    Bidirectional,
}

pub struct Edge {
    from_node: Uuid,
    to_node: Uuid,
    edge_type: EdgeType,
}
