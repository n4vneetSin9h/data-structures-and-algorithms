# Dijkstra's Algorithm

## Description

Dijkstra's algorithm is used to find the shortest path between nodes in a graph, which may represent, for example, road networks. It is a greedy algorithm that selects the smallest weight edge that is connected to the current node at each step.

## Steps

1. Create a set of unvisited nodes, initially containing all nodes in the graph.
2. Assign a tentative distance value to every node, set it to zero for the initial node, and to infinity for all other nodes.
3. Set the initial node as the current node.
4. For the current node, consider all of its neighbors and calculate their tentative distances from the initial node. Update the distance if a shorter path is found.
5. Mark the current node as visited and remove it from the unvisited set.
6. If the destination node has been visited or the smallest tentative distance among the nodes in the unvisited set is infinity (indicating that all remaining nodes are inaccessible), stop.
7. Otherwise, select the unvisited node with the smallest tentative distance as the new current node and go back to step 4.

## Pseudo Code

```plaintext
Dijkstra(graph, startNode):
    create a set of unvisited nodes
    set tentative distances to infinity for all nodes
    set the distance to the startNode to 0

    while unvisited set is not empty:
        current = node with the smallest tentative distance
        mark current as visited
        remove current from the unvisited set

        for neighbor of current:
            calculate tentative distance
            update tentative distance if smaller than current distance
```

# Dijkstra's Algorithm: Time and Space Complexity Analysis

## Time Complexity Analysis

- **Best Case**: O(|V^2|) where V is the number of vertices.

- **Worst Case**: O((|V| + |E|) * log|V|)

- **Average Case**: O((|V| + |E|) * log|V|)

## Space Complexity Analysis

The space complexity of Dijkstra's Algorithm is O(|V|) for the priority queue and visited set.

# Code

## C++

```cpp
#include <iostream>
#include <vector>
#include <queue>
#include <limits.h>

using namespace std;

#define INF INT_MAX

class Graph {
private:
    int V;
    vector<vector<pair<int, int>>> adj;

public:
    Graph(int V);
    void addEdge(int u, int v, int weight);
    void dijkstra(int src);
};

Graph::Graph(int V) {
    this->V = V;
    adj.resize(V);
}

void Graph::addEdge(int u, int v, int weight) {
    adj[u].push_back(make_pair(v, weight));
    adj[v].push_back(make_pair(u, weight));
}

void Graph::dijkstra(int src) {
    priority_queue<pair<int, int>, vector<pair<int, int>>, greater<pair<int, int>>> pq;
    vector<int> dist(V, INF);

    pq.push(make_pair(0, src));
    dist[src] = 0;

    while (!pq.empty()) {
        int u = pq.top().second;
        pq.pop();

        for (const auto& neighbor : adj[u]) {
            int v = neighbor.first;
            int weight = neighbor.second;

            if (dist[u] + weight < dist[v]) {
                dist[v] = dist[u] + weight;
                pq.push(make_pair(dist[v], v));
            }
        }
    }

    cout << "Shortest distances from node " << src << ":\n";
    for (int i = 0; i < V; ++i) {
        cout << "Node " << i << ": " << dist[i] << "\n";
    }
}

int main() {
    int V = 9;
    Graph g(V);

    g.addEdge(0, 1, 4);
    g.addEdge(0, 7, 8);
    g.addEdge(1, 2, 8);
    g.addEdge(1, 7, 11);
    g.addEdge(2, 3, 7);
    g.addEdge(2, 8, 2);
    g.addEdge(2, 5, 4);
    g.addEdge(3, 4, 9);
    g.addEdge(3, 5, 14);
    g.addEdge(4, 5, 10);
    g.addEdge(5, 6, 2);
    g.addEdge(6, 7, 1);
    g.addEdge(6, 8, 6);
    g.addEdge(7, 8, 7);

    g.dijkstra(0);

    return 0;
}
```

## Rust

