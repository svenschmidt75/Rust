// SS: Partition vertices of an undirected graph into connected components.
// We find connected components by using DFS.

mod graph;

use crate::graph::Graph;
use std::collections::{HashSet, VecDeque};

fn find_connected_components(graph: &Graph) -> Vec<Vec<u64>> {
    let mut connected_components = vec![];

    let mut queue = VecDeque::new();
    for (vertex, neighbors) in &graph.adjacency_list {
        queue.push_back(*vertex);
    }

    let mut visited = HashSet::new();

    while queue.is_empty() == false {
        let vertex = queue.pop_front().unwrap();
        if visited.contains(&vertex) {
            continue;
        }

        let connected_component = depth_first(graph, vertex, &mut visited);
        connected_components.push(connected_component);
    }

    connected_components
}

fn depth_first(graph: &Graph, vertex: u64, visited: &mut HashSet<u64>) -> Vec<u64> {
    // SS: must only be called on unvisited vertices
    let mut component = vec![];
    depth_first_recursive(graph, vertex, visited, &mut component);
    component
}

fn depth_first_recursive(
    graph: &Graph,
    vertex: u64,
    visited: &mut HashSet<u64>,
    component: &mut Vec<u64>,
) {
    if visited.contains(&vertex) == false {
        component.push(vertex);

        let neighbors = graph.adjacency_list.get(&vertex).unwrap();
        for (neighbor, _) in neighbors {
            depth_first_recursive(graph, *neighbor, visited, component);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn create_graph_1() -> Graph {
        let mut graph = Graph::new();

        graph.add_vertex(0); // A
        graph.add_vertex(1); // B
        graph.add_vertex(2); // C
        graph.add_vertex(3); // D
        graph.add_vertex(4); // E
        graph.add_vertex(5); // F
        graph.add_vertex(6); // G
        graph.add_vertex(7); // H

        // (A, B)
        graph.add_undirected_edge(0, 1, 1);

        // (C, D, 1)
        // (C, E, 1)
        // (C, H, 1)
        graph.add_undirected_edge(2, 3, 1);
        graph.add_undirected_edge(2, 4, 1);
        graph.add_undirected_edge(2, 7, 1);

        // (D, E, 1)
        graph.add_undirected_edge(3, 4, 1);

        // (E, G, 1)
        graph.add_undirected_edge(4, 6, 1);

        // (F, G, 1)
        // (F, H, 1)
        graph.add_undirected_edge(5, 6, 1);
        graph.add_undirected_edge(6, 7, 1);

        graph
    }

    #[test]
    fn test1() {
        // Arrange
        let graph = create_graph_1();

        // Act
        let connected_components = find_connected_components(&graph);

        // Assert
        assert_eq!(connected_components.len(), 2);
        assert_eq!(connected_components[0], vec![0, 1])
    }
}
