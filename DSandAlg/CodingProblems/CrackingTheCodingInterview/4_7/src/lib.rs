// Cracking the Coding Interview
// 6th ed, p. 109, ex. 4.7

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

    fn traverse(&self, node: u64) -> Vec<u64> {
        let mut result = vec![];
        let mut visited = HashSet::new();
        self.traverse_recursive(node, &mut visited, &mut result);
        result
    }

    fn traverse_recursive(&self, node: u64, visited: &mut HashSet<u64>, result: &mut Vec<u64>) {
        // SS: post-order depth-first traversal
        if visited.contains(&node) == false {
            visited.insert(node);
            for (vertex, _) in &self.adjacency_list[&node] {
                self.traverse_recursive(*vertex, visited, result);
            }
            result.push(node);
        }
    }
}

fn find_index(projects: &[&str], project: &str) -> u64 {
    projects.iter().position(|&p| p == project).unwrap() as u64
}

fn find_project_build_order<'a>(
    projects: &[&'a str],
    dependencies: &[(&str, &str)],
) -> Vec<&'a str> {
    let mut orphans = HashSet::new();
    let mut is_dependent = HashSet::new();
    let mut graph = Graph::new();
    projects.iter().for_each(|&p| {
        orphans.insert(p);
        is_dependent.insert(p);
        graph.add_vertex(find_index(projects, p));
    });

    for &(p1, p2) in dependencies {
        // SS: p2 depends on p1
        let p1_idx = find_index(projects, p1);
        let p2_idx = find_index(projects, p2);
        graph.add_directed_edge(p2_idx, p1_idx, 1);

        orphans.remove(p1);
        orphans.remove(p2);

        // SS: remove project if it is depended on
        is_dependent.remove(p1);
    }

    // SS: note, we are assuming the graph is connected!

    let mut result = vec![];

    // SS: add all orphans
    orphans.iter().for_each(|&orphan| {
        result.push(orphan);
        is_dependent.remove(orphan);
    });

    // SS: There should be only one element left in is_dependent,
    // which is the project that no other project depends on.
    // This is the start project...
    assert_eq!(is_dependent.len(), 1);

    let &root_project = is_dependent.iter().collect::<Vec<_>>()[0];
    let root_node = find_index(projects, root_project);

    let node_result = graph.traverse(root_node);

    // SS: convert to strs...
    node_result
        .into_iter()
        .for_each(|node_index| result.push(projects[node_index as usize]));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_build_order() {
        // Arrange
        let projects = ["a", "b", "c", "d", "e", "f"];
        let dependencies = [("a", "d"), ("f", "b"), ("b", "d"), ("f", "a"), ("d", "c")];

        // Act
        let build_order = find_project_build_order(&projects, &dependencies);

        // Assert
        assert_eq!(build_order, vec!["e", "f", "a", "b", "d", "c"]);
    }
}
