from typing import TypeVar, List, Optional, Dict, Set

T = TypeVar('T')


class Edge:
    def __init__(self, destination: 'GraphNode', weight: Optional[float] = None):
        self.destination = destination
        self.weight = weight


class GraphNode:
    def __init__(self, value: T):
        self.value = value
        self.edges: List[Edge] = []


class Graph:
    def __init__(self):
        self.nodes: List[GraphNode] = []

    # MARK: - Node Operations

    def add_node(self, value: T):
        new_node = GraphNode(value)
        self.nodes.append(new_node)

    def remove_node(self, value: T):
        self.nodes = [node for node in self.nodes if node.value != value]
        for node in self.nodes:
            node.edges = [edge for edge in node.edges if edge.destination.value != value]

    # MARK: - Edge Operations

    def add_edge(self, source_value: T, destination_value: T, weight: Optional[float] = None):
        source_node = self.get_node(source_value)
        destination_node = self.get_node(destination_value)

        if source_node and destination_node:
            edge = Edge(destination_node, weight)
            source_node.edges.append(edge)

    def remove_edge(self, source_value: T, destination_value: T):
        source_node = self.get_node(source_value)

        if source_node:
            source_node.edges = [edge for edge in source_node.edges if edge.destination.value != destination_value]

    # MARK: - Search

    def depth_first_search(self, start_value: T) -> List[T]:
        start_node = self.get_node(start_value)
        visited = set()
        result = []

        if start_node:
            self._depth_first_search_rec(start_node, visited, result)

        return result

    def _depth_first_search_rec(self, node: GraphNode, visited: Set[T], result: List[T]):
        if node.value in visited:
            return

        visited.add(node.value)
        result.append(node.value)

        for edge in node.edges:
            self._depth_first_search_rec(edge.destination, visited, result)

    def breadth_first_search(self, start_value: T) -> List[T]:
        start_node = self.get_node(start_value)
        visited = set()
        result = []
        queue = [start_node] if start_node else []

        while queue:
            current_node = queue.pop(0)

            if current_node.value in visited:
                continue

            visited.add(current_node.value)
            result.append(current_node.value)

            for edge in current_node.edges:
                queue.append(edge.destination)

        return result

    # MARK: - Utility

    def get_node(self, value: T) -> Optional[GraphNode]:
        for node in self.nodes:
            if node.value == value:
                return node
        return None

    # MARK: - Check Graph Properties

    def is_cyclic(self) -> bool:
        visited = set()
        stack = set()

        for node in self.nodes:
            if self._is_cyclic_helper(node, visited, stack):
                return True

        return False

    def _is_cyclic_helper(self, node: GraphNode, visited: Set[T], stack: Set[T]) -> bool:
        if node.value in stack:
            return True

        if node.value in visited:
            return False

        visited.add(node.value)
        stack.add(node.value)

        for edge in node.edges:
            if self._is_cyclic_helper(edge.destination, visited, stack):
                return True

        stack.remove(node.value)
        return False

    # MARK: - Shortest Path

    def shortest_path(self, start_value: T, end_value: T) -> List[T]:
        start_node = self.get_node(start_value)
        end_node = self.get_node(end_value)
        distances: Dict[T, float] = {}
        previous_nodes: Dict[T, T] = {}
        unvisited: Set[GraphNode] = set(self.nodes)

        if start_node:
            distances[start_node.value] = 0.0

        while unvisited:
            current_node = min(unvisited, key=lambda x: distances.get(x.value, float('inf')))
            unvisited.remove(current_node)

            for edge in current_node.edges:
                distance = distances.get(current_node.value, float('inf')) + (edge.weight or 0.0)
                if distance < distances.get(edge.destination.value, float('inf')):
                    distances[edge.destination.value] = distance
                    previous_nodes[edge.destination.value] = current_node.value

        shortest_path = []
        current = end_value
        while current != start_value:
            previous = previous_nodes.get(current)
            if previous is None:
                return []  # No path exists
            shortest_path.insert(0, previous)
            current = previous

        shortest_path.insert(0, start_value)
        return shortest_path


# Usage
if __name__ == "__main__":
    graph = Graph()

    graph.add_node(1)
    graph.add_node(2)
    graph.add_node(3)

    graph.add_edge(1, 2)
    graph.add_edge(2, 3)
    graph.add_edge(3, 1)

    print("Is cyclic:", graph.is_cyclic())
    print("DFS result:", graph.depth_first_search(1))
    print("BFS result:", graph.breadth_first_search(1))
    print("Shortest path from 1 to 3:", graph.shortest_path(1, 3))
