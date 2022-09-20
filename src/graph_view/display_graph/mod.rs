use std::collections::HashMap;

use druid::{Point, Vec2};
use rstar::{AABB, PointDistance, RTree, RTreeObject};
use rstar::primitives::Rectangle;

use crate::graph::edge::{Edge, EdgeId};
use crate::graph::Graph;
use crate::graph::node::{Node, NodeId};
use crate::graph_view::display_graph::edge::DisplayEdge;
use crate::graph_view::display_graph::node::DisplayNode;
use crate::graph_view::element_id::ElementId;

pub(crate) mod node;
pub(crate) mod edge;

type RPoint = (f64, f64);

#[derive(Eq, PartialEq)]
enum RegionType {
    Node,
    Edge,
}

struct RegionRef {
    id: ElementId,
    region: AABB<RPoint>,
}

impl PartialEq<Self> for RegionRef {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl From<&DisplayNode> for RegionRef {
    fn from(n: &DisplayNode) -> Self {
        RegionRef {
            id: ElementId::Node(n.id),
            region: AABB::from_corners((n.rect.x0, n.rect.y0), (n.rect.x1, n.rect.y1)),
        }
    }
}

impl From<&DisplayEdge> for RegionRef {
    fn from(e: &DisplayEdge) -> Self {
        RegionRef {
            id: ElementId::Edge(e.id),
            region: AABB::from_corners(RPoint::from(e.start_point), RPoint::from(e.end_point)),
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
    fn distance_2(&self, point: &RPoint) -> f64 {
        Rectangle::from(self.envelope()).distance_2(point)
    }

    fn contains_point(&self, point: &RPoint) -> bool {
        Rectangle::from(self.envelope()).contains_point(point)
    }

    fn distance_2_if_less_or_equal(&self, point: &RPoint, max_distance_2: f64) -> Option<f64> {
        Rectangle::from(self.envelope()).distance_2_if_less_or_equal(point, max_distance_2)
    }
}

#[derive(Default)]
pub struct DisplayGraph {
    rtree: RTree<RegionRef>,
    nodes: HashMap<NodeId, DisplayNode>,
    edges: HashMap<EdgeId, DisplayEdge>,
    node_edges: HashMap<NodeId, Vec<Edge>>,
}

impl DisplayGraph {
    pub(crate) fn get_mut_node_at_point(&mut self, p: RPoint) -> Option<&mut DisplayNode> {
        let node_id = self.rtree.locate_all_at_point(&p).filter_map(|r|
            match r.id {
                ElementId::Node(node_id) => Some(node_id),
                _ => None
            }).next();
        if let Some(node_id) = node_id {
            self.nodes.get_mut(&node_id)
        } else {
            None
        }
    }

    pub(crate) fn add_node(&mut self, node: Node) {
        let display_node = DisplayNode::from(&node);
        self.rtree.insert(RegionRef::from(&display_node));
        self.nodes.insert(display_node.id, display_node);
    }

    pub(crate) fn nodes(&self) -> Vec<&DisplayNode> { self.nodes.values().collect() }

    pub(crate) fn get_node(&self, id: &NodeId) -> Option<&DisplayNode> { self.nodes.get(id) }
    pub(crate) fn get_mut_node(&mut self, id: &NodeId) -> Option<&mut DisplayNode> { self.nodes.get_mut(id) }
    pub(crate) fn translate_node(&mut self, id: &NodeId, translation: Vec2) {
        self.rtree.remove(&RegionRef::from(self.get_node(id).unwrap()));
        let target_node = self.get_mut_node(id).unwrap();
        target_node.rect = target_node.rect.with_origin(target_node.rect.origin() + translation);
        let region_ref = RegionRef::from(&*target_node);
        self.rtree.insert(region_ref);
    }

    pub(crate) fn add_edge(&mut self, edge: Edge) {
        let display_edge = DisplayEdge::new(&edge, self.get_node_center(&edge.from_node_id),
                                            self.get_node_center(&edge.to_node_id));
        self.rtree.insert(RegionRef::from(&display_edge));
        self.edges.insert(display_edge.id, display_edge);
    }

    pub(crate) fn edges(&self) -> Vec<&DisplayEdge> {
        self.edges.values().collect()
    }

    fn get_node_center(&self, node_id: &NodeId) -> Point {
        self.nodes.get(node_id).unwrap().rect.center()
    }
}

impl From<&Graph> for DisplayGraph {
    fn from(g: &Graph) -> Self {
        let display_nodes: Vec<DisplayNode> = g.nodes.iter().map(DisplayNode::from).collect();
        let mut region_refs: Vec<RegionRef> = (&display_nodes).iter().map(RegionRef::from).collect();

        let node_map: HashMap<NodeId, DisplayNode> =
            display_nodes.into_iter().map(|n| (n.id, n)).collect();

        let display_edges: Vec<DisplayEdge> = g.edges.iter().map(|e|
            DisplayEdge::new(e, node_map.get(&e.from_node_id).unwrap().rect.center(),
                             node_map.get(&e.to_node_id).unwrap().rect.center())).collect();

        let mut node_edges: HashMap<NodeId, Vec<Edge>> = HashMap::new();
        for edge in &g.edges {
            if let Some(node_edge) = node_edges.get_mut(&edge.from_node_id) {
                node_edge.push(edge.clone())
            } else {
                node_edges.insert(edge.from_node_id, vec![edge.clone()]);
            }
            if let Some(node_edge) = node_edges.get_mut(&edge.to_node_id) {
                node_edge.push(edge.clone())
            } else {
                node_edges.insert(edge.to_node_id, vec![edge.clone()]);
            }
        }
        region_refs.append(&mut (display_edges.iter().map(RegionRef::from).collect()));
        DisplayGraph {
            rtree: RTree::bulk_load(region_refs),
            nodes: node_map,
            edges: display_edges.into_iter().map(|e| (e.id, e)).collect(),
            node_edges,
        }
    }
}
