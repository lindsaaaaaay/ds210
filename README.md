# ds210
GR-QC Collaboration Network Centrality Analysis

This project analyzes collaboration networks in the General Relativity and Quantum Cosmology (GR-QC) category from the ArXiv dataset. It provides insights into the structural properties of the network using centrality measures and visualizations.

Features

Load Collaboration Network: Load the dataset to construct an undirected graph of collaborations.

Centrality Measures:

Degree Centrality

Betweenness Centrality (approximation)

Eigenvector Centrality (iterative method)

Visualization: Generate a graphical representation of the collaboration network.

Connected Components: Compute the number of connected components in the graph.

How to Run the Project

Prerequisites

Rust (latest stable version)

cargo build system

The ArXiv GR-QC dataset (formatted as a text file where each line represents an edge as "node1 node2")

Steps

Clone this repository:

git clone <repository-url>
cd grqc_centrality_analysis

Build the project:

cargo build

Run the analysis:

cargo run -- <path_to_dataset>

Example:

cargo run -- data/grqc_collaboration.txt

Output

Console Output

Graph Summary: Number of nodes and edges in the graph.

Connected Components: Total number of connected components in the graph.

Top Centrality Scores: Top 10 authors ranked by degree, betweenness, and eigenvector centrality.

Example:

Graph loaded with 5242 nodes and 14496 edges.
Number of connected components: 13

Top authors by degree centrality:
Author 124: 32
Author 223: 29
...

Top authors by betweenness centrality:
Author 124: 10234
Author 523: 8912
...

Top authors by eigenvector centrality:
Author 412: 34567
Author 612: 29876
...

Visualization

A PNG file named network.png is generated in the output/ directory, depicting the collaboration network.

Project Structure

src/main.rs: Main logic tying all components together.

src/lib.rs: Core functionality, including graph loading, centrality computations, and visualization.

output/network.png: Generated network visualization.

Methodology

Degree Centrality

Calculates the number of direct connections for each author in the graph.

Betweenness Centrality

Uses a simple approximation based on Dijkstra's algorithm to estimate the shortest path contributions of each author.

Eigenvector Centrality

Iteratively computes the relative influence of nodes based on their connections.

Visualization

Leverages the plotters crate to produce a visual representation of the network.

Testing

The project includes unit tests to verify the correctness of the graph loading, centrality computations, and connected components.

Run the tests:

cargo test

Future Enhancements

Optimize eigenvector centrality using sparse matrix libraries.

Add support for directed graphs and weighted edges.

Enhance visualization with interactive features.

Acknowledgments

This project leverages the ArXiv dataset and the Petgraph library for graph operations.

License

This project is licensed under the MIT License.
