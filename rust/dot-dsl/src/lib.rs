use std::collections::HashMap;

macro_rules! impl_attrs {
    () => {
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs
                .iter()
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .collect();
            self
        }
        pub fn get_attr(&self, key: &str) -> Option<&str> {
            self.attrs.get(key).map(|s| s.as_str())
        }
    };
}

#[derive(PartialEq, Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
        self.nodes = nodes.to_vec();
        self
    }
    pub fn with_edges(mut self, edges: &[Edge]) -> Self {
        self.edges = edges.to_vec();
        self
    }
    pub fn get_node(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|node| node.name == name)
    }

    impl_attrs!();
}

#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub attrs: HashMap<String, String>,
}
impl Edge {
    pub fn new(from: &str, to: &str) -> Self {
        Edge {
            from: from.to_string(),
            to: to.to_string(),
            attrs: HashMap::new(),
        }
    }
    impl_attrs!();
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub name: String,
    pub attrs: HashMap<String, String>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: name.to_string(),
            attrs: HashMap::new(),
        }
    }
    impl_attrs!();
}

pub mod graph {
    pub use super::Graph;
    pub mod graph_items {
        pub mod edge {
            pub use super::super::super::Edge;
        }
        pub mod node {
            pub use super::super::super::Node;
        }
    }
}
