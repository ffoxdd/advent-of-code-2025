use std::collections::VecDeque;
use itertools::Itertools;

enum Direction {
    Forward,
    Backward,
}

pub struct DirectedGraph {
    nodes: Vec<String>,
    adjacency: Vec<Vec<usize>>,
    topological_order: Vec<usize>,
}

impl DirectedGraph {
    pub fn new(nodes: Vec<String>, edges: Vec<(usize, usize)>) -> Self {
        let adjacency = Self::adjacency(nodes.len(), &edges);
        let topological_order = Self::topological_order(nodes.len(), &edges, &adjacency);

        Self { nodes, adjacency, topological_order }
    }

    pub fn paths_between(&self, from_str: &str, to_str: &str) -> Result<u64, String> {
        let from = self.node_index(from_str)?;
        let to = self.node_index(to_str)?;

        let path_counts = self.path_counts(from, to, Direction::Forward);
        Ok(path_counts[to])
    }

    pub fn paths_between_including(&self, from_str: &str, to_str: &str, including_strs: &Vec<&str>) -> Result<u64, String> {
        if including_strs.is_empty() {
            return self.paths_between(from_str, to_str);
        }

        if !including_strs.iter().all_unique() {
            return Err("Including nodes must be unique".to_string());
        }

        let from = self.node_index(from_str)?;
        let to = self.node_index(to_str)?;
        let including = self.node_indices(including_strs)?;

        let forward_path_counts = self.path_counts(from, to, Direction::Forward);
        let backward_path_counts = self.path_counts(from, to, Direction::Backward);

        let from_path_counts: Vec<u64> = including.iter()
            .map(|&including_node| forward_path_counts[including_node])
            .collect();

        let to_path_counts: Vec<u64> = including.iter()
            .map(|&including_node| backward_path_counts[including_node])
            .collect();

        let between_path_counts: Vec<Vec<u64>> = including.iter()
            .map(|&including_node| {
                let path_counts = self.path_counts(including_node, to, Direction::Forward);
                including.iter().map(|&other_including_node| path_counts[other_including_node]).collect()
            }).collect();

        // NOTE: .permutations() won't work for large including_strs.len()
        let result = (0..including.len()).permutations(including.len()).map(|permutation| {
            from_path_counts[permutation[0]] *
            Self::tour_weight(&between_path_counts, &permutation) *
            to_path_counts[permutation[permutation.len() - 1]]
        }).sum::<u64>();

        Ok(result)
    }

    fn path_counts(&self, from: usize, to: usize, direction: Direction) -> Vec<u64> {
        let nodes = self.nodes_between(from, to);
        let mut path_counts = vec![0; self.nodes.len()];

        if let (_, Some(0)) = nodes.size_hint() {
            return path_counts;
        }

        match direction {
            Direction::Forward => self.path_counts_forwards(&mut path_counts, from, nodes),
            Direction::Backward => self.path_counts_backwards(&mut path_counts, to, nodes),
        }

        path_counts
    }

    fn path_counts_forwards(&self, path_counts: &mut Vec<u64>, from: usize, nodes: impl Iterator<Item = usize>) {
        path_counts[from] = 1;

        for node in nodes {
            for &child in &self.adjacency[node] {
                path_counts[child] += path_counts[node];
            }
        }
    }

    fn path_counts_backwards(&self, path_counts: &mut Vec<u64>, to: usize, nodes: impl DoubleEndedIterator<Item = usize>) {
        path_counts[to] = 1;

        for node in nodes.rev() {
            for &child in &self.adjacency[node] {
                path_counts[node] += path_counts[child];
            }
        }
    }

    fn topological_order(node_count: usize, edges: &[(usize, usize)], adjacency: &[Vec<usize>]) -> Vec<usize> {
        let mut indegrees = Self::indegrees(node_count, edges);
        let mut queue = VecDeque::new();

        indegrees.iter().enumerate()
            .filter(|(_, indegree)| **indegree == 0)
            .for_each(|(i, _)| queue.push_back(i));

        let mut sorted = Vec::with_capacity(node_count);

        while let Some(node) = queue.pop_front() {
            sorted.push(node);

            for &neighbor in &adjacency[node] {
                indegrees[neighbor] -= 1;

                if indegrees[neighbor] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }

        sorted
    }

    fn indegrees(node_count: usize, edges: &[(usize, usize)]) -> Vec<usize> {
        let mut indegrees = vec![0; node_count];

        for (_, to) in edges {
            indegrees[*to] += 1;
        }

        indegrees
    }

    fn nodes_between(&self, from: usize, to: usize) -> impl DoubleEndedIterator<Item = usize> + '_ {
        let from_pos = self.topological_order.iter().position(|&n| n == from).unwrap();
        let to_pos = self.topological_order.iter().position(|&n| n == to).unwrap();

        if from_pos > to_pos {
            return [].iter().copied();
        }

        self.topological_order[from_pos..=to_pos].iter().copied()
    }

    fn node_index(&self, node: &str) -> Result<usize, String> {
        self.nodes.iter()
            .position(|n| n == node)
            .ok_or_else(|| format!("Node '{}' not found", node))
    }

    fn node_indices(&self, nodes: impl IntoIterator<Item = impl AsRef<str>>) -> Result<Vec<usize>, String> {
        nodes.into_iter()
            .map(|node| self.node_index(node.as_ref()))
            .collect()
    }

    fn tour_weight(weight_matrix: &[Vec<u64>], tour: &[usize]) -> u64 {
        tour.windows(2)
            .map(|w| weight_matrix[w[0]][w[1]])
            .product()
    }

    fn adjacency(node_count: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
        let mut adjacency = vec![Vec::new(); node_count];

        for (from, to) in edges {
            adjacency[*from].push(*to);
        }

        adjacency
    }
}

impl TryFrom<&Vec<String>> for DirectedGraph {
    type Error = String;

    fn try_from(value: &Vec<String>) -> Result<Self, Self::Error> {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        for line in value {
            let (node, edges_string) = line.split_once(": ")
                .ok_or_else(|| format!("Invalid line: {}", line))?;

            let node = node.trim();

            let from_index = nodes.iter()
                .position(|n| *n == node)
                .unwrap_or_else(|| {
                    nodes.push(node.to_string());
                    nodes.len() - 1
                });

            let to_nodes: Vec<&str> = edges_string
                .split(' ')
                .map(|s| s.trim())
                .collect();

            for to_node in to_nodes {
                let to_index = nodes.iter()
                    .position(|n| *n == to_node)
                    .unwrap_or_else(|| {
                        nodes.push(to_node.to_string());
                        nodes.len() - 1
                    });

                edges.push((from_index, to_index));
            }
        }

        if !nodes.iter().all_unique() {
            return Err("Nodes must be unique".to_string());
        }

        Ok(Self::new(nodes, edges))
    }
}