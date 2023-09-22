struct Edge<T> {
    var destination: GraphNode<T>
    var weight: Double?
}

class GraphNode<T: Hashable> {
    var value: T
    var edges: [Edge<T>]

    init(value: T) {
        self.value = value
        self.edges = []
    }
}

class Graph<T: Hashable> {
    private var nodes: [GraphNode<T>]

    init() {
        self.nodes = []
    }

    // MARK: - Node Operations

    /// Add a node to the graph.
    func addNode(value: T) {
        let newNode = GraphNode(value: value)
        nodes.append(newNode)
    }

    /// Remove a node from the graph.
    func removeNode(value: T) {
        nodes.removeAll { $0.value == value }
        for node in nodes {
            node.edges.removeAll { $0.destination.value == value }
        }
    }

    // MARK: - Edge Operations

    /// Add an edge between two nodes in the graph.
    func addEdge(sourceValue: T, destinationValue: T, weight: Double? = nil) {
        guard let sourceNode = getNode(value: sourceValue),
              let destinationNode = getNode(value: destinationValue) else {
            return
        }
        let edge = Edge(destination: destinationNode, weight: weight)
        sourceNode.edges.append(edge)
    }

    /// Remove an edge between two nodes in the graph.
    func removeEdge(sourceValue: T, destinationValue: T) {
        guard let sourceNode = getNode(value: sourceValue) else {
            return
        }
        sourceNode.edges.removeAll { $0.destination.value == destinationValue }
    }

    // MARK: - Search

    /// Depth-First Search traversal starting from a specific node value.
    func depthFirstSearch(startValue: T) -> [T] {
        guard let startNode = getNode(value: startValue) else {
            return []
        }
        var visited: Set<T> = []
        var result: [T] = []
        depthFirstSearchRec(node: startNode, visited: &visited, result: &result)
        return result
    }

    private func depthFirstSearchRec(node: GraphNode<T>, visited: inout Set<T>, result: inout [T]) {
        if visited.contains(node.value) {
            return
        }
        visited.insert(node.value)
        result.append(node.value)
        for edge in node.edges {
            depthFirstSearchRec(node: edge.destination, visited: &visited, result: &result)
        }
    }

    /// Breadth-First Search traversal starting from a specific node value.
    func breadthFirstSearch(startValue: T) -> [T] {
        guard let startNode = getNode(value: startValue) else {
            return []
        }
        var visited: Set<T> = []
        var result: [T] = []
        var queue: [GraphNode<T>] = [startNode]

        while !queue.isEmpty {
            let currentNode = queue.removeFirst()
            if !visited.contains(currentNode.value) {
                visited.insert(currentNode.value)
                result.append(currentNode.value)
                queue.append(contentsOf: currentNode.edges.map { $0.destination })
            }
        }

        return result
    }

    // MARK: - Utility

    private func getNode(value: T) -> GraphNode<T>? {
        return nodes.first { $0.value == value }
    }

    // MARK: - Check Graph Properties

    /// Check if the graph is cyclic (contains cycles).
    func isCyclic() -> Bool {
        var visited: Set<T> = []
        var stack: Set<T> = []

        for node in nodes {
            if isCyclicHelper(node: node, visited: &visited, stack: &stack) {
                return true
            }
        }
        return false
    }

    private func isCyclicHelper(node: GraphNode<T>, visited: inout Set<T>, stack: inout Set<T>) -> Bool {
        if stack.contains(node.value) {
            return true
        }

        if visited.contains(node.value) {
            return false
        }

        visited.insert(node.value)
        stack.insert(node.value)

        for edge in node.edges {
            if isCyclicHelper(node: edge.destination, visited: &visited, stack: &stack) {
                return true
            }
        }

        stack.remove(node.value)
        return false
    }

    // MARK: - Shortest Path

    /// Find the shortest path between two nodes using Dijkstra's algorithm.
    func shortestPath(from startValue: T, to endValue: T) -> [T] {
        guard let startNode = getNode(value: startValue),
              let endNode = getNode(value: endValue) else {
            return []
        }

        var distances: [T: Double] = [:] // Store distances from startNode
        var previousNodes: [T: T] = [:]  // Store the previous node in the shortest path
        var unvisited: Set<GraphNode<T>> = Set(nodes)
        distances[startValue] = 0

        while let currentNode = unvisited.min(by: { distances[$0.value]! < distances[$1.value]! }) {
            unvisited.remove(currentNode)

            for edge in currentNode.edges {
                let distance = (distances[currentNode.value] ?? .infinity) + (edge.weight ?? 0)
                if distance < (distances[edge.destination.value] ?? .infinity) {
                    distances[edge.destination.value] = distance
                    previousNodes[edge.destination.value] = currentNode.value
                }
            }
        }

        var shortestPath: [T] = []
        var current = endValue
        while current != startValue {
            guard let previous = previousNodes[current] else {
                return []  // No path exists
            }
            shortestPath.insert(current, at: 0)
            current = previous
        }
        shortestPath.insert(startValue, at: 0)
        return shortestPath
    }

    // ... Add more graph operations as needed ...
}
