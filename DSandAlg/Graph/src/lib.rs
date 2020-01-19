use std::collections::HashMap;

struct Graph {
    adjacency_list: HashMap<u64, Vec<(u64, i64)>>
}

impl Graph {

    fn new() -> Graph {
        Graph { adjacency_list: HashMap::new() }
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

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_computerphile() {
        // SS: graph used in 'Dijkstra's Algorithm - Computerphile', https://www.youtube.com/watch?v=GazC3A4OQTE
        // undirected, weighted, cyclic

        // Arrange
        let mut graph = Graph::new();

        // Act
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

        // (A, S, 7)
        // (A, B, 3)
        // (A, D, 4)
        graph.add_undirected_edge(1, 0, 7);
        graph.add_undirected_edge(1, 2, 3);
        graph.add_undirected_edge(1, 3, 4);

        // (B, D, 4)
        // (B, H, 1)
        // (B, S, 2)
        graph.add_undirected_edge(2, 3, 4);
        graph.add_undirected_edge(2, 5, 1);
        graph.add_undirected_edge(2, 0, 2);

        // (C, L, 2)
        // (C, S, 3)
        graph.add_undirected_edge(12, 11, 2);
        graph.add_undirected_edge(12, 0, 3);

        // (D, F, 5)
        graph.add_undirected_edge(3, 4, 5);

        // (E, G, 2)
        // (E, K, 5)
        graph.add_undirected_edge(7, 6, 2);
        graph.add_undirected_edge(7, 8, 5);

        // (F, H, 3)
        graph.add_undirected_edge(4, 5, 3);

        // (G, H, 2)
        graph.add_undirected_edge(6, 5, 2);

        // (I, J, 6)
        // (I, K, 4)
        // (I, L, 4)
        graph.add_undirected_edge(9, 10, 6);
        graph.add_undirected_edge(9, 8, 4);
        graph.add_undirected_edge(9, 11, 4);

        // (J, K, 4)
        // (J, L, 4)
        graph.add_undirected_edge(10, 8, 4);
        graph.add_undirected_edge(10, 11, 4);

        // Assert
    }
}
