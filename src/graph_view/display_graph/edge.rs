use druid::Point;

use crate::graph::edge::{Edge, EdgeId};
use crate::graph::node::NodeId;

pub(crate) struct DisplayEdge {
    pub(crate) id: EdgeId,
    pub(crate) from_node: NodeId,
    pub(crate) to_node: NodeId,
    pub(crate) start_point: Point,
    pub(crate) end_point: Point,
}
impl DisplayEdge {
    pub fn new(edge: &Edge, start_point: Point, end_point: Point) -> Self {
        DisplayEdge {
            id: edge.id,
            from_node: edge.from_node_id,
            to_node: edge.to_node_id,
            start_point,
            end_point
        }
    }
}