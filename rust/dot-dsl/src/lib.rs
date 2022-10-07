pub mod graph {
    use std::collections::HashMap;
    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;

    type AttrMap = HashMap<String, String>;

    ///-------------------------------------------------------------------------------
    /// GRAPH
    ///-------------------------------------------------------------------------------
    #[derive(Default)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: AttrMap,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
            self.nodes = nodes.to_vec();
            self
        }

        pub fn with_edges(mut self, edges: &[Edge]) -> Self {
            self.edges = edges.to_vec();
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            self.attrs = attrs.iter().map(|(s1, s2)| (s1.to_string(), s2.to_string())).collect();
            self
        }

        pub fn node(&self, node_val: &str) -> Option<&Node> {
            for node in &self.nodes {
                if node.val == node_val {
                    return Some(node)
                }
            };
            None
        }
    }

    pub mod graph_items {
        ///---------------------------------------------------------------------------
        /// NODE
        ///---------------------------------------------------------------------------
        pub mod node {
            use std::collections::HashMap;
            use crate::graph::AttrMap;
            #[derive(Clone, Debug, PartialEq, Eq)]
            pub struct Node {
                pub val: String,
                attrs: AttrMap,
            }
            impl Node {
                pub fn new(val: &str) -> Self {
                    Node {
                        val: val.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs.iter().map(|(s1, s2)| (s1.to_string(), s2.to_string())).collect();
                    self
                }

                pub fn attr(&self, attr_name: &str) -> Option<&str> {
                    self.attrs.get(attr_name).map(|v| v.as_str())
                }
            }
        }
        ///---------------------------------------------------------------------------
        /// EDGE
        ///---------------------------------------------------------------------------
        pub mod edge {
            use std::collections::HashMap;
            use crate::graph::AttrMap;
            #[derive(Default, Debug, Clone, PartialEq, Eq)]
            pub struct Edge {
                from: String,
                to: String,
                attrs: AttrMap
            }
            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Edge {
                        attrs: HashMap::new(),
                        from: from.to_string(),
                        to: to.to_string(),
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs.iter().map(|(s1, s2)| (s1.to_string(), s2.to_string())).collect();
                    self
                }

                pub fn attr(&self, attr_name: &str) -> Option<&str> {
                    self.attrs.get(attr_name).map(|v| v.as_str())
                }
            }
        }
    }
}
