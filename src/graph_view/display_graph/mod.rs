use std::collections::HashMap;

use druid::{Point, Vec2};
use rstar::{AABB, PointDistance, RTree, RTreeObject};
use rstar::primitives::Rectangle;
use uuid::Uuid;

use crate::graph::edge::Edge;
use crate::graph::Graph;
use crate::graph::node::Node;
use crate::graph_view::display_graph::edge::DisplayEdge;
use crate::graph_view::display_graph::node::DisplayNode;

pub(crate) mod node;
pub(crate) mod edge;

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

impl From<&DisplayNode> for RegionRef {
    fn from(n: &DisplayNode) -> Self {
        RegionRef {
            id: n.id,
            region: AABB::from_corners((n.rect.x0, n.rect.y0), (n.rect.x1, n.rect.y1)),
            region_type: RegionType::Node,
        }
    }
}

impl From<&DisplayEdge> for RegionRef {
    fn from(e: &DisplayEdge) -> Self {
        RegionRef {
            id: e.id,
            region: AABB::from_corners(RPoint::from(e.start_point), RPoint::from(e.end_point)),
            region_type: RegionType::Node,
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
    nodes: HashMap<Uuid, DisplayNode>,
    edges: HashMap<Uuid, DisplayEdge>,
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
        let display_node = DisplayNode::from(&node);
        self.rtree.insert(RegionRef::from(&display_node));
        self.nodes.insert(display_node.id, display_node);
    }

    pub(crate) fn nodes(&self) -> Vec<&DisplayNode> { self.nodes.values().collect() }

    pub(crate) fn get_node(&self, id: &Uuid) -> Option<&DisplayNode> { self.nodes.get(id) }
    pub(crate) fn get_mut_node(&mut self, id: &Uuid) -> Option<&mut DisplayNode> { self.nodes.get_mut(id) }
    pub(crate) fn translate_node(&mut self, id: &Uuid, translation: Vec2) {
        let node_center = self.get_node_center(id);
        self.rtree.remove_at_point(&(node_center.x, node_center.y));

        let target_node = self.get_mut_node(id).unwrap();
        target_node.rect = target_node.rect.with_origin(target_node.rect.origin() + translation);
        let region_ref = RegionRef::from(&*target_node);
        self.rtree.insert(region_ref);
    }

    pub(crate) fn add_edge(&mut self, edge: Edge) {
        let display_edge = DisplayEdge::new(&edge, self.get_node_center(&edge.from_node),
                                            self.get_node_center(&edge.to_node));
        self.rtree.insert(RegionRef::from(&display_edge));
        self.edges.insert(display_edge.id, display_edge);
    }

    pub(crate) fn edges(&self) -> Vec<&DisplayEdge> {
        self.edges.values().collect()
    }

    fn get_node_center(&self, node_id: &Uuid) -> Point {
        self.nodes.get(node_id).unwrap().rect.center()
    }
}

impl From<&Graph> for DisplayGraph {
    fn from(g: &Graph) -> Self {
        let display_nodes: Vec<DisplayNode> = g.nodes.iter().map(DisplayNode::from).collect();
        let mut region_refs: Vec<RegionRef> = (&display_nodes).iter().map(RegionRef::from).collect();

        let node_map: HashMap<Uuid, DisplayNode> =
            display_nodes.into_iter().map(|n| (n.id, n)).collect();

        let display_edges: Vec<DisplayEdge> = g.edges.iter().map(|e|
            DisplayEdge::new(e, node_map.get(&e.from_node).unwrap().rect.center(),
                             node_map.get(&e.to_node).unwrap().rect.center())).collect();
        region_refs.append(&mut (display_edges.iter().map(RegionRef::from).collect()));
        DisplayGraph {
            rtree: RTree::bulk_load(region_refs),
            nodes: node_map,
            edges: display_edges.into_iter().map(|e| (e.id, e)).collect(),
        }
    }
}