```rs
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::usize::MAX;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    node: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Graph {
    v: usize,
    adj_list: Vec<Vec<(usize, usize)>>,
}

impl Graph {
    fn new(v: usize) -> Graph {
        Graph {
            v,
            adj_list: vec![Vec::new(); v],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, w: usize) {
        self.adj_list[u].push((v, w));
        self.adj_list[v].push((u, w));
    }

    fn dijkstra(&self, start: usize) -> Vec<usize> {
        let mut dist: Vec<usize> = vec![MAX; self.v];
        dist[start] = 0;

        let mut pq = BinaryHeap::new();
        pq.push(State { cost: 0, node: start });

        while let Some(State { cost, node }) = pq.pop() {
            if cost > dist[node] {
                continue;
            }

            for &(nei, d) in &self.adj_list[node] {
                let new_cost = cost + d;
                if new_cost < dist[nei] {
                    pq.push(State { cost: new_cost, node: nei });
                    dist[nei] = new_cost;
                }
            }
        }

        dist
    }
}

fn main() {
    let v = 9;
    let mut g = Graph::new(v);

    g.add_edge(0, 1, 4);
    g.add_edge(0, 7, 8);
    g.add_edge(1, 2, 8);
    g.add_edge(1, 7, 11);
    g.add_edge(2, 3, 7);
    g.add_edge(2, 8, 2);
    g.add_edge(2, 5, 4);
    g.add_edge(3, 4, 9);
    g.add_edge(3, 5, 14);
    g.add_edge(4, 5, 10);
    g.add_edge(5, 6, 2);
    g.add_edge(6, 7, 1);
    g.add_edge(6, 8, 6);
    g.add_edge(7, 8, 7);

    let start_node = 0;
    let shortest_distances = g.dijkstra(start_node);

    println!("Shortest distances from node {}: {:?}", start_node, shortest_distances);
}
```

## Python

```py
import heapq
import sys

class Graph:
    def __init__(self, vertices):
        self.V = vertices
        self.adj_list = [[] for _ in range(vertices)]

    def add_edge(self, u, v, w):
        self.adj_list[u].append((v, w))
        self.adj_list[v].append((u, w))

    def dijkstra(self, start):
        dist = [float('inf')] * self.V
        dist[start] = 0
        pq = [(0, start)]

        while pq:
            cost, u = heapq.heappop(pq)

            if cost > dist[u]:
                continue

            for v, w in self.adj_list[u]:
                new_cost = cost + w
                if new_cost < dist[v]:
                    dist[v] = new_cost
                    heapq.heappush(pq, (new_cost, v))

        return dist

# Example usage
V = 9
g = Graph(V)

g.add_edge(0, 1, 4)
g.add_edge(0, 7, 8)
g.add_edge(1, 2, 8)
g.add_edge(1, 7, 11)
g.add_edge(2, 3, 7)
g.add_edge(2, 8, 2)
g.add_edge(2, 5, 4)
g.add_edge(3, 4, 9)
g.add_edge(3, 5, 14)
g.add_edge(4, 5, 10)
g.add_edge(5, 6, 2)
g.add_edge(6, 7, 1)
g.add_edge(6, 8, 6)
g.add_edge(7, 8, 7)

start_node = 0
shortest_distances = g.dijkstra(start_node)
print(f"Shortest distances from node {start_node}: {shortest_distances}")
```

## Swift

