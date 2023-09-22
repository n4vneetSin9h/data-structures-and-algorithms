data class Edge<T>(val destination: GraphNode<T>, val weight: Double?)

class GraphNode<T>(val value: T) {
    val edges: MutableList<Edge<T>> = mutableListOf()
}

class Graph<T> {
    private val nodes: MutableList<GraphNode<T>> = mutableListOf()

    // MARK: - Node Operations

    fun addNode(value: T) {
        val newNode = GraphNode(value)
        nodes.add(newNode)
    }

    fun removeNode(value: T) {
        nodes.removeIf { it.value == value }
        nodes.forEach { it.edges.removeIf { edge -> edge.destination.value == value } }
    }

    // MARK: - Edge Operations

    fun addEdge(sourceValue: T, destinationValue: T, weight: Double? = null) {
        val sourceNode = getNode(sourceValue)
        val destinationNode = getNode(destinationValue)

        if (sourceNode != null && destinationNode != null) {
            val edge = Edge(destinationNode, weight)
            sourceNode.edges.add(edge)
        }
    }

    fun removeEdge(sourceValue: T, destinationValue: T) {
        val sourceNode = getNode(sourceValue)
        sourceNode?.edges?.removeIf { it.destination.value == destinationValue }
    }

    // MARK: - Search

    fun depthFirstSearch(startValue: T): List<T> {
        val startNode = getNode(startValue)
        val visited = mutableSetOf<T>()
        val result = mutableListOf<T>()

        startNode?.let { depthFirstSearchRec(it, visited, result) }

        return result
    }

    private fun depthFirstSearchRec(node: GraphNode<T>, visited: MutableSet<T>, result: MutableList<T>) {
        if (node.value in visited) return

        visited.add(node.value)
        result.add(node.value)

        node.edges.forEach { edge ->
            depthFirstSearchRec(edge.destination, visited, result)
        }
    }

    fun breadthFirstSearch(startValue: T): List<T> {
        val startNode = getNode(startValue)
        val visited = mutableSetOf<T>()
        val result = mutableListOf<T>()
        val queue: MutableList<GraphNode<T>> = mutableListOf()

        startNode?.let { queue.add(it) }

        while (queue.isNotEmpty()) {
            val currentNode = queue.removeAt(0)

            if (currentNode.value in visited) continue

            visited.add(currentNode.value)
            result.add(currentNode.value)

            queue.addAll(currentNode.edges.map { it.destination })
        }

        return result
    }

    // MARK: - Utility

    private fun getNode(value: T): GraphNode<T>? {
        return nodes.find { it.value == value }
    }

    // MARK: - Check Graph Properties

    fun isCyclic(): Boolean {
        val visited = mutableSetOf<T>()
        val stack = mutableSetOf<T>()

        for (node in nodes) {
            if (isCyclicHelper(node, visited, stack)) {
                return true
            }
        }

        return false
    }

    private fun isCyclicHelper(node: GraphNode<T>, visited: MutableSet<T>, stack: MutableSet<T>): Boolean {
        if (node.value in stack) {
            return true
        }

        if (node.value in visited) {
            return false
        }

        visited.add(node.value)
        stack.add(node.value)

        for (edge in node.edges) {
            if (isCyclicHelper(edge.destination, visited, stack)) {
                return true
            }
        }

        stack.remove(node.value)
        return false
    }

    // MARK: - Shortest Path

    fun shortestPath(startValue: T, endValue: T): List<T> {
        val startNode = getNode(startValue)
        val endNode = getNode(endValue)
        val distances = mutableMapOf<T, Double>()
        val previousNodes = mutableMapOf<T, T>()
        val unvisited = nodes.toMutableSet()

        startNode?.let { distances[it.value] = 0.0 }

        while (unvisited.isNotEmpty()) {
            val currentNode = unvisited.minByOrNull { distances.getOrDefault(it.value, Double.POSITIVE_INFINITY) }
            unvisited.remove(currentNode)

            for (edge in currentNode?.edges.orEmpty()) {
                val distance = (distances[currentNode?.value] ?: Double.POSITIVE_INFINITY) + (edge.weight ?: 0.0)
                if (distance < (distances[edge.destination.value] ?: Double.POSITIVE_INFINITY)) {
                    distances[edge.destination.value] = distance
                    previousNodes[edge.destination.value] = currentNode?.value
                }
            }
        }

        val shortestPath = mutableListOf<T>()
        var current = endValue
        while (current != startValue) {
            val previous = previousNodes[current]
            if (previous != null) {
                shortestPath.add(0, previous)
                current = previous
            } else {
                return emptyList() // No path exists
            }
        }
        shortestPath.add(0, startValue)
        return shortestPath
    }

    // ... Add more graph operations as needed ...
}

fun main() {
    val graph = Graph<Int>()

    graph.addNode(1)
    graph.addNode(2)
    graph.addNode(3)

    graph.addEdge(1, 2)
    graph.addEdge(2, 3)
    graph.addEdge(3, 1)

    println("Is cyclic: ${graph.isCyclic()}")
    println("DFS result: ${graph.depthFirstSearch(1)}")
    println("BFS result: ${graph.breadthFirstSearch(1)}")
    println("Shortest path from 1 to 3: ${graph.shortestPath(1, 3)}")
}
