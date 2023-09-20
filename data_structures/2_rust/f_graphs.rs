use std::collections::{HashSet, HashMap, VecDeque};

struct Edge<T> {
    destination: GraphNode<T>,
    weight: Option<f64>,
}

struct GraphNode<T> {
    value: T,
    edges: Vec<Edge<T>>,
}

struct Graph<T> {
    nodes: Vec<GraphNode<T>>,
}

impl<T: PartialEq> Graph<T> {
    // MARK: - Node Operations

    /// Add a node to the graph.
    fn add_node(&mut self, value: T) {
        let new_node = GraphNode {
            value,
            edges: Vec::new(),
        };
        self.nodes.push(new_node);
    }

    /// Remove a node from the graph.
    fn remove_node(&mut self, value: T) {
        self.nodes.retain(|node| node.value != value);
        for node in &mut self.nodes {
            node.edges.retain(|edge| edge.destination.value != value);
        }
    }

    // MARK: - Edge Operations

    /// Add an edge between two nodes in the graph.
    fn add_edge(&mut self, source_value: T, destination_value: T, weight: Option<f64>) {
        if let Some(source_node) = self.get_node_mut(source_value) {
            if let Some(destination_node) = self.get_node(destination_value) {
                let edge = Edge {
                    destination: destination_node,
                    weight,
                };
                source_node.edges.push(edge);
            }
        }
    }

    /// Remove an edge between two nodes in the graph.
    fn remove_edge(&mut self, source_value: T, destination_value: T) {
        if let Some(source_node) = self.get_node_mut(source_value) {
            source_node.edges.retain(|edge| edge.destination.value != destination_value);
        }
    }

    // MARK: - Search

    /// Depth-First Search traversal starting from a specific node value.
    fn depth_first_search(&self, start_value: T) -> Vec<T> {
        let start_node = match self.get_node(start_value) {
            Some(node) => node,
            None => return Vec::new(),
        };
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        self.depth_first_search_rec(&start_node, &mut visited, &mut result);
        result
    }

    fn depth_first_search_rec(&self, node: &GraphNode<T>, visited: &mut HashSet<T>, result: &mut Vec<T>) {
        if visited.contains(&node.value) {
            return;
        }
        visited.insert(node.value.clone());
        result.push(node.value.clone());

        for edge in &node.edges {
            self.depth_first_search_rec(&edge.destination, visited, result);
        }
    }

    /// Breadth-First Search traversal starting from a specific node value.
    fn breadth_first_search(&self, start_value: T) -> Vec<T> {
        let start_node = match self.get_node(start_value) {
            Some(node) => node,
            None => return Vec::new(),
        };
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        queue.push_back(start_node);

        while let Some(current_node) = queue.pop_front() {
            if visited.contains(&current_node.value) {
                continue;
            }

            visited.insert(current_node.value.clone());
            result.push(current_node.value.clone());

            for edge in &current_node.edges {
                queue.push_back(&edge.destination);
            }
        }

        result
    }

    // MARK: - Utility

    fn get_node(&self, value: T) -> Option<&GraphNode<T>> {
        self.nodes.iter().find(|node| node.value == value)
    }

    fn get_node_mut(&mut self, value: T) -> Option<&mut GraphNode<T>> {
        self.nodes.iter_mut().find(|node| node.value == value)
    }

    // MARK: - Check Graph Properties

    /// Check if the graph is cyclic (contains cycles).
    fn is_cyclic(&self) -> bool {
        let mut visited = HashSet::new();
        let mut stack = HashSet::new();

        for node in &self.nodes {
            if self.is_cyclic_helper(node, &mut visited, &mut stack) {
                return true;
            }
        }

        false
    }

    fn is_cyclic_helper(&self, node: &GraphNode<T>, visited: &mut HashSet<T>, stack: &mut HashSet<T>) -> bool {
        if stack.contains(&node.value) {
            return true;
        }

        if visited.contains(&node.value) {
            return false;
        }

        visited.insert(node.value.clone());
        stack.insert(node.value.clone());

        for edge in &node.edges {
            if self.is_cyclic_helper(&edge.destination, visited, stack) {
                return true;
            }
        }

        stack.remove(&node.value);
        false
    }

    // MARK: - Shortest Path

    /// Find the shortest path between two nodes using Dijkstra's algorithm.
    fn shortest_path(&self, start_value: T, end_value: T) -> Vec<T> {
        let start_node = match self.get_node(start_value) {
            Some(node) => node,
            None => return Vec::new(),
        };
        let end_node = match self.get_node(end_value) {
            Some(node) => node,
            None => return Vec::new(),
        };

        let mut distances: HashMap<T, f64> = HashMap::new();
        let mut previous_nodes: HashMap<T, T> = HashMap::new();
        let mut unvisited: HashSet<GraphNode<T>> = self.nodes.iter().cloned().collect();
        distances.insert(start_value, 0.0);

        while let Some(current_node) = unvisited.iter().min_by_key(|node| {
            *distances.get(&node.value).unwrap_or(&f64::INFINITY) as i64
        }) {
            unvisited.remove(current_node);

            for edge in &current_node.edges {
                let distance = (distances.get(&current_node.value).unwrap_or(&f64::INFINITY)) + (edge.weight.unwrap_or(0.0));
                if distance < *distances.get(&edge.destination.value).unwrap_or(&f64::INFINITY) {
                    distances.insert(edge.destination.value.clone(), distance);
                    previous_nodes.insert(edge.destination.value.clone(), current_node.value.clone());
                }
            }
        }

        let mut shortest_path: Vec<T> = Vec::new();
        let mut current = end_value.clone();
        while current != start_value {
            if let Some(previous) = previous_nodes.get(&current) {
                shortest_path.insert(0, previous.clone());
                current = previous.clone();
            } else {
                return Vec::new(); // No path exists
            }
        }
        shortest_path.insert(0, start_value);
        shortest_path
    }

    // ... Add more graph operations as needed ...
}

fn main() {
    let mut graph = Graph::<i32> {
        nodes: Vec::new(),
    };

    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(3);

    graph.add_edge(1, 2, None);
    graph.add_edge(2, 3, None);
    graph.add_edge(3, 1, None);

    println!("Is cyclic: {}", graph.is_cyclic());

    let dfs_result = graph.depth_first_search(1);
    println!("DFS result: {:?}", dfs_result);

    let bfs_result = graph.breadth_first_search(1);
    println!("BFS result: {:?}", bfs_result);

    let shortest_path_result = graph.shortest_path(1, 3);
    println!("Shortest path from 1 to 3: {:?}", shortest_path_result);
}
