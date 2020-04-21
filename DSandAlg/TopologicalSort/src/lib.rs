use std::collections::{HashMap, HashSet, VecDeque};

struct Graph {
    adjacency_list: HashMap<u64, Vec<u64>>,
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

    fn add_undirected_edge(&mut self, from_vertex: u64, to_vertex: u64) {
        self.add_directed_edge(from_vertex, to_vertex);
        self.add_directed_edge(to_vertex, from_vertex);
    }

    fn add_directed_edge(&mut self, from_vertex: u64, to_vertex: u64) {
        let mut adj_list = self.adjacency_list.get_mut(&from_vertex).unwrap();
        adj_list.push(to_vertex);
    }

    fn topological_sort(&self, start_vertex: u64) -> Vec<u64> {
        let mut sorted_vertices_stack = VecDeque::new();
        let mut not_visited = self
            .adjacency_list
            .iter()
            .map(|(vertex, _)| *vertex)
            .collect::<HashSet<_>>();

        // SS: depth-first
        self.topological_sort_internal(start_vertex, &mut not_visited, &mut sorted_vertices_stack);

        let mut remaining = not_visited
            .iter()
            .map(|&vertex| vertex)
            .collect::<VecDeque<u64>>();

        // SS: process all remaining vertices
        while remaining.is_empty() == false {
            let vertex = remaining.pop_back().unwrap();
            self.topological_sort_internal(vertex, &mut not_visited, &mut sorted_vertices_stack);
            remaining = not_visited
                .iter()
                .map(|&vertex| vertex)
                .collect::<VecDeque<u64>>();
        }

        sorted_vertices_stack.into_iter().rev().collect()
    }

    fn topological_sort_internal(
        &self,
        vertex: u64,
        not_visited: &mut HashSet<u64>,
        sorted_vertices_stack: &mut VecDeque<u64>,
    ) {
        not_visited.remove(&vertex);
        let neighbors = self.adjacency_list.get(&vertex).unwrap();
        for &neighbor in neighbors {
            if not_visited.contains(&neighbor) == false {
                // SS: vertex already visited, skip
                continue;
            }
            // SS: `neighbor` depends on `vertex`, so process first
            self.topological_sort_internal(neighbor, not_visited, sorted_vertices_stack);
        }
        sorted_vertices_stack.push_back(vertex);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_graph_1() -> Graph {
        // Udemy - https://www.udemy.com/course/draft/1330262/learn/lecture/14175831#overview
        // Data Structures and Algorithms, DS Guy, S11 - L10
        let mut graph = Graph::new();

        graph.add_vertex(0); // A
        graph.add_vertex(1); // B
        graph.add_vertex(2); // C
        graph.add_vertex(3); // D
        graph.add_vertex(4); // E
        graph.add_vertex(5); // F
        graph.add_vertex(6); // G
        graph.add_vertex(7); // H

        // (A, C) - connection from A to C => C depends on A
        graph.add_directed_edge(0, 2);

        // (B, C)
        // (B, D)
        graph.add_directed_edge(1, 2);
        graph.add_directed_edge(1, 3);

        // (C, E)
        graph.add_directed_edge(2, 4);

        // (D, F)
        graph.add_directed_edge(3, 5);

        // (E, F)
        // (E, H)
        graph.add_directed_edge(4, 5);
        graph.add_directed_edge(4, 7);

        // (F, G)
        graph.add_directed_edge(5, 6);

        graph
    }

    #[test]
    fn topological_sort_test() {
        // Arrange
        let g = create_graph_1();

        // Act
        let sorted_vertices = g.topological_sort(0);

        // Assert
        assert_eq!(sorted_vertices, vec![1, 3, 0, 2, 4, 7, 5, 6]);
    }
}
