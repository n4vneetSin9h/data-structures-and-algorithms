# Depth-First Search (DFS) Algorithm

## Description

Depth-First Search (DFS) is an algorithm used to traverse or search tree or graph data structures. It starts at the tree root or some arbitrary node of a graph, and explores as far as possible along each branch before backtracking.

## Steps

1. Start with selecting a node to begin traversal (usually the root node in a tree or an arbitrary node in a graph).
2. Mark the selected node as visited and visit it.
3. Recur for all the adjacent and unvisited nodes of the visited node.
4. Repeat step 3 until all nodes are visited.

## Pseudo Code

```plaintext
DFS(graph, startNode):
    mark startNode as visited
    visit startNode

    for each neighbor of startNode:
        if neighbor is not visited:
            DFS(graph, neighbor)
```

# Depth-First Search (DFS) Algorithm: Time and Space Complexity Analysis

## Time Complexity Analysis

- **Best Case**: O(|V| + |E|) where V is the number of vertices and E is the number of edges.

- **Worst Case**: O(|V| + |E|)

- **Average Case**: O(|V| + |E|)

## Space Complexity Analysis

The space complexity of the Depth-First Search (DFS) Algorithm can be O(|V|) where V is the number of vertices in the graph, considering the space used for recursion and the visited set.

# Code

## C++

```cpp
#include <iostream>
#include <vector>
#include <stack>

using namespace std;

class Graph {
private:
    int V;  // Number of vertices
    vector<vector<int>> adj;  // Adjacency list

public:
    Graph(int vertices) : V(vertices) {
        adj.resize(V);
    }

    void addEdge(int u, int v) {
        adj[u].push_back(v);
    }

    void DFS(int startVertex) {
        vector<bool> visited(V, false);
        stack<int> s;
        s.push(startVertex);

        while (!s.empty()) {
            int vertex = s.top();
            s.pop();

            if (!visited[vertex]) {
                cout << vertex << " ";
                visited[vertex] = true;
            }

            for (const auto &neighbor : adj[vertex]) {
                if (!visited[neighbor]) {
                    s.push(neighbor);
                }
            }
        }
    }
};

int main() {
    Graph g(7);  // Create a graph with 7 vertices

    // Add edges
    g.addEdge(0, 1);
    g.addEdge(0, 2);
    g.addEdge(1, 3);
    g.addEdge(1, 4);
    g.addEdge(2, 5);
    g.addEdge(2, 6);

    cout << "DFS starting from vertex 0: ";
    g.DFS(0);

    return 0;
}
```

## Rust

```rs
use std::collections::HashSet;

struct Graph {
    vertices: usize,
    adjacency_list: Vec<Vec<usize>>,
}

impl Graph {
    fn new(vertices: usize) -> Self {
        Graph {
            vertices,
            adjacency_list: vec![vec![]; vertices],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.adjacency_list[u].push(v);
    }

    fn dfs(&self, start_vertex: usize) {
        let mut visited = vec![false; self.vertices];
        let mut stack = vec![start_vertex];

        while let Some(vertex) = stack.pop() {
            if !visited[vertex] {
                print!("{} ", vertex);
                visited[vertex] = true;
            }

            for &neighbor in &self.adjacency_list[vertex] {
                if !visited[neighbor] {
                    stack.push(neighbor);
                }
            }
        }
    }
}

fn main() {
    let mut g = Graph::new(7);

    // Add edges
    g.add_edge(0, 1);
    g.add_edge(0, 2);
    g.add_edge(1, 3);
    g.add_edge(1, 4);
    g.add_edge(2, 5);
    g.add_edge(2, 6);

    print!("DFS starting from vertex 0: ");
    g.dfs(0);
}
```

## Python

```py
from collections import defaultdict

class Graph:
    def __init__(self, vertices):
        self.vertices = vertices
        self.adjacency_list = defaultdict(list)

    def add_edge(self, u, v):
        self.adjacency_list[u].append(v)

    def dfs(self, start_vertex):
        visited = [False] * self.vertices
        stack = [start_vertex]

        while stack:
            vertex = stack.pop()

            if not visited[vertex]:
                print(vertex, end=" ")
                visited[vertex] = True

            stack.extend(neighbor for neighbor in self.adjacency_list[vertex] if not visited[neighbor])

# Create a graph with 7 vertices
g = Graph(7)

# Add edges
g.add_edge(0, 1)
g.add_edge(0, 2)
g.add_edge(1, 3)
g.add_edge(1, 4)
g.add_edge(2, 5)
g.add_edge(2, 6)

print("DFS starting from vertex 0:", end=" ")
g.dfs(0)
```

