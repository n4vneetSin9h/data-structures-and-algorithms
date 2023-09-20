package main

import (
	"fmt"
)

type DisjointSets struct {
	parent map[int]int
	rank   map[int]int
}

func NewDisjointSets() *DisjointSets {
	return &DisjointSets{
		parent: make(map[int]int),
		rank:   make(map[int]int),
	}
}

func (ds *DisjointSets) makeSet(element int) {
	if _, ok := ds.parent[element]; !ok {
		ds.parent[element] = element
		ds.rank[element] = 0
	}
}

func (ds *DisjointSets) find(element int) int {
	parentElement, ok := ds.parent[element]
	if !ok {
		return element
	}

	if parentElement == element {
		return parentElement
	}

	ds.parent[element] = ds.find(parentElement) // Path compression
	return ds.parent[element]
}

func (ds *DisjointSets) union(element1, element2 int) {
	parent1 := ds.find(element1)
	parent2 := ds.find(element2)

	if parent1 != parent2 {
		rank1 := ds.rank[parent1]
		rank2 := ds.rank[parent2]

		if rank1 > rank2 {
			ds.parent[parent2] = parent1
		} else if rank1 < rank2 {
			ds.parent[parent1] = parent2
		} else {
			ds.parent[parent2] = parent1
			ds.rank[parent1]++
		}
	}
}

func (ds *DisjointSets) contains(element int) bool {
	_, ok := ds.parent[element]
	return ok
}

func (ds *DisjointSets) setSize(element int) int {
	representative := ds.find(element)
	count := 0

	for _, val := range ds.parent {
		if ds.find(val) == representative {
			count++
		}
	}

	return count
}

func (ds *DisjointSets) elementsInSameSet(element int) []int {
	representative := ds.find(element)
	var elements []int

	for key, val := range ds.parent {
		if ds.find(val) == representative {
			elements = append(elements, key)
		}
	}

	return elements
}

func (ds *DisjointSets) allSets() [][]int {
	sets := make(map[int][]int)

	for element := range ds.parent {
		representative := ds.find(element)
		if _, ok := sets[representative]; !ok {
			sets[representative] = []int{}
		}
		sets[representative] = append(sets[representative], element)
	}

	var result [][]int
	for _, elements := range sets {
		result = append(result, elements)
	}

	return result
}

func (ds *DisjointSets) reset() {
	ds.parent = make(map[int]int)
	ds.rank = make(map[int]int)
}

func (ds *DisjointSets) isSameSet(element1, element2 int) bool {
	parent1 := ds.find(element1)
	parent2 := ds.find(element2)
	return parent1 == parent2
}

func (ds *DisjointSets) removeSet(element int) {
	representative := ds.find(element)
	for key := range ds.parent {
		if ds.find(key) == representative {
			delete(ds.parent, key)
			delete(ds.rank, key)
		}
	}
}

func (ds *DisjointSets) pathToRoot(element int) []int {
	representative := ds.find(element)
	var path []int
	currentNode := element

	for currentNode != representative {
		path = append(path, currentNode)
		parent, ok := ds.parent[currentNode]
		if !ok {
			return nil
		}
		currentNode = parent
	}

	path = append(path, representative)
	return path
}

func (ds *DisjointSets) getRepresentatives() []int {
	representativesSet := make(map[int]bool)
	var representatives []int

	for _, val := range ds.parent {
		representative := ds.find(val)
		if !representativesSet[representative] {
			representatives = append(representatives, representative)
			representativesSet[representative] = true
		}
	}

	return representatives
}

func (ds *DisjointSets) findAndCompress(element int) int {
	representative := ds.find(element)
	ds.compressPath(element, representative)
	return representative
}

func (ds *DisjointSets) compressPath(element, representative int) {
	currentNode := element

	for currentNode != representative {
		parent, ok := ds.parent[currentNode]
		if !ok {
			return
		}
		ds.parent[currentNode] = representative
		currentNode = parent
	}
}

func (ds *DisjointSets) printSets() {
	sets := make(map[int][]int)

	for element := range ds.parent {
		representative := ds.find(element)
		if _, ok := sets[representative]; !ok {
			sets[representative] = []int{}
		}
		sets[representative] = append(sets[representative], element)
	}

	for representative, elements := range sets {
		fmt.Printf("Set with representative %d: %v\n", representative, elements)
	}
}
