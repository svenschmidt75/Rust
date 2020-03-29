// Cracking the Coding Interview
// 6th ed, p. 109, ex. 4.1

use std::collections::{HashMap, HashSet, VecDeque};
use std::thread::current;

#[derive(PartialEq, Debug)]
struct WeightedEdge(u64, u64, i64);

struct Path(Vec<WeightedEdge>);

struct Graph {
    adjacency_list: HashMap<u64, Vec<(u64, i64)>>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    fn add_vertex(&mut self, vertex: u64) {
        let mut adj_list = self.adjacency_list.get_mut(&vertex);
        if let None = adj_list {
            self.adjacency_list.insert(vertex, vec![]);
        }
    }

    fn add_undirected_edge(&mut self, from_vertex: u64, to_vertex: u64, weight: i64) {
        self.add_directed_edge(from_vertex, to_vertex, weight);
        self.add_directed_edge(to_vertex, from_vertex, weight);
    }

    fn add_directed_edge(&mut self, from_vertex: u64, to_vertex: u64, weight: i64) {
        let mut adj_list = self.adjacency_list.get_mut(&from_vertex).unwrap();
        adj_list.push((to_vertex, weight));
    }

    fn route_between_nodes(&self, from_vertex: u64, to_vertex: u64) -> bool {
        // SS: use breadth-first approach
        let mut queue = VecDeque::new();
        queue.push_front(from_vertex);
        while queue.is_empty() == false {
            let vertex = queue.pop_back().unwrap();
            if vertex == to_vertex {
                return true;
            }

            // SS: put all neighbor's into queue
            self.adjacency_list[&vertex]
                .iter()
                .for_each(|(neighbor, _)| queue.push_front(*neighbor));
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_graph() -> Graph {
        let mut graph = Graph::new();

        graph.add_vertex(0);
        graph.add_vertex(1);
        graph.add_vertex(2);
        graph.add_vertex(3);
        graph.add_vertex(4);
        graph.add_vertex(5);
        graph.add_vertex(6);
        graph.add_vertex(7);
        graph.add_vertex(8);
        graph.add_vertex(9);
        graph.add_vertex(10);
        graph.add_vertex(11);
        graph.add_vertex(12);

        // (0, 1)
        // (0, 2)
        // (0, 6)
        graph.add_directed_edge(0, 1, 1);
        graph.add_directed_edge(0, 2, 1);
        graph.add_directed_edge(0, 6, 1);

        // (1, 5)
        // (1, 6)
        graph.add_directed_edge(1, 5, 1);
        graph.add_directed_edge(1, 5, 1);

        // (2, 3)
        // (2, 4)
        graph.add_directed_edge(2, 3, 1);
        graph.add_directed_edge(2, 4, 1);

        // (3, 11)
        graph.add_directed_edge(3, 11, 1);

        // (4, 8)
        // (4, 10)
        graph.add_directed_edge(4, 8, 1);
        graph.add_directed_edge(4, 10, 1);

        // (6, 7)
        graph.add_directed_edge(6, 7, 1);

        // (7, 9)
        graph.add_directed_edge(7, 9, 1);

        // (12, 10)
        graph.add_directed_edge(12, 10, 1);

        graph
    }

    #[test]
    fn test_has_path() {
        // Arrange
        let graph = create_graph();

        // Act
        let path_exists = graph.route_between_nodes(0, 10);

        // Assert
        assert_eq!(path_exists, true);
    }

    #[test]
    fn test_has_no_path() {
        // Arrange
        let graph = create_graph();

        // Act
        let path_exists = graph.route_between_nodes(8, 0);

        // Assert
        assert_eq!(path_exists, false);
    }
}
