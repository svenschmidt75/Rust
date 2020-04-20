use crate::naive_priority_queue;
use crate::naive_priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet, VecDeque};

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

    fn shortest_path1(&self, from_vertex: u64, to_vertex: u64) -> (i64, Vec<u64>) {
        // SS: Dijkstra's shortest path
        let mut visited = HashSet::new();

        // SS: Contains a child-parent relationship, to trace the shortest path
        // back to the start.
        let mut path: HashMap<u64, u64> = HashMap::new();

        // SS: to store the shortest distances from the start vertex to all other nodes
        let mut distances = HashMap::new();

        // SS: insert all vertices with initial priorities
        let mut pq = PriorityQueue::new();
        self.adjacency_list.iter().for_each(|(v, edges)| {
            if *v == from_vertex {
                pq.enqueue(from_vertex, 0);
                distances.insert(v, 0);
            } else {
                pq.enqueue(*v, std::i64::MAX);
                distances.insert(v, std::i64::MAX);
            }
        });

        while pq.is_empty() == false {
            let (_, vertex) = pq.dequeue();

            if vertex == to_vertex {
                // SS: we're done
                break;
            }

            if visited.contains(&vertex) == false {
                visited.insert(vertex);

                // SS: check all of vertex's neighbors and update their distances
                // from the start vertex, from_vertex
                for (neighbor_vertex, neighbor_priority) in &self.adjacency_list[&vertex] {
                    if visited.contains(neighbor_vertex) {
                        continue;
                    }
                    // SS: calculate new distance
                    let new_shortest_distance = distances[&vertex] + *neighbor_priority;
                    let previous_shortest_distance = distances[neighbor_vertex];
                    if new_shortest_distance < previous_shortest_distance {
                        // SS: This tripped me up. We don't actually modify the priority of an existing item in the PQ,
                        // rather, we just insert it again, with updated (i.e. smaller) priority...

                        // SS: insert neighbor with new priority
                        pq.enqueue(*neighbor_vertex, new_shortest_distance);

                        // SS: update the neighbor with the "parent"
                        *path.entry(*neighbor_vertex).or_insert(std::u64::MAX) = vertex;

                        // SS: update shortest distance to neighbor
                        *distances.get_mut(&neighbor_vertex).unwrap() = new_shortest_distance;
                    }
                }
            }
        }

        // SS: we should have found the shortest path
        // Trace path backwards
        let mut shortest_path = vec![to_vertex];
        let mut parent = path[&to_vertex];
        while parent != from_vertex {
            shortest_path.push(parent);
            parent = path[&parent];
        }
        shortest_path.push(from_vertex);
        shortest_path.reverse();

        let distance_shortest_path = distances[&to_vertex];

        (distance_shortest_path, shortest_path)
    }

    fn shortest_path2(&self, from_vertex: u64, to_vertex: u64) -> (i64, Vec<u64>) {
        // SS: Dijkstra's shortest path
        let mut visited = HashSet::new();

        // SS: Contains a child-parent relationship, to trace the shortest path
        // back to the start.
        let mut path: HashMap<u64, u64> = HashMap::new();

        // SS: to store the shortest distances from the start vertex to all other nodes
        let mut distances = HashMap::new();
        self.adjacency_list.iter().for_each(|(v, edges)| {
            distances.insert(v, std::i64::MAX);
        });
        distances.insert(&from_vertex, 0);

        // SS: insert all vertices with initial priorities
        let mut pq = PriorityQueue::new();

        // SS: we don't have to add all vertices to the PQ, rather, we add them as we see
        // them...
        pq.enqueue(from_vertex, 0);

        while pq.is_empty() == false {
            let (_, vertex) = pq.dequeue();

            if vertex == to_vertex {
                // SS: we're done
                break;
            }

            visited.insert(vertex);

            // SS: check all of vertex's neighbors and update their distances
            // from the start vertex, from_vertex
            for (neighbor_vertex, neighbor_priority) in &self.adjacency_list[&vertex] {
                if visited.contains(neighbor_vertex) {
                    continue;
                }

                // SS: calculate new distance
                let new_shortest_distance = distances[&vertex] + *neighbor_priority;
                let previous_shortest_distance = distances[neighbor_vertex];
                if new_shortest_distance < previous_shortest_distance {
                    // SS: This tripped me up. We don't actually modify the priority of an existing item in the PQ,
                    // rather, we just insert it again, with updated (i.e. smaller) priority...

                    // SS: insert neighbor with new priority
                    pq.enqueue(*neighbor_vertex, new_shortest_distance);

                    // SS: update the neighbor with the "parent"
                    *path.entry(*neighbor_vertex).or_insert(std::u64::MAX) = vertex;

                    // SS: update shortest distance to neighbor
                    *distances.entry(&neighbor_vertex).or_insert(std::i64::MAX) =
                        new_shortest_distance;
                }
            }
        }

        // SS: we should have found the shortest path
        // Trace path backwards
        let mut shortest_path = vec![to_vertex];
        let mut parent = path[&to_vertex];
        while parent != from_vertex {
            shortest_path.push(parent);
            parent = path[&parent];
        }
        shortest_path.push(from_vertex);
        shortest_path.reverse();

        let distance_shortest_path = distances[&to_vertex];

        (distance_shortest_path, shortest_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_graph_1() -> Graph {
        // Udemy: JavaScript Algorithms and Data Structures Masterclass
        // Created by Colt Steele
        let mut graph = Graph::new();

        graph.add_vertex(0); // A
        graph.add_vertex(1); // B
        graph.add_vertex(2); // C
        graph.add_vertex(3); // D
        graph.add_vertex(4); // E
        graph.add_vertex(5); // F

        // (A, B, 4)
        // (A, C, 2)
        graph.add_undirected_edge(0, 1, 4);
        graph.add_undirected_edge(0, 2, 2);

        // (B, E, 3)
        graph.add_undirected_edge(1, 4, 3);

        // (C, D, 2)
        // (C, F, 4)
        graph.add_undirected_edge(2, 3, 2);
        graph.add_undirected_edge(2, 5, 4);

        // (D, F, 1)
        // (D, E, 3)
        graph.add_undirected_edge(3, 5, 1);
        graph.add_undirected_edge(3, 4, 3);

        // (E, F, 1)
        graph.add_undirected_edge(4, 5, 1);

        graph
    }

    fn create_graph_2() -> Graph {
        // Youtube: Dijkstra's Algorithm - Computerphile
        let mut graph = Graph::new();

        graph.add_vertex(0); // S
        graph.add_vertex(1); // A
        graph.add_vertex(2); // B
        graph.add_vertex(3); // D
        graph.add_vertex(4); // F
        graph.add_vertex(5); // H
        graph.add_vertex(6); // G
        graph.add_vertex(7); // E
        graph.add_vertex(8); // K
        graph.add_vertex(9); // I
        graph.add_vertex(10); // J
        graph.add_vertex(11); // L
        graph.add_vertex(12); // C

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

    fn create_graph_3() -> Graph {
        // Udemy - https://www.udemy.com/course/algorithms-and-data-structures-in-python/
        let mut graph = Graph::new();

        graph.add_vertex(0); // A
        graph.add_vertex(1); // B
        graph.add_vertex(2); // C
        graph.add_vertex(3); // D
        graph.add_vertex(4); // E
        graph.add_vertex(5); // F
        graph.add_vertex(6); // G
        graph.add_vertex(7); // H

        // (A, B, 5)
        // (A, H, 8)
        // (A, E, 9)
        graph.add_directed_edge(0, 1, 5);
        graph.add_directed_edge(0, 7, 8);
        graph.add_directed_edge(0, 4, 9);

        // (B, C, 12)
        // (B, D, 15)
        // (B, H, 4)
        graph.add_directed_edge(1, 2, 12);
        graph.add_directed_edge(1, 3, 15);
        graph.add_directed_edge(1, 7, 4);

        // (C, D, 3)
        // (C, G, 11)
        graph.add_directed_edge(2, 3, 3);
        graph.add_directed_edge(2, 6, 11);

        // (D, G, 9)
        graph.add_directed_edge(3, 6, 9);

        // (E, F, 4)
        // (E, G, 20)
        // (E, H, 5)
        graph.add_directed_edge(4, 5, 4);
        graph.add_directed_edge(4, 6, 20);
        graph.add_directed_edge(4, 7, 5);

        // (F, C, 1)
        // (F, G, 13)
        graph.add_directed_edge(5, 2, 1);
        graph.add_directed_edge(5, 6, 13);

        // (H, C, 7)
        // (H, F, 6)
        graph.add_directed_edge(7, 2, 7);
        graph.add_directed_edge(7, 5, 6);

        graph
    }

    #[test]
    fn shortest_path_1() {
        // Arrange
        let g = create_graph_1();

        // Act
        let (distance, shortest_path) = g.shortest_path1(0, 4);

        // Assert
        assert_eq!(distance, 6);
        assert_eq!(shortest_path, vec![0, 2, 3, 5, 4]);
    }

    #[test]
    fn shortest_path_2() {
        // Arrange
        let g = create_graph_2();

        // Act
        let (distance, shortest_path) = g.shortest_path1(0, 7);

        // Assert
        assert_eq!(distance, 7);
        assert_eq!(shortest_path, vec![0, 2, 5, 6, 7]);
    }

    #[test]
    fn shortest_path_3() {
        // Arrange
        let mut g = Graph::new();
        g.add_vertex(0);
        g.add_vertex(1);
        g.add_vertex(2);

        // (0, 1, 4)
        g.add_undirected_edge(0, 1, 4);

        // (0, 2, 1)
        g.add_undirected_edge(0, 2, 1);

        // (2, 1, 1)
        g.add_undirected_edge(2, 1, 1);

        // Act
        let (distance, shortest_path) = g.shortest_path1(0, 1);

        // Assert
        assert_eq!(distance, 2);
        assert_eq!(shortest_path, vec![0, 2, 1]);
    }

    #[test]
    fn shortest_path_4() {
        // Arrange
        let g = create_graph_3();

        // Act
        let (distance, shortest_path) = g.shortest_path2(0, 6);

        // Assert
        assert_eq!(distance, 25);
        assert_eq!(shortest_path, vec![0, 4, 5, 2, 6]);
    }
}
