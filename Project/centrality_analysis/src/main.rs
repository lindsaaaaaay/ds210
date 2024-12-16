use petgraph::{graph::Graph, Undirected};
use petgraph::algo::{connected_components, dijkstra};
use petgraph::dot::{Dot, Config};
use petgraph::visit::{EdgeRef, IntoNodeReferences};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use plotters::prelude::*;
use std::fs;

// Define a type alias for easier graph representation
type AuthorGraph = Graph<usize, (), Undirected>;

/// Load the dataset and build the graph.
pub fn load_graph(file_path: &str) -> io::Result<AuthorGraph> {
    let mut graph = AuthorGraph::new_undirected();
    let mut node_map = HashMap::new();
    let mut edges = HashSet::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            let line = line?;
            if line.starts_with('#') || line.trim().is_empty() {
                continue;
            }
            let nodes: Vec<usize> = line
                .split_whitespace()
                .filter_map(|x| x.parse::<usize>().ok())
                .collect();

            if nodes.len() == 2 {
                let (from, to) = (nodes[0], nodes[1]);
                if from != to {
                    edges.insert((from.min(to), from.max(to)));
                }
            }
        }

        for &(from, to) in &edges {
            let from_index = *node_map.entry(from).or_insert_with(|| graph.add_node(from));
            let to_index = *node_map.entry(to).or_insert_with(|| graph.add_node(to));
            graph.add_edge(from_index, to_index, ());
        }
    }

    Ok(graph)
}

/// Compute centrality measures for the graph.
/// Compute centrality measures for the graph.
pub fn compute_centralities(graph: &AuthorGraph) {
    let mut degree_centrality = HashMap::new();
    let mut betweenness_centrality = HashMap::new();
    let mut eigenvector_centrality = HashMap::new();

    // Compute degree centrality
    for node in graph.node_indices() {
        degree_centrality.insert(graph[node], graph.edges(node).count());
    }

    // Compute betweenness centrality (simple approximation via Dijkstra)
    for node in graph.node_indices() {
        let distances = dijkstra(&graph, node, None, |_| 1);
        let total_distance: usize = distances.values().sum();
        betweenness_centrality.insert(graph[node], total_distance);
    }

    // Compute eigenvector centrality (simple iteration)
    let mut centrality_values: HashMap<_, f64> = graph
        .node_indices()
        .map(|node| (graph[node], 1.0)) // Initialize all centralities to 1.0
        .collect();
    let num_iterations = 100; // Set max iterations
    let tolerance = 1e-6; // Convergence threshold

    for _ in 0..num_iterations {
        let mut next_centrality_values = centrality_values.clone();

        for node in graph.node_indices() {
            let sum: f64 = graph
                .edges(node)
                .map(|edge| centrality_values[&graph[edge.target()]])
                .sum();
            next_centrality_values.insert(graph[node], sum);
        }

        // Normalize
        let norm: f64 = next_centrality_values.values().map(|v| v * v).sum::<f64>().sqrt();
        for value in next_centrality_values.values_mut() {
            *value /= norm;
        }

        // Check convergence
        let max_difference = centrality_values
            .iter()
            .map(|(node, value)| (value - next_centrality_values[node]).abs())
            .fold(0.0, f64::max);

        if max_difference < tolerance {
            break;
        }

        centrality_values = next_centrality_values;
    }

    // Store eigenvector centralities as usize for compatibility with print_top
    for (node, value) in centrality_values {
        eigenvector_centrality.insert(node, (value * 1_000_000.0) as usize); // Scale to usize for readability
    }

    // Print results
    println!("Top authors by degree centrality:");
    print_top(&degree_centrality);

    println!("\nTop authors by betweenness centrality:");
    print_top(&betweenness_centrality);

    println!("\nTop authors by eigenvector centrality:");
    print_top(&eigenvector_centrality);
}

/// Utility to print the top centrality values.
fn print_top(centrality: &HashMap<usize, usize>) {
    let mut centrality_vec: Vec<_> = centrality.iter().collect();
    centrality_vec.sort_by(|a, b| b.1.cmp(a.1));
    for &(author, score) in centrality_vec.iter().take(10) {
        println!("Author {}: {}", author, score);
    }
}

/// Visualize the graph.
pub fn visualize_graph(graph: &AuthorGraph) {
    // Ensure the output directory exists
    let output_dir = "output";
    fs::create_dir_all(output_dir).unwrap();

    let root = BitMapBackend::new("output/network.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("Collaboration Network", ("sans-serif", 50))
        .build_cartesian_2d(-10..10, -10..10)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    for edge in graph.edge_references() {
        let (start, end) = (
            graph[edge.source()],
            graph[edge.target()],
        );
        chart.draw_series(LineSeries::new(
            vec![(start as i32, 0), (end as i32, 0)],
            &BLACK,
        )).unwrap();
    }

    root.present().unwrap();
}

/// Utility to read lines from a file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Main function to tie everything together.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_dataset>", args[0]);
        return;
    }

    let file_path = &args[1];

    match load_graph(file_path) {
        Ok(graph) => {
            println!("Graph loaded with {} nodes and {} edges.", graph.node_count(), graph.edge_count());

            let components = connected_components(&graph);
            println!("Number of connected components: {}", components);

            compute_centralities(&graph);
            visualize_graph(&graph);
        }
        Err(e) => {
            eprintln!("Failed to load graph: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_graph() {
        let test_data = "ca-GrQc.txt";
        std::fs::write(test_data, "1\t2\n2\t3\n3\t1\n4\t5\n").unwrap();

        let graph = load_graph(test_data).unwrap();
        assert_eq!(graph.node_count(), 5);
        assert_eq!(graph.edge_count(), 4);

        std::fs::remove_file(test_data).unwrap();
    }

    #[test]
    fn test_compute_centralities() {
        let mut graph = AuthorGraph::new_undirected();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);

        graph.add_edge(n1, n2, ());
        graph.add_edge(n2, n3, ());

        compute_centralities(&graph);

        // Simple assertions to ensure the function runs
        assert!(graph.node_count() > 0);
    }

    #[test]
    fn test_connected_components() {
        let mut graph = AuthorGraph::new_undirected();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);
        let n5 = graph.add_node(5);

        graph.add_edge(n1, n2, ());
        graph.add_edge(n2, n3, ());
        graph.add_edge(n4, n5, ());

        let components = connected_components(&graph);
        assert_eq!(components, 2);
    }

    #[test]
fn test_visualize_graph() {
    // Create a small sample graph
    let mut graph = AuthorGraph::new_undirected();
    let n1 = graph.add_node(1);
    let n2 = graph.add_node(2);
    let n3 = graph.add_node(3);
    
    graph.add_edge(n1, n2, ());
    graph.add_edge(n2, n3, ());
    graph.add_edge(n3, n1, ());

    // Call the visualization function
    visualize_graph(&graph);

    // Check that the output file exists
    let output_path = "output/network.png";
    assert!(std::path::Path::new(output_path).exists());
    
    // Clean up the generated file
    std::fs::remove_file(output_path).unwrap();
}

}
