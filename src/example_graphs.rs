use phf::phf_map;
use crate::graph::Graph;

static mut EXAMPLE_GRAPHS: phf::Map<char, Graph> = phf_map!(
    'A' => Graph {nodes: vec![],edges: vec![]},
    'B'=> Graph {nodes: vec![],edges: vec![]}
);