```swift
import Foundation

struct Edge {
    let vertex: Int
    let weight: Int
}

struct Graph {
    let vertices: Int
    var adjList: [[Edge]]

    init(vertices: Int) {
        self.vertices = vertices
        self.adjList = Array(repeating: [], count: vertices)
    }

    mutating func addEdge(from: Int, to: Int, weight: Int) {
        adjList[from].append(Edge(vertex: to, weight: weight))
        adjList[to].append(Edge(vertex: from, weight: weight))
    }

    func dijkstra(start: Int) -> [Int] {
        var distance = Array(repeating: Int.max, count: vertices)
        distance[start] = 0

        var priorityQueue = PriorityQueue<(vertex: Int, distance: Int)> {
            return $0.distance < $1.distance
        }

        priorityQueue.enqueue((start, 0))

        while !priorityQueue.isEmpty {
            let (currentVertex, currentDistance) = priorityQueue.dequeue()!

            if currentDistance > distance[currentVertex] {
                continue
            }

            for edge in adjList[currentVertex] {
                let newDistance = currentDistance + edge.weight
                if newDistance < distance[edge.vertex] {
                    distance[edge.vertex] = newDistance
                    priorityQueue.enqueue((edge.vertex, newDistance))
                }
            }
        }

        return distance
    }
}

class PriorityQueue<T> {
    private var heap: [T]
    private let comparator: (T, T) -> Bool

    init(comparator: @escaping (T, T) -> Bool) {
        self.heap = []
        self.comparator = comparator
    }

    var isEmpty: Bool {
        return heap.isEmpty
    }

    func enqueue(_ element: T) {
        heap.append(element)
        var currentIndex = heap.count - 1

        while currentIndex > 0 {
            let parentIndex = (currentIndex - 1) / 2
            if comparator(heap[parentIndex], heap[currentIndex]) {
                heap.swapAt(parentIndex, currentIndex)
                currentIndex = parentIndex
            } else {
                break
            }
        }
    }

    func dequeue() -> T? {
        guard !heap.isEmpty else { return nil }

        heap.swapAt(0, heap.count - 1)
        let last = heap.removeLast()

        if !heap.isEmpty {
            heapifyDown(0)
        }

        return last
    }

    private func heapifyDown(_ index: Int) {
        let leftChildIndex = 2 * index + 1
        let rightChildIndex = 2 * index + 2
        var smallestIndex = index

        if leftChildIndex < heap.count && comparator(heap[leftChildIndex], heap[smallestIndex]) {
            smallestIndex = leftChildIndex
        }

        if rightChildIndex < heap.count && comparator(heap[rightChildIndex], heap[smallestIndex]) {
            smallestIndex = rightChildIndex
        }

        if smallestIndex != index {
            heap.swapAt(index, smallestIndex)
            heapifyDown(smallestIndex)
        }
    }
}

// Example usage
var g = Graph(vertices: 9)

g.addEdge(from: 0, to: 1, weight: 4)
g.addEdge(from: 0, to: 7, weight: 8)
g.addEdge(from: 1, to: 2, weight: 8)
g.addEdge(from: 1, to: 7, weight: 11)
g.addEdge(from: 2, to: 3, weight: 7)
g.addEdge(from: 2, to: 8, weight: 2)
g.addEdge(from: 2, to: 5, weight: 4)
g.addEdge(from: 3, to: 4, weight: 9)
g.addEdge(from: 3, to: 5, weight: 14)
g.addEdge(from: 4, to: 5, weight: 10)
g.addEdge(from: 5, to: 6, weight: 2)
g.addEdge(from: 6, to: 7, weight: 1)
g.addEdge(from: 6, to: 8, weight: 6)
g.addEdge(from: 7, to: 8, weight: 7)

let startNode = 0
let shortestDistances = g.dijkstra(start: startNode)
print("Shortest distances from node \(startNode): \(shortestDistances)")
```

## Kotlin

