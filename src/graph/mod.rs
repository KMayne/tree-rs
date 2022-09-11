use druid_shell::kurbo::Point;
use crate::graph::edge::Edge;
use crate::graph::node::Node;

mod edge;
mod node;

#[derive(Default)]
pub(crate) struct Graph {
    pub(crate) nodes: Vec<Box<Node>>,
    pub(crate) edges: Vec<Box<Edge>>,
}

impl Graph {
    pub fn add_empty_node(&mut self, center: Point) {
        self.nodes.push(Box::new(Node::new(center, None)))
    }
}