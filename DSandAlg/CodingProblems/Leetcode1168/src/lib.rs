// Leetcode1168 - Optimize Water Distribution in a Village

mod pq;

use crate::pq::PriorityQueue;
use std::cmp;
use std::collections::{HashMap, HashSet};

struct Graph {
    vertices: HashMap<usize, u64>,
    adjacency_list: HashMap<usize, Vec<(usize, u64)>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            vertices: HashMap::new(),
            adjacency_list: HashMap::new(),
        }
    }

    fn add_vertex(&mut self, vertex: usize, cost: u64) {
        self.vertices.insert(vertex, cost);

        let mut adj_list = self.adjacency_list.get_mut(&vertex);
        if let None = adj_list {
            self.adjacency_list.insert(vertex, vec![]);
        }
    }

    fn add_edge(&mut self, from_vertex: usize, to_vertex: usize, weight: u64) {
        let mut adj_list = self.adjacency_list.get_mut(&from_vertex).unwrap();
        adj_list.push((to_vertex, weight));

        let mut adj_list = self.adjacency_list.get_mut(&to_vertex).unwrap();
        adj_list.push((from_vertex, weight));
    }
}

fn find_min_cost(well_cost: &[u64], pipes: &[(usize, usize, u64)], n: usize) -> u64 {
    // SS: Input: cost for drilling a well per house
    // cost of piping water from one house to the next, (house1, house2, cost)
    // n: number of houses

    // SS: construct graph
    let mut graph = Graph::new();
    for i in 0..n {
        let cost = well_cost[i];
        graph.add_vertex(i, cost);
    }

    for &edge in pipes {
        let (from, to, cost) = edge;
        graph.add_edge(from, to, cost);
    }

    // SS: start with the house that has the smallest cost to drill a well
    let (start_vertex, &min_cost) = well_cost
        .iter()
        .enumerate()
        .min_by_key(|&key| key.1)
        .unwrap();

    let mut min_pq = PriorityQueue::new();
    min_pq.insert(min_cost as i64, start_vertex);

    let mut visited = HashSet::new();
    visited.insert(start_vertex);

    let mut global_cost = 0;

    // SS: similarities to Dijkstra's algorithm,
    // we push/pop each vertex once onto the min-PQ, which is O(|V| log V),
    // and we also iterate over each edge per vertex, adding neighbors, at
    // a cost of O(|E| log V), combined at O((|V| + |E|) log V).

    // SS: I read people talking about MST (minimum spanning tree), need to look at it again
    // once I better understand MSTs...

    while min_pq.is_empty() == false {
        // SS: vertex we are at, with cost so far
        let (cost, vertex) = min_pq.pop();

        // SS: breadth-first, get all neighbors
        let neighbors = graph.adjacency_list.get(&vertex).unwrap();
        for &neighbor in neighbors {
            let (neighbor_vertex, pipe_cost) = neighbor;

            if visited.contains(&neighbor_vertex) {
                continue;
            }
            visited.insert(neighbor_vertex);

            let drill_well_at_house_cost = cost as u64 + well_cost[neighbor_vertex];
            let lay_pipe_to_house_cost = cost as u64 + pipe_cost;
            if drill_well_at_house_cost < lay_pipe_to_house_cost {
                min_pq.insert(drill_well_at_house_cost as i64, neighbor_vertex);
                global_cost = cmp::max(global_cost, drill_well_at_house_cost);
            } else {
                min_pq.insert(lay_pipe_to_house_cost as i64, neighbor_vertex);
                global_cost = cmp::max(global_cost, lay_pipe_to_house_cost);
            }
        }
    }

    global_cost
}

#[cfg(test)]
mod tests {
    use crate::find_min_cost;

    #[test]
    fn test() {
        // Arrange
        let well_cost = [1, 2, 2];
        let pipes = [(0, 1, 1), (1, 2, 1)];

        // Act
        let min_cost = find_min_cost(&well_cost, &pipes, 3);

        // Assert
        assert_eq!(min_cost, 3);
    }
}
