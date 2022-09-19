use druid::Point;
use uuid::Uuid;
use crate::graph::edge::{Edge, EdgeType};

pub(crate) struct DisplayEdge {
    pub(crate) id: Uuid,
    pub(crate) from_node: Uuid,
    pub(crate) to_node: Uuid,
    pub(crate) edge_type: EdgeType,
    pub(crate) start_point: Point,
    pub(crate) end_point: Point,
    pub(crate) selected: bool
}
impl DisplayEdge {
    pub fn new(edge: &Edge, start_point: Point, end_point: Point) -> Self {
        DisplayEdge {
            id: edge.id,
            from_node: edge.from_node,
            to_node: edge.to_node,
            edge_type: edge.edge_type,
            start_point,
            end_point,
            selected: false
        }
    }
}