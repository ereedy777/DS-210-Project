use super::*; // Import the parent module (containing the code to test)
use std::error::Error;
use crate::graph::FinancialGraph;



#[test]
fn test_read_csv() -> Result<(), Box<dyn Error>> {
    let data = io::read_csv("synthetic_financial_data.csv")?;
    assert_eq!(data.len(), 5);
    Ok(())
}

#[test]
fn test_graph_degree_distribution() {
    let mut graph = FinancialGraph::new();
    let data = vec![
        ("A".to_string(), "B".to_string(), 1.0),
        // Add more data here
    ];
    graph.build_graph(&data);

    let degree_dist = graph.degree_distribution();
    // Add assertions for degree distribution
}

#[test]
fn test_calculate_neighbors_at_distance_2() {
    let mut graph = FinancialGraph::new();
    let data = vec![
        ("A".to_string(), "B".to_string(), 1.0),
        ("B".to_string(), "C".to_string(), 1.0),
        ("C".to_string(), "D".to_string(), 1.0),
        ("A".to_string(), "D".to_string(), 1.0),
        ("D".to_string(), "E".to_string(), 1.0),
    ];
    graph.build_graph(&data);

    let neighbors_2 = calculate_neighbors_at_distance_2(&graph);
    // Add assertions for neighbors at distance 2
}

#[test]
fn test_power_law_analysis() {
    let degree_distribution = vec![3, 5, 8, 10, 15, 20, 25];
    let is_power_law = analyze_power_law(&degree_distribution);
    assert!(is_power_law);
}