use crate::graph::edge::Edge;
use crate::graph::node::Node;

pub mod edge;
pub mod node;

#[derive(Default)]
pub(crate) struct Graph {
    pub(crate) nodes: Vec<Node>,
    pub(crate) edges: Vec<Edge>,
}
