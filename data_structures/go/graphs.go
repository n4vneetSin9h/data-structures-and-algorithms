package main

import (
	"fmt"
	"math"
)

type Edge struct {
	destination *GraphNode
	weight      *float64
}

type GraphNode struct {
	value int
	edges []*Edge
}

type Graph struct {
	nodes []*GraphNode
}

// NewGraphNode creates a new graph node with the given value.
func NewGraphNode(value int) *GraphNode {
	return &GraphNode{
		value: value,
		edges: []*Edge{},
	}
}

// NewGraph creates a new graph.
func NewGraph() *Graph {
	return &Graph{
		nodes: []*GraphNode{},
	}
}

// AddNode adds a node to the graph.
func (g *Graph) AddNode(value int) {
	node := NewGraphNode(value)
	g.nodes = append(g.nodes, node)
}

// RemoveNode removes a node from the graph.
func (g *Graph) RemoveNode(value int) {
	var newNodes []*GraphNode

	for _, node := range g.nodes {
		if node.value != value {
			newNodes = append(newNodes, node)
		}
	}

	g.nodes = newNodes

	for _, node := range g.nodes {
		var newEdges []*Edge

		for _, edge := range node.edges {
			if edge.destination.value != value {
				newEdges = append(newEdges, edge)
			}
		}

		node.edges = newEdges
	}
}

// AddEdge adds an edge between two nodes in the graph.
func (g *Graph) AddEdge(sourceValue, destinationValue int, weight *float64) {
	sourceNode := g.getNode(sourceValue)
	destinationNode := g.getNode(destinationValue)

	if sourceNode != nil && destinationNode != nil {
		edge := &Edge{
			destination: destinationNode,
			weight:      weight,
		}
		sourceNode.edges = append(sourceNode.edges, edge)
	}
}

// RemoveEdge removes an edge between two nodes in the graph.
func (g *Graph) RemoveEdge(sourceValue, destinationValue int) {
	sourceNode := g.getNode(sourceValue)

	if sourceNode != nil {
		var newEdges []*Edge

		for _, edge := range sourceNode.edges {
			if edge.destination.value != destinationValue {
				newEdges = append(newEdges, edge)
			}
		}

		sourceNode.edges = newEdges
	}
}

// DepthFirstSearch performs a depth-first search starting from a specific node value.
func (g *Graph) DepthFirstSearch(startValue int) []int {
	startNode := g.getNode(startValue)
	visited := make(map[int]bool)
	var result []int

	if startNode != nil {
		g.depthFirstSearchRec(startNode, visited, &result)
	}

	return result
}

func (g *Graph) depthFirstSearchRec(node *GraphNode, visited map[int]bool, result *[]int) {
	if visited[node.value] {
		return
	}

	visited[node.value] = true
	*result = append(*result, node.value)

	for _, edge := range node.edges {
		g.depthFirstSearchRec(edge.destination, visited, result)
	}
}

// BreadthFirstSearch performs a breadth-first search starting from a specific node value.
func (g *Graph) BreadthFirstSearch(startValue int) []int {
	startNode := g.getNode(startValue)
	visited := make(map[int]bool)
	var result []int
	var queue []*GraphNode

	if startNode != nil {
		queue = append(queue, startNode)
	}

	for len(queue) > 0 {
		currentNode := queue[0]
		queue = queue[1:]

		if visited[currentNode.value] {
			continue
		}

		visited[currentNode.value] = true
		result = append(result, currentNode.value)

		for _, edge := range currentNode.edges {
			queue = append(queue, edge.destination)
		}
	}

	return result
}

// Utility function to get a node by its value.
func (g *Graph) getNode(value int) *GraphNode {
	for _, node := range g.nodes {
		if node.value == value {
			return node
		}
	}
	return nil
}

// IsCyclic checks if the graph is cyclic.
func (g *Graph) IsCyclic() bool {
	visited := make(map[int]bool)
	stack := make(map[int]bool)

	for _, node := range g.nodes {
		if g.isCyclicHelper(node, visited, stack) {
			return true
		}
	}

	return false
}

func (g *Graph) isCyclicHelper(node *GraphNode, visited, stack map[int]bool) bool {
	if stack[node.value] {
		return true
	}

	if visited[node.value] {
		return false
	}

	visited[node.value] = true
	stack[node.value] = true

	for _, edge := range node.edges {
		if g.isCyclicHelper(edge.destination, visited, stack) {
			return true
		}
	}

	stack[node.value] = false

	return false
}

func edgeWeightOrZero(weight *float64) float64 {
	if weight != nil {
		return *weight
	}
	return 0
}

// ShortestPath finds the shortest path between two nodes using Dijkstra's algorithm.
func (g *Graph) ShortestPath(startValue, endValue int) []int {
	startNode := g.getNode(startValue)
	endNode := g.getNode(endValue)

	if startNode == nil || endNode == nil {
		return nil
	}

	distances := make(map[int]float64)
	previousNodes := make(map[int]int)
	unvisited := make(map[int]bool)

	for _, node := range g.nodes {
		distances[node.value] = math.Inf(1)
		previousNodes[node.value] = -1
		unvisited[node.value] = true
	}

	distances[startNode.value] = 0

	for len(unvisited) > 0 {
		minNodeValue := g.getMinDistanceNode(distances, unvisited)
		if minNodeValue == -1 {
			break
		}

		minNode := g.getNode(minNodeValue)
		delete(unvisited, minNodeValue)

		for _, edge := range minNode.edges {
			neighborValue := edge.destination.value
			alt := distances[minNodeValue] + edgeWeightOrZero(edge.weight)

			if alt < distances[neighborValue] {
				distances[neighborValue] = alt
				previousNodes[neighborValue] = minNodeValue
			}
		}
	}

	// Build the path from endNode to startNode
	path := []int{endValue}
	current := endValue
	for previousNodes[current] != -1 {
		current = previousNodes[current]
		path = append([]int{current}, path...)
	}

	return path
}

func (g *Graph) getMinDistanceNode(distances map[int]float64, unvisited map[int]bool) int {
	minDist := math.Inf(1)
	minNode := -1

	for nodeValue, dist := range distances {
		if unvisited[nodeValue] && dist < minDist {
			minDist = dist
			minNode = nodeValue
		}
	}

	return minNode
}

// printGraph prints the graph in adjacency list representation.
func (g *Graph) printGraph() {
	for _, node := range g.nodes {
		fmt.Printf("Node %d:", node.value)
		for _, edge := range node.edges {
			fmt.Printf(" -> %d", edge.destination.value)
		}
		fmt.Println()
	}
}