```kotlin
import java.util.*

class Graph(private val V: Int) {
    private val adjList: MutableList<MutableList<Pair<Int, Int>>> = MutableList(V) { mutableListOf() }

    fun addEdge(u: Int, v: Int, w: Int) {
        adjList[u].add(Pair(v, w))
        adjList[v].add(Pair(u, w))
    }

    fun dijkstra(src: Int) {
        val dist = IntArray(V) { Int.MAX_VALUE }
        dist[src] = 0

        val pq = PriorityQueue<Pair<Int, Int>>(compareBy { it.second })
        pq.add(Pair(src, 0))

        while (pq.isNotEmpty()) {
            val (u, d) = pq.poll()

            if (d > dist[u])
                continue

            for ((v, w) in adjList[u]) {
                val newDist = dist[u] + w
                if (newDist < dist[v]) {
                    dist[v] = newDist
                    pq.add(Pair(v, newDist))
                }
            }
        }

        println("Shortest distances from node $src:")
        for (i in 0 until V)
            println("Node $i: ${if (dist[i] == Int.MAX_VALUE) "INF" else dist[i]}")
    }
}

fun main() {
    val V = 9
    val g = Graph(V)

    g.addEdge(0, 1, 4)
    g.addEdge(0, 7, 8)
    g.addEdge(1, 2, 8)
    g.addEdge(1, 7, 11)
    g.addEdge(2, 3, 7)
    g.addEdge(2, 8, 2)
    g.addEdge(2, 5, 4)
    g.addEdge(3, 4, 9)
    g.addEdge(3, 5, 14)
    g.addEdge(4, 5, 10)
    g.addEdge(5, 6, 2)
    g.addEdge(6, 7, 1)
    g.addEdge(6, 8, 6)
    g.addEdge(7, 8, 7)

    g.dijkstra(0)
}
```

## Go

```go
package main

import (
	"container/heap"
	"fmt"
	"math"
)

type Edge struct {
	vertex int
	weight int
}

type Graph struct {
	vertices int
	adjList  map[int][]Edge
}

func NewGraph(vertices int) *Graph {
	return &Graph{
		vertices: vertices,
		adjList:  make(map[int][]Edge),
	}
}

func (g *Graph) addEdge(u, v, w int) {
	g.adjList[u] = append(g.adjList[u], Edge{vertex: v, weight: w})
	g.adjList[v] = append(g.adjList[v], Edge{vertex: u, weight: w})
}

func (g *Graph) dijkstra(start int) []int {
	dist := make([]int, g.vertices)
	for i := range dist {
		dist[i] = math.MaxInt64
	}
	dist[start] = 0

	pq := &PriorityQueue{}
	heap.Init(pq)
	heap.Push(pq, &Item{value: start, priority: 0})

	for pq.Len() > 0 {
		item := heap.Pop(pq).(*Item)
		u := item.value

		for _, edge := range g.adjList[u] {
			v := edge.vertex
			weight := edge.weight
			if dist[u]+weight < dist[v] {
				dist[v] = dist[u] + weight
				heap.Push(pq, &Item{value: v, priority: dist[v]})
			}
		}
	}

	return dist
}

type Item struct {
	value    int
	priority int
	index    int
}

type PriorityQueue []*Item

func (pq PriorityQueue) Len() int {
	return len(pq)
}

func (pq PriorityQueue) Less(i, j int) bool {
	return pq[i].priority < pq[j].priority
}

func (pq PriorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
	pq[i].index = i
	pq[j].index = j
}

func (pq *PriorityQueue) Push(x interface{}) {
	n := len(*pq)
	item := x.(*Item)
	item.index = n
	*pq = append(*pq, item)
}

func (pq *PriorityQueue) Pop() interface{} {
	old := *pq
	n := len(old)
	item := old[n-1]
	item.index = -1
	*pq = old[0 : n-1]
	return item
}

func main() {
	V := 9
	g := NewGraph(V)

	g.addEdge(0, 1, 4)
	g.addEdge(0, 7, 8)
	g.addEdge(1, 2, 8)
	g.addEdge(1, 7, 11)
	g.addEdge(2, 3, 7)
	g.addEdge(2, 8, 2)
	g.addEdge(2, 5, 4)
	g.addEdge(3, 4, 9)
	g.addEdge(3, 5, 14)
	g.addEdge(4, 5, 10)
	g.addEdge(5, 6, 2)
	g.addEdge(6, 7, 1)
	g.addEdge(6, 8, 6)
	g.addEdge(7, 8, 7)

	startNode := 0
	shortestDistances := g.dijkstra(startNode)
	fmt.Printf("Shortest distances from node %d: %v\n", startNode, shortestDistances)
}
```

## Java

