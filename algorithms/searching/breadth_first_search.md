# Breadth-First Search (BFS) Algorithm

## Description

Breadth-First Search (BFS) is an algorithm used to traverse or search tree or graph data structures. It starts at the tree root or some arbitrary node of a graph, and explores the neighbor nodes at the present depth prior to moving on to nodes at the next depth level. It visits all the vertices at the present depth prior to moving on to vertices at the next depth level.

## Steps

1. Start with selecting a node to begin traversal (usually the root node in a tree or an arbitrary node in a graph).
2. Enqueue the selected node and mark it as visited.
3. Dequeue a node from the queue and visit it.
4. Enqueue all adjacent and unvisited nodes of the visited node.
5. Repeat steps 3 and 4 until the queue is empty.

## Pseudo Code

```plaintext
BFS(graph, startNode):
    initialize an empty queue
    enqueue startNode into the queue
    mark startNode as visited

    while queue is not empty:
        currentNode = dequeue from the queue
        visit currentNode

        for each neighbor of currentNode:
            if neighbor is not visited:
                mark neighbor as visited
                enqueue neighbor into the queue
```

# Breadth-First Search (BFS) Algorithm: Time and Space Complexity Analysis

## Time Complexity Analysis

- **Best Case**: O(|V| + |E|) where V is the number of vertices and E is the number of edges.

- **Worst Case**: O(|V| + |E|)

- **Average Case**: O(|V| + |E|)

## Space Complexity Analysis

The space complexity of the Breadth-First Search (BFS) Algorithm can be O(|V|).

# Code

## C++

```cpp
#include <iostream>
#include <unordered_map>

class HashTable {
private:
    std::unordered_map<int, int> table;

public:
    void insert(int key, int value) {
        table[key] = value;
    }

    bool search(int key, int& value) {
        auto it = table.find(key);
        if (it != table.end()) {
            value = it->second;
            return true;
        }
        return false;
    }

    void remove(int key) {
        table.erase(key);
    }
};

int main() {
    HashTable hashTable;

    // Insert
    hashTable.insert(1, 10);
    hashTable.insert(2, 20);

    // Search
    int value;
    if (hashTable.search(1, value))
        std::cout << "Value for key 1: " << value << std::endl;
    else
        std::cout << "Key not found" << std::endl;

    // Delete
    hashTable.remove(1);

    return 0;
}
```

## Rust

```rs
use std::collections::HashMap;

struct HashTable {
    table: HashMap<i32, i32>,
}

impl HashTable {
    fn new() -> Self {
        HashTable {
            table: HashMap::new(),
        }
    }

    fn insert(&mut self, key: i32, value: i32) {
        self.table.insert(key, value);
    }

    fn search(&self, key: i32) -> Option<&i32> {
        self.table.get(&key)
    }

    fn remove(&mut self, key: i32) {
        self.table.remove(&key);
    }
}

fn main() {
    let mut hash_table = HashTable::new();

    // Insert
    hash_table.insert(1, 10);
    hash_table.insert(2, 20);

    // Search
    if let Some(value) = hash_table.search(1) {
        println!("Value for key 1: {}", value);
    } else {
        println!("Key not found");
    }

    // Delete
    hash_table.remove(1);
}
```

## Python

```py
class HashTable:
    def __init__(self):
        self.table = {}

    def insert(self, key, value):
        self.table[key] = value

    def search(self, key):
        return self.table.get(key, None)

    def remove(self, key):
        self.table.pop(key, None)

# Example usage
hash_table = HashTable()

# Insert
hash_table.insert(1, 10)
hash_table.insert(2, 20)

# Search
value = hash_table.search(1)
if value is not None:
    print("Value for key 1:", value)
else:
    print("Key not found")

# Delete
hash_table.remove(1)
```

## Swift

```swift
class HashTable {
    var table: [Int: Int] = [:]

    func insert(_ key: Int, _ value: Int) {
        table[key] = value
    }

    func search(_ key: Int) -> Int? {
        return table[key]
    }

    func remove(_ key: Int) {
        table[key] = nil
    }
}

let hashTable = HashTable()

// Insert
hashTable.insert(1, 10)
hashTable.insert(2, 20)

// Search
if let value = hashTable.search(1) {
    print("Value for key 1: \(value)")
} else {
    print("Key not found")
}

// Delete
hashTable.remove(1)
```

