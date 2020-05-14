use std::collections::HashMap;

#[derive(PartialEq, Debug)]
struct WeightedEdge(u64, u64, i64);

struct Path(Vec<WeightedEdge>);

pub struct Graph {
    pub adjacency_list: HashMap<u64, Vec<(u64, i64)>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: u64) {
        let mut adj_list = self.adjacency_list.get_mut(&vertex);
        if let None = adj_list {
            self.adjacency_list.insert(vertex, vec![]);
        }
    }

    pub fn add_undirected_edge(&mut self, from_vertex: u64, to_vertex: u64, weight: i64) {
        self.add_directed_edge(from_vertex, to_vertex, weight);
        self.add_directed_edge(to_vertex, from_vertex, weight);
    }

    pub fn add_directed_edge(&mut self, from_vertex: u64, to_vertex: u64, weight: i64) {
        let mut adj_list = self.adjacency_list.get_mut(&from_vertex).unwrap();
        adj_list.push((to_vertex, weight));
    }
}