```java
import java.util.ArrayList;
import java.util.Arrays;
import java.util.PriorityQueue;

class Graph {
    private int V;
    private ArrayList<ArrayList<Node>> adjList;

    public Graph(int V) {
        this.V = V;
        adjList = new ArrayList<>(V);
        for (int i = 0; i < V; i++) {
            adjList.add(new ArrayList<>());
        }
    }

    public void addEdge(int u, int v, int weight) {
        adjList.get(u).add(new Node(v, weight));
        adjList.get(v).add(new Node(u, weight));
    }

    public void dijkstra(int src) {
        int[] dist = new int[V];
        Arrays.fill(dist, Integer.MAX_VALUE);
        dist[src] = 0;

        PriorityQueue<Node> pq = new PriorityQueue<>((a, b) -> Integer.compare(a.weight, b.weight));
        pq.offer(new Node(src, 0));

        while (!pq.isEmpty()) {
            Node node = pq.poll();
            int u = node.vertex;

            if (node.weight > dist[u]) {
                continue;
            }

            for (Node neighbor : adjList.get(u)) {
                int v = neighbor.vertex;
                int weight = neighbor.weight;

                if (dist[u] + weight < dist[v]) {
                    dist[v] = dist[u] + weight;
                    pq.offer(new Node(v, dist[v]));
                }
            }
        }

        System.out.println("Shortest distances from node " + src + ":");
        for (int i = 0; i < V; i++) {
            System.out.println("Node " + i + ": " + (dist[i] == Integer.MAX_VALUE ? "INF" : dist[i]));
        }
    }

    private static class Node {
        int vertex;
        int weight;

        public Node(int vertex, int weight) {
            this.vertex = vertex;
            this.weight = weight;
        }
    }
}

public class DijkstraAlgorithm {
    public static void main(String[] args) {
        int V = 9;
        Graph g = new Graph(V);

        g.addEdge(0, 1, 4);
        g.addEdge(0, 7, 8);
        g.addEdge(1, 2, 8);
        g.addEdge(1, 7, 11);
        g.addEdge(2, 3, 7);
        g.addEdge(2, 8, 2);
        g.addEdge(2, 5, 4);
        g.addEdge(3, 4, 9);
        g.addEdge(3, 5, 14);
        g.addEdge(4, 5, 10);
        g.addEdge(5, 6, 2);
        g.addEdge(6, 7, 1);
        g.addEdge(6, 8, 6);
        g.addEdge(7, 8, 7);

        int startNode = 0;
        g.dijkstra(startNode);
    }
}
```

## JavaScript