## Kotlin

```kotlin
class HashTable {
    private val table: MutableMap<Int, Int> = mutableMapOf()

    fun insert(key: Int, value: Int) {
        table[key] = value
    }

    fun search(key: Int): Int? {
        return table[key]
    }

    fun remove(key: Int) {
        table.remove(key)
    }
}

fun main() {
    val hashTable = HashTable()

    // Insert
    hashTable.insert(1, 10)
    hashTable.insert(2, 20)

    // Search
    val value = hashTable.search(1)
    if (value != null) {
        println("Value for key 1: $value")
    } else {
        println("Key not found")
    }

    // Delete
    hashTable.remove(1)
}
```

## Go

```go
package main

import "fmt"

type HashTable struct {
    table map[int]int
}

func (h *HashTable) insert(key, value int) {
    h.table[key] = value
}

func (h *HashTable) search(key int) (int, bool) {
    value, found := h.table[key]
    return value, found
}

func (h *HashTable) remove(key int) {
    delete(h.table, key)
}

func main() {
    hashTable := HashTable{
        table: make(map[int]int),
    }

    // Insert
    hashTable.insert(1, 10)
    hashTable.insert(2, 20)

    // Search
    value, found := hashTable.search(1)
    if found {
        fmt.Printf("Value for key 1: %d\n", value)
    } else {
        fmt.Println("Key not found")
    }

    // Delete
    hashTable.remove(1)
}
```

## Java

```java
import java.util.HashMap;

class HashTable {
    private HashMap<Integer, Integer> table = new HashMap<>();

    public void insert(int key, int value) {
        table.put(key, value);
    }

    public Integer search(int key) {
        return table.get(key);
    }

    public void remove(int key) {
        table.remove(key);
    }
}

public class HashTableExample {
    public static void main(String[] args) {
        HashTable hashTable = new HashTable();

        // Insert
        hashTable.insert(1, 10);
        hashTable.insert(2, 20);

        // Search
        Integer value = hashTable.search(1);
        if (value != null) {
            System.out.println("Value for key 1: " + value);
        } else {
            System.out.println("Key not found");
        }

        // Delete
        hashTable.remove(1);
    }
}
```

## JavaScript

```js
class HashTable {
    constructor() {
        this.table = {};
    }

    insert(key, value) {
        this.table[key] = value;
    }

    search(key) {
        return this.table[key];
    }

    remove(key) {
        delete this.table[key];
    }
}

const hashTable = new HashTable();

// Insert
hashTable.insert(1, 10);
hashTable.insert(2, 20);

// Search
const value = hashTable.search(1);
if (value !== undefined) {
    console.log("Value for key 1:", value);
} else {
    console.log("Key not found");
}

// Delete
hashTable.remove(1);
```

## C#

```cs
using System;

class HashTable {
    private System.Collections.Generic.Dictionary<int, int> table = new System.Collections.Generic.Dictionary<int, int>();

    public void Insert(int key, int value) {
        table[key] = value;
    }

    public bool Search(int key, out int value) {
        return table.TryGetValue(key, out value);
    }

    public void Remove(int key) {
        table.Remove(key);
    }
}

class Program {
    static void Main(string[] args) {
        HashTable hashTable = new HashTable();

        // Insert
        hashTable.Insert(1, 10);
        hashTable.Insert(2, 20);

        // Search
        if (hashTable.Search(1, out int value)) {
            Console.WriteLine("Value for key 1: " + value);
        } else {
            Console.WriteLine("Key not found");
        }

        // Delete
        hashTable.Remove(1);
    }
}
```

## Ruby

```rb
class HashTable
  def initialize
    @table = {}
  end

  def insert(key, value)
    @table[key] = value
  end

  def search(key)
    @table[key]
  end

  def remove(key)
    @table.delete(key)
  end
end

hash_table = HashTable.new

# Insert
hash_table.insert(1, 10)
hash_table.insert(2, 20)

# Search
value = hash_table.search(1)
if value
  puts "Value for key 1: #{value}"
else
  puts "Key not found"
end

# Delete
hash_table.remove(1)
```

