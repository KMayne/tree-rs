use druid::{Point, Rect, Size};
use uuid::Uuid;
use crate::graph::edge::{Edge, EdgeType};
use crate::graph::Graph;
use crate::graph::node::Node;
use crate::graph_view::display_graph::DisplayGraph;

pub(crate) fn arborealis_graph() -> DisplayGraph {
    let sector_9_id = Uuid::new_v4();
    let sapling_id = Uuid::new_v4();
    let tree_rs_id = Uuid::new_v4();
    let seed_rs_id = Uuid::new_v4();
    let leaf_id = Uuid::new_v4();
    let root_id = Uuid::new_v4();
    let arboretum_id = Uuid::new_v4();
    let automerge_id = Uuid::new_v4();
    DisplayGraph::from(&Graph {
        nodes: vec![
            Node {
                id: Uuid::new_v4(),
                text: String::from("ARBOREALIS"),
                rect: Rect::from_origin_size(Point::new(896.7, 170.2), Size::new(197.0, 75.0)),
            },
            Node {
                id: sector_9_id,
                text: String::from("sector9"),
                rect: Rect::from_origin_size(Point::new(1327.7, 171.5), Size::new(132.0, 66.0)),
            },
            Node {
                id: sapling_id,
                text: String::from("sapling (based on druid)"),
                rect: Rect::from_origin_size(Point::new(1592.5, 338.5), Size::new(179.0, 100.0)),
            },
            Node {
                id: tree_rs_id,
                text: String::from("tree-rs"),
                rect: Rect::from_origin_size(Point::new(1288.0, 340.4), Size::new(217.0, 58.0)),
            },
            Node {
                id: Uuid::new_v4(),
                text: String::from("tree-js (abandon?)"),
                rect: Rect::from_origin_size(Point::new(927.5, 348.9), Size::new(237.0, 55.0)),
            },
            Node {
                id: seed_rs_id,
                text: String::from("seed-rs"),
                rect: Rect::from_origin_size(Point::new(1269.2, 462.9), Size::new(231.0, 58.0)),
            },
            Node {
                id: leaf_id,
                text: String::from("leaf"),
                rect: Rect::from_origin_size(Point::new(867.0, 466.9), Size::new(126.0, 160.0)),
            },
            Node {
                id: root_id,
                text: String::from("root"),
                rect: Rect::from_origin_size(Point::new(1080.0, 485.7), Size::new(100.0, 60.0)),
            },
            Node {
                id: arboretum_id,
                text: String::from("arboretum"),
                rect: Rect::from_origin_size(Point::new(1595.6, 580.4), Size::new(168.0, 58.0)),
            },
            Node {
                id: automerge_id,
                text: String::from("automerge"),
                rect: Rect::from_origin_size(Point::new(1190.0, 707.9), Size::new(176.0, 79.0)),
            },
        ],
        edges: vec![
            Edge {
                id: Uuid::new_v4(),
                from_node: leaf_id,
                to_node: root_id,
                edge_type: EdgeType::Bidirectional
            },
            Edge {
                id: Uuid::new_v4(),
                from_node: tree_rs_id,
                to_node: sapling_id,
                edge_type: EdgeType::Directional
            },
            Edge {
                id: Uuid::new_v4(),
                from_node: arboretum_id,
                to_node: sapling_id,
                edge_type: EdgeType::Directional
            },
            Edge {
                id: Uuid::new_v4(),
                from_node: sector_9_id,
                to_node: tree_rs_id,
                edge_type: EdgeType::Directional
            },
            Edge {
                id: Uuid::new_v4(),
                from_node: tree_rs_id,
                to_node: seed_rs_id,
                edge_type: EdgeType::Directional
            },
            Edge {
                id: Uuid::new_v4(),
                from_node: seed_rs_id,
                to_node: root_id,
                edge_type: EdgeType::Bidirectional
            },
            Edge {
                id: Uuid::new_v4(),
                from_node: arboretum_id,
                to_node: seed_rs_id,
                edge_type: EdgeType::Directional
            },
            Edge {
                id: Uuid::new_v4(),
                from_node: seed_rs_id,
                to_node: automerge_id,
                edge_type: EdgeType::Directional
            },
            Edge {
                id: Uuid::new_v4(),
                from_node: root_id,
                to_node: automerge_id,
                edge_type: EdgeType::Directional
            },
            Edge {
                id: Uuid::new_v4(),
                from_node: leaf_id,
                to_node: automerge_id,
                edge_type: EdgeType::Directional
            }
        ]
    })
}