```js
class PriorityQueue {
    constructor() {
        this.heap = [];
    }

    enqueue(value, priority) {
        this.heap.push({ value, priority });
        this.bubbleUp();
    }

    dequeue() {
        const max = this.heap[0];
        const end = this.heap.pop();
        if (this.heap.length > 0) {
            this.heap[0] = end;
            this.sinkDown();
        }
        return max;
    }

    bubbleUp() {
        let index = this.heap.length - 1;
        const element = this.heap[index];
        while (index > 0) {
            let parentIndex = Math.floor((index - 1) / 2);
            let parent = this.heap[parentIndex];
            if (element.priority >= parent.priority)
                break;
            this.heap[parentIndex] = element;
            this.heap[index] = parent;
            index = parentIndex;
        }
    }

    sinkDown() {
        let index = 0;
        const length = this.heap.length;
        const element = this.heap[0];
        while (true) {
            let leftChildIndex = 2 * index + 1;
            let rightChildIndex = 2 * index + 2;
            let leftChild, rightChild;
            let swap = null;

            if (leftChildIndex < length) {
                leftChild = this.heap[leftChildIndex];
                if (leftChild.priority < element.priority)
                    swap = leftChildIndex;
            }

            if (rightChildIndex < length) {
                rightChild = this.heap[rightChildIndex];
                if (
                    (swap === null && rightChild.priority < element.priority) ||
                    (swap !== null && rightChild.priority < leftChild.priority)
                )
                    swap = rightChildIndex;
            }

            if (swap === null)
                break;

            this.heap[index] = this.heap[swap];
            this.heap[swap] = element;
            index = swap;
        }
    }

    isEmpty() {
        return this.heap.length === 0;
    }
}

class Graph {
    constructor(vertices) {
        this.V = vertices;
        this.adjList = new Array(this.V).fill(null).map(() => []);
    }

    addEdge(u, v, w) {
        this.adjList[u].push({ v, w });
        this.adjList[v].push({ v: u, w });
    }

    dijkstra(src) {
        const dist = Array(this.V).fill(Number.MAX_VALUE);
        dist[src] = 0;
        const pq = new PriorityQueue();
        pq.enqueue(src, 0);

        while (!pq.isEmpty()) {
            const { value: u } = pq.dequeue();
            for (const { v, w } of this.adjList[u]) {
                const newDist = dist[u] + w;
                if (newDist < dist[v]) {
                    dist[v] = newDist;
                    pq.enqueue(v, newDist);
                }
            }
        }

        console.log(`Shortest distances from node ${src}:`);
        for (let i = 0; i < this.V; i++) {
            console.log(`Node ${i}: ${dist[i] === Number.MAX_VALUE ? 'INF' : dist[i]}`);
        }
    }
}

// Example usage
const V = 9;
const g = new Graph(V);

g.addEdge(0, 1, 4);
g.addEdge(0, 7, 8);
g.addEdge(1, 2, 8);
g.addEdge(1, 7, 11);
g.addEdge(2, 3, 7);
g.addEdge(2, 8, 2);
g.addEdge(2, 5, 4);
g.addEdge(3, 4, 9);
g.addEdge(3, 5, 14);
g.addEdge(4, 5, 10);
g.addEdge(5, 6, 2);
g.addEdge(6, 7, 1);
g.addEdge(6, 8, 6);
g.addEdge(7, 8, 7);

const startNode = 0;
g.dijkstra(startNode);
```

## C#

```cs
using System;
using System.Collections.Generic;

class Graph
{
    private int V;
    private List<KeyValuePair<int, int>>[] adjList;

    public Graph(int V)
    {
        this.V = V;
        adjList = new List<KeyValuePair<int, int>>[V];
        for (int i = 0; i < V; i++)
        {
            adjList[i] = new List<KeyValuePair<int, int>>();
        }
    }

    public void AddEdge(int u, int v, int weight)
    {
        adjList[u].Add(new KeyValuePair<int, int>(v, weight));
        adjList[v].Add(new KeyValuePair<int, int>(u, weight));
    }

    public void Dijkstra(int src)
    {
        int[] dist = new int[V];
        for (int i = 0; i < V; i++)
        {
            dist[i] = int.MaxValue;
        }
        dist[src] = 0;

        SortedSet<KeyValuePair<int, int>> pq = new SortedSet<KeyValuePair<int, int>>(
            Comparer<KeyValuePair<int, int>>.Create((a, b) => a.Value.CompareTo(b.Value))
        );
        pq.Add(new KeyValuePair<int, int>(src, 0));

        while (pq.Count > 0)
        {
            KeyValuePair<int, int> node = pq.First;
            pq.Remove(node);
            int u = node.Key;

            if (node.Value > dist[u])
                continue;

            foreach (var neighbor in adjList[u])
            {
                int v = neighbor.Key;
                int weight = neighbor.Value;

                if (dist[u] + weight < dist[v])
                {
                    pq.Add(new KeyValuePair<int, int>(v, dist[u] + weight));
                    dist[v] = dist[u] + weight;
                }
            }
        }

        Console.WriteLine("Shortest distances from node " + src + ":");
        for (int i = 0; i < V; i++)
        {
            Console.WriteLine("Node " + i + ": " + (dist[i] == int.MaxValue ? "INF" : dist[i].ToString()));
        }
    }
}

class Program
{
    static void Main(string[] args)
    {
        int V = 9;
        Graph g = new Graph(V);

        g.AddEdge(0, 1, 4);
        g.AddEdge(0, 7, 8);
        g.AddEdge(1, 2, 8);
        g.AddEdge(1, 7, 11);
        g.AddEdge(2, 3, 7);
        g.AddEdge(2, 8, 2);
        g.AddEdge(2, 5, 4);
        g.AddEdge(3, 4, 9);
        g.AddEdge(3, 5, 14);
        g.AddEdge(4, 5, 10);
        g.AddEdge(5, 6, 2);
        g.AddEdge(6, 7, 1);
        g.AddEdge(6, 8, 6);
        g.AddEdge(7, 8, 7);

        int startNode = 0;
        g.Dijkstra(startNode);
    }
}
```