## Swift

```swift
class Graph {
    var vertices: Int
    var adjacencyList: [[Int]]
    
    init(vertices: Int) {
        self.vertices = vertices
        self.adjacencyList = Array(repeating: [Int](), count: vertices)
    }
    
    func addEdge(_ u: Int, _ v: Int) {
        adjacencyList[u].append(v)
    }
    
    func dfs(_ startVertex: Int) {
        var visited = Array(repeating: false, count: vertices)
        var stack = [startVertex]
        
        while !stack.isEmpty {
            let vertex = stack.removeLast()
            
            if !visited[vertex] {
                print("\(vertex)", terminator: " ")
                visited[vertex] = true
            }
            
            stack += adjacencyList[vertex].reversed().filter { !visited[$0] }
        }
    }
}

let g = Graph(vertices: 7)

// Add edges
g.addEdge(0, 1)
g.addEdge(0, 2)
g.addEdge(1, 3)
g.addEdge(1, 4)
g.addEdge(2, 5)
g.addEdge(2, 6)

print("DFS starting from vertex 0:", terminator: " ")
g.dfs(0)
```

## Kotlin

```kotlin
import java.util.*

class Graph(private val vertices: Int) {
    private val adjacencyList: Array<MutableList<Int>> = Array(vertices) { mutableListOf() }

    fun addEdge(u: Int, v: Int) {
        adjacencyList[u].add(v)
    }

    fun dfs(startVertex: Int) {
        val visited = BooleanArray(vertices)
        val stack = Stack<Int>()
        stack.push(startVertex)

        while (stack.isNotEmpty()) {
            val vertex = stack.pop()

            if (!visited[vertex]) {
                print("$vertex ")
                visited[vertex] = true
            }

            for (neighbor in adjacencyList[vertex]) {
                if (!visited[neighbor]) {
                    stack.push(neighbor)
                }
            }
        }
    }
}

fun main() {
    val g = Graph(7)

    // Add edges
    g.addEdge(0, 1)
    g.addEdge(0, 2)
    g.addEdge(1, 3)
    g.addEdge(1, 4)
    g.addEdge(2, 5)
    g.addEdge(2, 6)

    print("DFS starting from vertex 0: ")
    g.dfs(0)
}
```

## Go

```go
package main

import (
	"fmt"
)

type Graph struct {
	vertices     int
	adjacencyMap map[int][]int
}

func NewGraph(vertices int) *Graph {
	return &Graph{
		vertices:     vertices,
		adjacencyMap: make(map[int][]int),
	}
}

func (g *Graph) AddEdge(u, v int) {
	g.adjacencyMap[u] = append(g.adjacencyMap[u], v)
}

func (g *Graph) DFS(startVertex int) {
	visited := make(map[int]bool)
	stack := []int{startVertex}

	for len(stack) > 0 {
		vertex := stack[len(stack)-1]
		stack = stack[:len(stack)-1]

		if !visited[vertex] {
			fmt.Printf("%d ", vertex)
			visited[vertex] = true
		}

		for _, neighbor := range g.adjacencyMap[vertex] {
			if !visited[neighbor] {
				stack = append(stack, neighbor)
			}
		}
	}
}

func main() {
	g := NewGraph(7)

	// Add edges
	g.AddEdge(0, 1)
	g.AddEdge(0, 2)
	g.AddEdge(1, 3)
	g.AddEdge(1, 4)
	g.AddEdge(2, 5)
	g.AddEdge(2, 6)

	fmt.Print("DFS starting from vertex 0: ")
	g.DFS(0)
	fmt.Println()
}
```

## Java

