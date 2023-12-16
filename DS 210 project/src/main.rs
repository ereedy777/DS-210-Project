mod graph;
mod io;
mod tests;

extern crate petgraph;
extern crate csv;


use std::collections::VecDeque;


fn main() {
    let file_path = "data/synthetic_financial_data.csv";
    let data = io::read_csv(file_path).expect("Failed to read CSV");

    let mut financial_graph = graph::FinancialGraph::new();
    financial_graph.build_graph(&data);

    // Vertex Degrees
    let degree_distribution = financial_graph.degree_distribution();
    println!("Vertex Degrees: {:?}", degree_distribution);

    // Number of Neighbors at Distance 2 (Sample Implementation)
    let distance_2_neighbors = calculate_neighbors_at_distance_2(&financial_graph);
    println!("Number of Neighbors at Distance 2: {:?}", distance_2_neighbors);

    // Clustering Coefficient
    let clustering_coefficient = calculate_clustering_coefficient(&financial_graph);
    println!("Clustering Coefficient: {:.4}", clustering_coefficient);

    // Power-Law Distribution Analysis (Sample Implementation)
    let is_power_law = analyze_power_law(&degree_distribution);
    if is_power_law {
        println!("The degree distribution follows a power-law.");
    } else {
        println!("The degree distribution does not follow a power-law.");
    }
}

fn calculate_neighbors_at_distance_2(graph: &graph::FinancialGraph) -> Vec<(String, usize)> {
    let mut neighbors_2 = Vec::new();
    for node in graph.graph.node_indices() {
        let mut visited = vec![false; graph.graph.node_count()];
        visited[node.index()] = true;
        
        let mut queue = VecDeque::new();
        queue.push_back(node);

        let mut count = 0;
        while let Some(current_node) = queue.pop_front() {
            for neighbor in graph.graph.neighbors(current_node) {
                if !visited[neighbor.index()] {
                    visited[neighbor.index()] = true;
                    for second_neighbor in graph.graph.neighbors(neighbor) {
                        if second_neighbor != node {
                            count += 1;
                        }
                    }
                    queue.push_back(neighbor);
                }
            }
        }
        neighbors_2.push((graph.graph[node].clone(), count));
    }
    neighbors_2
}
fn calculate_clustering_coefficient(graph: &graph::FinancialGraph) -> f64 {
    let mut total_coefficient = 0.0;
    for node in graph.graph.node_indices() {
        let neighbors: Vec<_> = graph.graph.neighbors(node).collect();
        let neighbor_count = neighbors.len();
        if neighbor_count >= 2 {
            let mut triangles = 0;
            for i in 0..neighbor_count {
                for j in (i + 1)..neighbor_count {
                    if graph.graph.contains_edge(neighbors[i], neighbors[j]) {
                        triangles += 1;
                    }
                }
            }
            if triangles > 0 {
                let coefficient = (2 * triangles) as f64 / (neighbor_count * (neighbor_count - 1)) as f64;
                total_coefficient += coefficient;
            }
        }
    }
    if graph.graph.node_count() > 0 {
        total_coefficient / graph.graph.node_count() as f64
    } else {
        0.0
    }
}

fn calculate_mean(data: &Vec<f64>) -> f64 {
    let sum: f64 = data.iter().sum();
    let count = data.len() as f64;
    sum / count
}

fn analyze_power_law(degree_distribution: &Vec<usize>) -> bool {
    // Check if the degree distribution is empty to prevent panics
    if degree_distribution.is_empty() {
        return false;
    }

    // Convert degree_distribution to a float vector
    let degrees: Vec<f64> = degree_distribution.iter().map(|&x| x as f64).collect();

    // Calculate the mean of the degree distribution
    let mean = calculate_mean(&degrees);

    // Calculate the threshold for power-law analysis
    // You can choose a threshold based on your criteria
    let threshold = 0.05; // Adjust as needed

    // Perform the power-law analysis
    degrees.iter().all(|&x| x >= threshold * mean)
}