## Ruby

```rb
class Graph
  attr_reader :adj_list

  def initialize(vertices)
    @vertices = vertices
    @adj_list = Hash.new { |hash, key| hash[key] = [] }
  end

  def add_edge(u, v, w)
    @adj_list[u] << { vertex: v, weight: w }
    @adj_list[v] << { vertex: u, weight: w }
  end

  def dijkstra(src)
    dist = Array.new(@vertices, Float::INFINITY)
    dist[src] = 0

    pq = PriorityQueue.new { |a, b| dist[a] < dist[b] }
    pq.push(src)

    while !pq.empty?
      u = pq.pop

      @adj_list[u].each do |neighbor|
        v = neighbor[:vertex]
        weight = neighbor[:weight]

        if dist[u] + weight < dist[v]
          dist[v] = dist[u] + weight
          pq.push(v)
        end
      end
    end

    puts "Shortest distances from node #{src}:"
    (0...@vertices).each do |i|
      puts "Node #{i}: #{dist[i] == Float::INFINITY ? 'INF' : dist[i]}"
    end
  end
end

class PriorityQueue
  def initialize(&comparator)
    @elements = []
    @comparator = comparator || proc { |a, b| a <=> b }
  end

  def push(element)
    @elements << element
    heapify_up
  end

  def pop
    return nil if @elements.empty?

    if @elements.size == 1
      return @elements.pop
    end

    top = @elements.first
    @elements[0] = @elements.pop
    heapify_down
    top
  end

  def empty?
    @elements.empty?
  end

  private

  def heapify_up
    index = @elements.size - 1

    while parent_index(index) && @comparator.call(@elements[parent_index(index)], @elements[index]) > 0
      swap(index, parent_index(index))
      index = parent_index(index)
    end
  end

  def heapify_down
    index = 0

    while left_child_index(index)
      smaller_child_index = left_child_index(index)
      if right_child_index(index) && @comparator.call(@elements[right_child_index(index)], @elements[left_child_index(index)]) < 0
        smaller_child_index = right_child_index(index)
      end

      if @comparator.call(@elements[index], @elements[smaller_child_index]) < 0
        break
      else
        swap(index, smaller_child_index)
      end

      index = smaller_child_index
    end
  end

  def left_child_index(parent_index)
    2 * parent_index + 1 if 2 * parent_index + 1 < @elements.size
  end

  def right_child_index(parent_index)
    2 * parent_index + 2 if 2 * parent_index + 2 < @elements.size
  end

  def parent_index(child_index)
    (child_index - 1) / 2 if child_index > 0
  end

  def swap(a, b)
    @elements[a], @elements[b] = @elements[b], @elements[a]
  end
end

# Example usage
V = 9
g = Graph.new(V)

g.add_edge(0, 1, 4)
g.add_edge(0, 7, 8)
g.add_edge(1, 2, 8)
g.add_edge(1, 7, 11)
g.add_edge(2, 3, 7)
g.add_edge(2, 8, 2)
g.add_edge(2, 5, 4)
g.add_edge(3, 4, 9)
g.add_edge(3, 5, 14)
g.add_edge(4, 5, 10)
g.add_edge(5, 6, 2)
g.add_edge(6, 7, 1)
g.add_edge(6, 8, 6)
g.add_edge(7, 8, 7)

start_node = 0
g.dijkstra(start_node)
```

