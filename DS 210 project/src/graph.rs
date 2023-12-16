extern crate petgraph;
use petgraph::graph::DiGraph;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;



pub struct FinancialGraph {
    pub graph: DiGraph<String, f64>,
}

impl FinancialGraph {
    pub fn new() -> Self {
        FinancialGraph {
            graph: DiGraph::new(),
        }
    }

    pub fn build_graph(&mut self, data: &Vec<(String, String, f64)>) {
        let mut node_indices = HashMap::new(); // To store node indices

        for (source, target, weight) in data {
            // Get or create node indices
            let source_index = *node_indices.entry(source).or_insert_with(|| {
                let index = self.graph.add_node(source.clone());
                NodeIndex::new(index.index())
            });

            let target_index = *node_indices.entry(target).or_insert_with(|| {
                let index = self.graph.add_node(target.clone());
                NodeIndex::new(index.index())
            });

            // Add an edge using node indices
            self.graph.add_edge(source_index, target_index, *weight);
        }
    }

    pub fn degree_distribution(&self) -> Vec<usize> {
        // degrees(&self.graph).map(|(_, degree)| degree).collect()
        unimplemented!("You need to implement degree_distribution")
    }
}