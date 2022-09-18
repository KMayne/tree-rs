use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use rstar::{AABB, PointDistance, RTree, RTreeObject};
use uuid::Uuid;

use crate::graph::edge::Edge;
use crate::graph::Graph;
use crate::graph::node::Node;
use crate::graph_view::display_graph::node::DisplayNode;

pub(crate) mod node;

type RPoint = (f64, f64);

enum ElementRef {
    NodeRef(Rc<RefCell<DisplayNode>>),
    // EdgeRef(Rc<Edge>),
}

impl ElementRef {
    fn to_r_rect(&self) -> rstar::primitives::Rectangle<RPoint> {
        rstar::primitives::Rectangle::from(self.envelope())
    }
}

impl RTreeObject for ElementRef {
    type Envelope = AABB<RPoint>;

    fn envelope(&self) -> Self::Envelope {
        match self {
            ElementRef::NodeRef(n) => {
                let rect = (**n).borrow().rect;
                AABB::from_corners((rect.x0, rect.y0), (rect.x1, rect.y1))
            }
            // ElementRef::EdgeRef(e) => {}
        }
    }
}

impl PointDistance for ElementRef {
    fn distance_2(&self, point: &RPoint) -> f64 { self.to_r_rect().distance_2(point) }

    fn contains_point(&self, point: &RPoint) -> bool { self.to_r_rect().contains_point(point) }

    fn distance_2_if_less_or_equal(&self, point: &RPoint, max_distance_2: f64) -> Option<f64> {
        self.to_r_rect().distance_2_if_less_or_equal(point, max_distance_2)
    }
}

#[derive(Default)]
pub struct DisplayGraph {
    rtree: RTree<ElementRef>,
    nodes: HashMap<Uuid, Rc<RefCell<DisplayNode>>>,
    edges: HashMap<Uuid, Rc<Edge>>,
}

impl DisplayGraph {
    pub(crate) fn get_nodes_at_point(&self, p: RPoint) -> Vec<Rc<RefCell<DisplayNode>>> {
        self.rtree.locate_all_at_point(&p).filter_map(|r| match r
        {
            ElementRef::NodeRef(node) => Some(node.clone()),
            _ => None
        }).collect()
    }

    pub(crate) fn add_node(&mut self, node: Node) {
        let node_id = node.id;
        let node_rc = Rc::new(RefCell::new(DisplayNode::from(node)));
        self.nodes.insert(node_id, node_rc.clone());
        self.rtree.insert(ElementRef::NodeRef(node_rc));
    }

    pub(crate) fn nodes(&self) -> Vec<Rc<RefCell<DisplayNode>>> {
        self.nodes.values().map(|n| n.clone()).collect()
    }
}

impl From<Graph> for DisplayGraph {
    fn from(g: Graph) -> Self {
        let nodes: &Vec<Rc<RefCell<DisplayNode>>> = &g.nodes.into_iter().map(|n| Rc::new(RefCell::new(DisplayNode::from(n)))).collect();
        DisplayGraph {
            rtree: RTree::bulk_load(nodes.into_iter().map(|n| ElementRef::NodeRef(n.clone())).collect()),
            nodes: nodes.into_iter().map(|n| ((**n).borrow().id, n.clone())).collect(),
            edges: g.edges.into_iter().map(|e| (e.id, Rc::new(e))).collect(),
        }
    }
}
