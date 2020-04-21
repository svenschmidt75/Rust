mod pq;

use crate::pq::PriorityQueue;
use std::collections::{HashMap, HashSet};

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

    fn prim(&self, start_vertex: u64) -> (u64, Vec<(u64, u64)>) {
        // SS: implementation of Prim's algorithm
        let mut mst_cost = 0;
        let mut mst = vec![];

        let mut visited = HashSet::new();

        let mut pq = PriorityQueue::new();

        let mut current_vertex = start_vertex;

        let number_of_vertices = self.adjacency_list.len();

        loop {
            visited.insert(current_vertex);

            if number_of_vertices == visited.len() {
                break;
            }

            let edges = self.adjacency_list.get(&current_vertex).unwrap();
            for edge in edges {
                let (target_vertex, weight) = *edge;
                if visited.contains(&target_vertex) == false {
                    pq.insert(weight, (current_vertex, edge.0, edge.1));
                }
            }

            let (_, (vertex1, vertex2, weight)) = pq.pop();
            mst.push((vertex1, vertex2));
            mst_cost += weight;
            current_vertex = vertex2;
        }

        (mst_cost as u64, mst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_graph_1() -> Graph {
        // Udemy - https://www.udemy.com/course/algorithms-and-data-structures-in-python/
        // Prim's algorithm, section 15, module 111
        let mut graph = Graph::new();

        graph.add_vertex(0); // A
        graph.add_vertex(1); // B
        graph.add_vertex(2); // C
        graph.add_vertex(3); // D
        graph.add_vertex(4); // E
        graph.add_vertex(5); // F
        graph.add_vertex(6); // G

        // (A, B, 2)
        // (A, C, 6)
        // (A, E, 5)
        // (A, F, 10)
        graph.add_undirected_edge(0, 1, 2);
        graph.add_undirected_edge(0, 2, 6);
        graph.add_undirected_edge(0, 4, 5);
        graph.add_undirected_edge(0, 5, 10);

        // (B, D, 3)
        // (B, E, 3)
        graph.add_undirected_edge(1, 3, 3);
        graph.add_undirected_edge(1, 4, 3);

        // (C, D, 1)
        // (C, F, 2)
        graph.add_undirected_edge(2, 3, 1);
        graph.add_undirected_edge(2, 5, 2);

        // (D, E, 4)
        // (D, G, 5)
        graph.add_undirected_edge(3, 4, 4);
        graph.add_undirected_edge(3, 6, 5);

        // (F, G, 3)
        graph.add_undirected_edge(5, 6, 3);

        graph
    }

    #[test]
    fn shortest_path_1() {
        // Arrange
        let g = create_graph_1();

        // Act
        let (mst_cost, mst) = g.prim(3);

        // Assert
        assert_eq!(mst_cost, 14);
        assert_eq!(mst, vec![(3, 2), (2, 5), (3, 1), (1, 0), (1, 4), (5, 6)]);
    }
}
