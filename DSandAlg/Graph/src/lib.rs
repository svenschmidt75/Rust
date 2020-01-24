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

    fn find_path_bfs(&self, from_vertex: u64, to_vertex: u64) -> Option<Path> {
        // SS: Use BFS to find a path between two nodes. DFS is often more efficient for this.
        // Note that BFS finds the shortest path in an unweighted graph, NOT all possible paths!!!
        let mut shortest_path = VecDeque::new();

        let mut queue = VecDeque::new();
        let mut path = VecDeque::new();

        // SS: since the graph may have cycles, we need to keep track of the vertices already visited
        let mut visited_vertices = HashSet::new();

        let mut current_vertex = from_vertex;
        path.push_back(current_vertex);
        queue.push_back((current_vertex, path));

        while queue.is_empty() == false {
            let (current_vertex, path) = queue.pop_front().unwrap();

            if visited_vertices.contains(&current_vertex) {
                continue;
            }

            visited_vertices.insert(current_vertex);

            if current_vertex == to_vertex {
                // SS: done, we found a path
                shortest_path.clone_from(&path);
                break;
            }

            let adj_list = self.adjacency_list.get(&current_vertex).unwrap();
            for (next_vertex, edge_weight) in adj_list {
                let mut new_path = path.clone();
                new_path.push_back(*next_vertex);
                queue.push_back((*next_vertex, new_path));
            }
        }

        if shortest_path.is_empty() {
            None
        } else {
            let p = shortest_path
                .iter()
                .zip(shortest_path.iter().skip(1))
                .map(|(&from, &to)| {
                    let adj_list = self.adjacency_list.get(&from).unwrap();

                    // SS: linear search
                    let idx = adj_list.iter().position(|elem| elem.0 == to).unwrap();
                    let weight = adj_list[idx].1;

                    WeightedEdge(from, to, weight)
                })
                .collect::<Vec<_>>();
            Some(Path(p))
        }
    }

    fn find_path_dfs(&self, from_vertex: u64, to_vertex: u64) -> Vec<Path> {
        let mut paths = vec![];
        let path = [];
        let mut visited_vertices = HashSet::new();
        self.find_path_dfs_internal(
            from_vertex,
            to_vertex,
            &mut visited_vertices,
            &path,
            &mut paths,
        );

        let solutions = paths
            .iter()
            .map(|path| {
                path.iter()
                    .zip(path.iter().skip(1))
                    .map(|(&from, &to)| {
                        let adj_list = self.adjacency_list.get(&from).unwrap();

                        // SS: linear search
                        let idx = adj_list.iter().position(|elem| elem.0 == to).unwrap();
                        let weight = adj_list[idx].1;

                        WeightedEdge(from, to, weight)
                    })
                    .collect::<Vec<_>>()
            })
            .map(|p| Path(p))
            .collect::<Vec<_>>();
        solutions
    }

    fn find_path_dfs_internal(
        &self,
        current_vertex: u64,
        to_vertex: u64,
        visited_vertices: &mut HashSet<u64>,
        path: &[u64],
        paths: &mut Vec<Vec<u64>>,
    ) {
        if visited_vertices.contains(&current_vertex) {
            return;
        }

        let mut new_path = path.to_vec().clone();
        new_path.push(current_vertex);
        visited_vertices.insert(current_vertex);

        if current_vertex == to_vertex {
            // SS: we have a solution
            paths.push(new_path.clone());
        } else {
            let adj_list = self.adjacency_list.get(&current_vertex).unwrap();
            for (v, _) in adj_list {
                self.find_path_dfs_internal(*v, to_vertex, visited_vertices, &new_path, paths);
            }
        }

        // SS: backtrack
        visited_vertices.remove(&current_vertex);
    }

    fn test() {

        let f = ||{
            f();
        };

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

        graph
    }

    #[test]
    fn test_create_computerphile() {
        // SS: graph used in 'Dijkstra's Algorithm - Computerphile', https://www.youtube.com/watch?v=GazC3A4OQTE
        // undirected, weighted, cyclic

        // Arrange
        // Act
        let graph = create_graph();

        // Assert
    }

    #[test]
    fn test_bfs() {
        // SS: graph used in 'Dijkstra's Algorithm - Computerphile', https://www.youtube.com/watch?v=GazC3A4OQTE
        // undirected, weighted, cyclic

        // Arrange
        let graph = create_graph();

        // Act
        let shortest_path = graph.find_path_bfs(0, 7).unwrap();

        // Assert
        assert_eq!(shortest_path.0.len(), 4);
        assert_eq!(shortest_path.0[0], WeightedEdge(0, 2, 2));
        assert_eq!(shortest_path.0[1], WeightedEdge(2, 5, 1));
        assert_eq!(shortest_path.0[2], WeightedEdge(5, 6, 2));
        assert_eq!(shortest_path.0[3], WeightedEdge(6, 7, 2));
    }

    #[test]
    fn test_dfs() {
        // SS: graph used in 'Dijkstra's Algorithm - Computerphile', https://www.youtube.com/watch?v=GazC3A4OQTE
        // undirected, weighted, cyclic

        // Arrange
        let graph = create_graph();

        // Act
        let paths = graph.find_path_dfs(0, 7);

        // Assert
        assert_eq!(paths.len(), 11);
    }
}
