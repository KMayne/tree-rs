use std::collections::HashMap;

use rstar::{AABB, PointDistance, RTree, RTreeObject};
use rstar::primitives::Rectangle;
use uuid::Uuid;

use crate::graph::edge::Edge;
use crate::graph::Graph;
use crate::graph::node::Node;
use crate::graph_view::display_graph::node::DisplayNode;

pub(crate) mod node;

type RPoint = (f64, f64);

enum RegionType {
    Node,
    Edge,
}

struct RegionRef {
    id: Uuid,
    region: AABB<RPoint>,
    region_type: RegionType,
}

impl From<&Node> for RegionRef {
    fn from(n: &Node) -> Self {
        RegionRef {
            id: n.id,
            region:  AABB::from_corners((n.rect.x0, n.rect.y0), (n.rect.x1, n.rect.y1)),
            region_type: RegionType::Node
        }
    }
}

impl RTreeObject for RegionRef {
    type Envelope = AABB<RPoint>;

    fn envelope(&self) -> Self::Envelope {
        self.region
    }
}

impl PointDistance for RegionRef {
    fn distance_2(&self, point: &RPoint) -> f64 { Rectangle::from(self.envelope()).distance_2(point) }

    fn contains_point(&self, point: &RPoint) -> bool { Rectangle::from(self.envelope()).contains_point(point) }

    fn distance_2_if_less_or_equal(&self, point: &RPoint, max_distance_2: f64) -> Option<f64> {
        Rectangle::from(self.envelope()).distance_2_if_less_or_equal(point, max_distance_2)
    }
}

#[derive(Default)]
pub struct DisplayGraph {
    rtree: RTree<RegionRef>,
    nodes: HashMap<Uuid, DisplayNode>,
    edges: HashMap<Uuid, Edge>,
}

impl DisplayGraph {
    pub(crate) fn get_mut_node_at_point(&mut self, p: RPoint) -> Option<&mut DisplayNode> {
        let node_id = self.rtree.locate_all_at_point(&p).filter_map(|r|
            match r.region_type {
                RegionType::Node => Some(r.id),
                _ => None
            }).next();
        if let Some(node_id) = node_id {
            self.nodes.get_mut(&node_id)
        } else {
            None
        }
    }

    pub(crate) fn add_node(&mut self, node: Node) {
        self.rtree.insert(RegionRef::from(&node));
        self.nodes.insert(node.id, DisplayNode::from(node));
    }

    pub(crate) fn nodes(&self) -> Vec<&DisplayNode> {
        self.nodes.values().collect()
    }
}

impl From<Graph> for DisplayGraph {
    fn from(g: Graph) -> Self {
        DisplayGraph {
            rtree: RTree::bulk_load((&g.nodes).into_iter().map(|n| RegionRef::from(n)).collect()),
            nodes: g.nodes.into_iter().map(|n| (n.id, DisplayNode::from(n))).collect(),
            edges: g.edges.into_iter().map(|e| (e.id, e)).collect(),
        }
    }
}
