use std::collections::{HashMap, HashSet};

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

    fn find_path(&self, from_vertex: u32, to_vertex: u32) -> Vec<u32> {
        let mut path = vec![];

        let mut visited = HashSet::new();
        visited.insert(from_vertex);

        self.dfs_recursive(from_vertex, to_vertex, &mut path, &mut visited);

        path
    }

    fn dfs_recursive(
        &self,
        vertex: u32,
        to_vertex: u32,
        path: &mut Vec<u32>,
        visited: &mut HashSet<u32>,
    ) {
        path.push(vertex);
        if vertex != to_vertex {
            let adj_list = self.adjacency_list.get(&vertex).unwrap();
            for v in adj_list {
                if visited.contains(&v) == false {
                    visited.contains(v);
                    self.dfs_recursive(*v, to_vertex, path, visited);

                    // SS: found a path?
                    if path[path.len() - 1] == to_vertex {
                        // SS: yes, found a path
                        return;
                    }
                }
            }

            // SS: dead end, remove vertex from path
            path.remove(path.len() - 1);
        }
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
    fn graph1_path_from_0_to_10() {
        // Arrange
        let g = create_graph1();

        // Act
        let path = g.find_path(0, 10);

        // Assert
        assert_eq!(path[0], 0);
        assert_eq!(*path.last().unwrap(), 10);
    }

    #[test]
    fn graph1_path_from_2_to_12() {
        // Arrange
        let g = create_graph1();

        // Act
        let path = g.find_path(2, 12);

        // Assert
        assert_eq!(path[0], 2);
        assert_eq!(*path.last().unwrap(), 12);
    }

    #[test]
    fn graph1_path_from_8_to_2() {
        // Arrange
        let g = create_graph1();

        // Act
        let path = g.find_path(8, 2);

        // Assert
        // SS: there is no path
        assert_eq!(path.len(), 0);
    }
}
