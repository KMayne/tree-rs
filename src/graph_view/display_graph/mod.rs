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
    node_edges: HashMap<NodeId, Vec<EdgeId>>,
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

    pub(crate) fn get_node(&self, node_id: &NodeId) -> Option<&DisplayNode> {
        self.nodes.get(node_id)
    }
    pub(crate) fn get_mut_node(&mut self, node_id: &NodeId) -> Option<&mut DisplayNode> {
        self.nodes.get_mut(node_id)
    }
    pub(crate) fn translate_node(&mut self, node_id: &NodeId, translation: Vec2) {
        // Update node with translation & reinsert it
        // TODO: Is there a nicer way to phrase this?
        let new_node_center = {
            let target_node = self.get_mut_node(node_id).unwrap();
            target_node.rect = target_node.rect.with_origin(target_node.rect.origin() + translation);
            let new_node_center = (&target_node).rect.center().clone();
            let node_region_ref = RegionRef::from(&*target_node);
            self.rtree.insert(node_region_ref);
            new_node_center
        };

        // TODO: Surely there's a better way to do this
        const EMPTY_EDGE_LIST: &Vec<EdgeId> = &vec![];
        // TODO: This is absurd I should find a way to avoid having to copy this vec
        let affected_edge_ids = self.node_edges.get(node_id).unwrap_or(EMPTY_EDGE_LIST).clone();
        // Remove affected node and connected edges from the R-Tree
        self.rtree.remove(&RegionRef::from(self.get_node(node_id).unwrap()));
        for edge_id in &affected_edge_ids {
            self.rtree.remove(&RegionRef::from(self.edges.get(&edge_id).unwrap()));
        }
        // Update the edges and reinsert them into the R-Tree
        for edge_id in &affected_edge_ids {
            let moved_edge = self.edges.get_mut(&edge_id).unwrap();
            if &moved_edge.from_node == node_id {
                moved_edge.start_point = new_node_center.clone();
            } else {
                moved_edge.end_point = new_node_center.clone();
            }
            self.rtree.insert(RegionRef::from(&*moved_edge));
        }
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

        let mut node_edges: HashMap<NodeId, Vec<EdgeId>> = HashMap::new();
        for edge in &g.edges {
            node_edges.entry(edge.from_node_id).and_modify(|vec| vec.push(edge.id)).or_insert(vec![edge.id]);
            node_edges.entry(edge.to_node_id).and_modify(|vec| vec.push(edge.id)).or_insert(vec![edge.id]);
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
