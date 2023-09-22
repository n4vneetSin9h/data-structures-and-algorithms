#include <iostream>
#include <vector>
#include <unordered_set>
#include <unordered_map>
#include <queue>

template <typename T>
struct Edge {
    GraphNode<T>* destination;
    double weight;
};

template <typename T>
class GraphNode {
public:
    T value;
    std::vector<Edge<T>> edges;

    GraphNode(T val) : value(val) {}
};

template <typename T>
class Graph {
private:
    std::vector<GraphNode<T>*> nodes;

public:
    // MARK: - Node Operations

    /// Add a node to the graph.
    void addNode(T value) {
        GraphNode<T>* newNode = new GraphNode<T>(value);
        nodes.push_back(newNode);
    }

    /// Remove a node from the graph.
    void removeNode(T value) {
        nodes.erase(
            std::remove_if(nodes.begin(), nodes.end(),
                [value](const GraphNode<T>* node) { return node->value == value; }),
            nodes.end()
        );

        for (auto& node : nodes) {
            node->edges.erase(
                std::remove_if(node->edges.begin(), node->edges.end(),
                    [value](const Edge<T>& edge) { return edge.destination->value == value; }),
                node->edges.end()
            );
        }
    }

    // MARK: - Edge Operations

    /// Add an edge between two nodes in the graph.
    void addEdge(T sourceValue, T destinationValue, double weight = 0) {
        auto sourceNode = getNode(sourceValue);
        auto destinationNode = getNode(destinationValue);

        if (sourceNode && destinationNode) {
            Edge<T> edge = { destinationNode, weight };
            sourceNode->edges.push_back(edge);
        }
    }

    /// Remove an edge between two nodes in the graph.
    void removeEdge(T sourceValue, T destinationValue) {
        auto sourceNode = getNode(sourceValue);

        if (sourceNode) {
            sourceNode->edges.erase(
                std::remove_if(sourceNode->edges.begin(), sourceNode->edges.end(),
                    [destinationValue](const Edge<T>& edge) { return edge.destination->value == destinationValue; }),
                sourceNode->edges.end()
            );
        }
    }

    // MARK: - Search

    /// Depth-First Search traversal starting from a specific node value.
    std::vector<T> depthFirstSearch(T startValue) {
        std::unordered_set<T> visited;
        std::vector<T> result;
        auto startNode = getNode(startValue);

        if (startNode)
            depthFirstSearchRec(startNode, visited, result);

        return result;
    }

    void depthFirstSearchRec(GraphNode<T>* node, std::unordered_set<T>& visited, std::vector<T>& result) {
        if (visited.find(node->value) != visited.end())
            return;

        visited.insert(node->value);
        result.push_back(node->value);

        for (const auto& edge : node->edges)
            depthFirstSearchRec(edge.destination, visited, result);
    }

    /// Breadth-First Search traversal starting from a specific node value.
    std::vector<T> breadthFirstSearch(T startValue) {
        std::unordered_set<T> visited;
        std::vector<T> result;
        std::queue<GraphNode<T>*> queue;

        auto startNode = getNode(startValue);

        if (!startNode)
            return result;

        queue.push(startNode);
        visited.insert(startValue);

        while (!queue.empty()) {
            auto currentNode = queue.front();
            queue.pop();

            result.push_back(currentNode->value);

            for (const auto& edge : currentNode->edges) {
                if (visited.find(edge.destination->value) == visited.end()) {
                    visited.insert(edge.destination->value);
                    queue.push(edge.destination);
                }
            }
        }

        return result;
    }

    // MARK: - Utility

    GraphNode<T>* getNode(T value) {
        auto it = std::find_if(nodes.begin(), nodes.end(), [value](const GraphNode<T>* node) { return node->value == value; });
        return (it != nodes.end()) ? *it : nullptr;
    }

    // MARK: - Check Graph Properties

    /// Check if the graph is cyclic (contains cycles).
    bool isCyclic() {
        std::unordered_set<T> visited;
        std::unordered_set<T> stack;

        for (const auto& node : nodes) {
            if (isCyclicHelper(node, visited, stack))
                return true;
        }

        return false;
    }

    bool isCyclicHelper(GraphNode<T>* node, std::unordered_set<T>& visited, std::unordered_set<T>& stack) {
        if (stack.find(node->value) != stack.end())
            return true;

        if (visited.find(node->value) != visited.end())
            return false;

        visited.insert(node->value);
        stack.insert(node->value);

        for (const auto& edge : node->edges) {
            if (isCyclicHelper(edge.destination, visited, stack))
                return true;
        }

        stack.erase(node->value);
        return false;
    }

    // MARK: - Shortest Path

    /// Find the shortest path between two nodes using Dijkstra's algorithm.
    std::vector<T> shortestPath(T startValue, T endValue) {
        std::unordered_map<T, double> distances; // Store distances from startNode
        std::unordered_map<T, T> previousNodes;   // Store the previous node in the shortest path
        std::unordered_set<GraphNode<T>*> unvisited;
        std::vector<T> shortestPath;

        for (auto& node : nodes) {
            unvisited.insert(node);
            distances[node->value] = std::numeric_limits<double>::infinity();
        }

        distances[startValue] = 0;

        while (!unvisited.empty()) {
            auto currentNode = *std::min_element(unvisited.begin(), unvisited.end(),
                [&distances](GraphNode<T>* a, GraphNode<T>* b) { return distances[a->value] < distances[b->value]; });

            unvisited.erase(currentNode);

            for (const auto& edge : currentNode->edges) {
                double distance = distances[currentNode->value] + edge.weight;
                if (distance < distances[edge.destination->value]) {
                    distances[edge.destination->value] = distance;
                    previousNodes[edge.destination->value] = currentNode->value;
                }
            }
        }

        auto current = endValue;
        while (current != startValue) {
            auto it = previousNodes.find(current);
            if (it == previousNodes.end())
                return {}; // No path exists

            shortestPath.insert(shortestPath.begin(), current);
            current = it->second;
        }

        shortestPath.insert(shortestPath.begin(), startValue);
        return shortestPath;
    }

    // ... Add more graph operations as needed ...
};

int main() {
    Graph<int> graph;

    graph.addNode(1);
    graph.addNode(2);
    graph.addNode(3);

    graph.addEdge(1, 2);
    graph.addEdge(2, 3);
    graph.addEdge(3, 1);

    std::cout << "Is cyclic: " << (graph.isCyclic() ? "true" : "false") << std::endl;

    std::vector<int> dfsResult = graph.depthFirstSearch(1);
    std::cout << "DFS result: ";
    for (int val : dfsResult)
        std::cout << val << " ";
    std::cout << std::endl;

    std::vector<int> bfsResult = graph.breadthFirstSearch(1);
    std::cout << "BFS result: ";
    for (int val : bfsResult)
        std::cout << val << " ";
    std::cout << std::endl;

    std::vector<int> shortestPathResult = graph.shortestPath(1, 3);
    std::cout << "Shortest path from 1 to 3: ";
    for (int val : shortestPathResult)
        std::cout << val << " ";
    std::cout << std::endl;

    return 0;
}