```java
import java.util.ArrayList;
import java.util.Stack;

class Graph {
    private int vertices;
    private ArrayList<ArrayList<Integer>> adjacencyList;

    public Graph(int vertices) {
        this.vertices = vertices;
        this.adjacencyList = new ArrayList<>(vertices);
        for (int i = 0; i < vertices; i++) {
            this.adjacencyList.add(new ArrayList<>());
        }
    }

    public void addEdge(int u, int v) {
        this.adjacencyList.get(u).add(v);
    }

    public void dfs(int startVertex) {
        boolean[] visited = new boolean[vertices];
        Stack<Integer> stack = new Stack<>();
        stack.push(startVertex);

        while (!stack.isEmpty()) {
            int vertex = stack.pop();

            if (!visited[vertex]) {
                System.out.print(vertex + " ");
                visited[vertex] = true;
            }

            for (int neighbor : this.adjacencyList.get(vertex)) {
                if (!visited[neighbor]) {
                    stack.push(neighbor);
                }
            }
        }
    }
}

public class DepthFirstSearch {
    public static void main(String[] args) {
        Graph g = new Graph(7);

        // Add edges
        g.addEdge(0, 1);
        g.addEdge(0, 2);
        g.addEdge(1, 3);
        g.addEdge(1, 4);
        g.addEdge(2, 5);
        g.addEdge(2, 6);

        System.out.print("DFS starting from vertex 0: ");
        g.dfs(0);
    }
}
```

## JavaScript

```js
class Graph {
    constructor(vertices) {
        this.vertices = vertices;
        this.adjacencyList = new Array(vertices).fill(null).map(() => []);
    }

    addEdge(u, v) {
        this.adjacencyList[u].push(v);
    }

    dfs(startVertex) {
        const visited = new Array(this.vertices).fill(false);
        const stack = [startVertex];

        while (stack.length > 0) {
            const vertex = stack.pop();

            if (!visited[vertex]) {
                process.stdout.write(`${vertex} `);
                visited[vertex] = true;
            }

            for (const neighbor of this.adjacencyList[vertex]) {
                if (!visited[neighbor]) {
                    stack.push(neighbor);
                }
            }
        }
    }
}

const g = new Graph(7);

// Add edges
g.addEdge(0, 1);
g.addEdge(0, 2);
g.addEdge(1, 3);
g.addEdge(1, 4);
g.addEdge(2, 5);
g.addEdge(2, 6);

process.stdout.write("DFS starting from vertex 0: ");
g.dfs(0);
console.log();
```

## C#

```cs
using System;
using System.Collections.Generic;

class Graph
{
    private int vertices;
    private List<List<int>> adjacencyList;

    public Graph(int vertices)
    {
        this.vertices = vertices;
        adjacencyList = new List<List<int>>();
        for (int i = 0; i < vertices; i++)
        {
            adjacencyList.Add(new List<int>());
        }
    }

    public void AddEdge(int u, int v)
    {
        adjacencyList[u].Add(v);
    }

    public void DFS(int startVertex)
    {
        bool[] visited = new bool[vertices];
        Stack<int> stack = new Stack<int>();
        stack.Push(startVertex);

        while (stack.Count > 0)
        {
            int vertex = stack.Pop();

            if (!visited[vertex])
            {
                Console.Write(vertex + " ");
                visited[vertex] = true;
            }

            foreach (int neighbor in adjacencyList[vertex])
            {
                if (!visited[neighbor])
                {
                    stack.Push(neighbor);
                }
            }
        }
    }
}

public class DepthFirstSearch
{
    public static void Main(string[] args)
    {
        Graph g = new Graph(7);

        // Add edges
        g.AddEdge(0, 1);
        g.AddEdge(0, 2);
        g.AddEdge(1, 3);
        g.AddEdge(1, 4);
        g.AddEdge(2, 5);
        g.AddEdge(2, 6);

        Console.Write("DFS starting from vertex 0: ");
        g.DFS(0);
    }
}
```

## Ruby

```rb
class Graph
  def initialize(vertices)
    @vertices = vertices
    @adjacency_list = Array.new(vertices) { [] }
  end

  def add_edge(u, v)
    @adjacency_list[u] << v
  end

  def dfs(start_vertex)
    visited = Array.new(@vertices, false)
    stack = [start_vertex]

    until stack.empty?
      vertex = stack.pop

      unless visited[vertex]
        print "#{vertex} "
        visited[vertex] = true
      end

      @adjacency_list[vertex].reverse_each do |neighbor|
        stack.push(neighbor) unless visited[neighbor]
      end
    end
  end
end

# Create a graph with 7 vertices
g = Graph.new(7)

# Add edges
g.add_edge(0, 1)
g.add_edge(0, 2)
g.add_edge(1, 3)
g.add_edge(1, 4)
g.add_edge(2, 5)
g.add_edge(2, 6)

print "DFS starting from vertex 0: "
g.dfs(0)
puts
```

