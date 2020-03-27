use std::collections::{HashMap, HashSet, VecDeque};

struct Graph {
    adjacency_list: HashMap<u32, Vec<u32>>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    fn add_vertex(&mut self, vertex: u32) {
        self.adjacency_list.insert(vertex, vec![]);
    }

    fn add_directed_edge(&mut self, vertex1: u32, vertex2: u32) {
        self.adjacency_list.get_mut(&vertex1).unwrap().push(vertex2);
    }

    fn add_undirected_edge(&mut self, vertex1: u32, vertex2: u32) {
        self.add_directed_edge(vertex1, vertex2);
        self.add_directed_edge(vertex2, vertex1);
    }

    fn find_vertices_of_distance_k(&self, start_vertex: u32, k: u32) -> Vec<u32> {
        // SS: find all vertices that are k vertices distance away.
        // We use breadth-search approach...
        let mut result = vec![];

        let mut queue = VecDeque::new();
        queue.push_front((start_vertex, k));

        // SS: during graph traversal, always need to keep track of visited
        // vertices due to possible cycles...
        let mut visited = HashSet::new();
        visited.insert(start_vertex);

        while queue.is_empty() == false {
            let (vertex, d) = queue.pop_back().unwrap();
            if d == 0 {
                result.push(vertex);
            } else {
                let adj_list = &self.adjacency_list[&vertex];
                for v in adj_list {
                    if visited.contains(v) == false {
                        visited.insert(*v);
                        queue.push_front((*v, d - 1));
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Graph;

    fn create_graph1() -> Graph {
        let mut g = Graph::new();

        g.add_vertex(0);
        g.add_vertex(1);
        g.add_vertex(2);
        g.add_vertex(3);
        g.add_vertex(4);
        g.add_vertex(5);
        g.add_vertex(6);
        g.add_vertex(7);
        g.add_vertex(8);
        g.add_vertex(9);
        g.add_vertex(10);
        g.add_vertex(11);
        g.add_vertex(12);

        g.add_directed_edge(0, 1);
        g.add_directed_edge(0, 2);
        g.add_directed_edge(0, 6);

        g.add_directed_edge(1, 5);
        g.add_directed_edge(1, 6);

        g.add_directed_edge(2, 3);
        g.add_directed_edge(2, 4);

        g.add_directed_edge(3, 11);

        g.add_directed_edge(4, 8);
        g.add_directed_edge(4, 10);

        g.add_directed_edge(6, 7);

        g.add_directed_edge(7, 9);

        g.add_directed_edge(8, 6);

        g.add_directed_edge(9, 12);

        g.add_directed_edge(12, 10);

        g
    }

    #[test]
    fn graph1_distance_2_from_0() {
        // Arrange
        let g = create_graph1();

        // Act
        let vertices = g.find_vertices_of_distance_k(0, 2);

        // Assert
        assert_eq!(vertices, vec![5, 3, 4, 7]);
    }

    #[test]
    fn graph1_distance_3_from_8() {
        // Arrange
        let g = create_graph1();

        // Act
        let vertices = g.find_vertices_of_distance_k(8, 3);

        // Assert
        assert_eq!(vertices, vec![9]);
    }
}
