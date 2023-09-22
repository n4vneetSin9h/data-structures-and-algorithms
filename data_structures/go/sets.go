package main

import (
	"fmt"
	"strconv"
	"strings"
)

type CustomSet struct {
	elements map[int]bool // Map to mimic a set
}

// Basic Set Operations

// isEmpty checks if the set is empty.
func (set *CustomSet) isEmpty() bool {
	return len(set.elements) == 0
}

// count returns the number of elements in the set.
func (set *CustomSet) count() int {
	return len(set.elements)
}

// insert inserts an element into the set.
func (set *CustomSet) insert(element int) {
	set.elements[element] = true
}

// remove removes an element from the set.
func (set *CustomSet) remove(element int) {
	delete(set.elements, element)
}

// contains checks if the set contains a specific element.
func (set *CustomSet) contains(element int) bool {
	_, exists := set.elements[element]
	return exists
}

// Set Operations

// union performs the union operation with another set and returns a new set.
func (set *CustomSet) union(otherSet *CustomSet) *CustomSet {
	newSet := &CustomSet{elements: make(map[int]bool)}
	for element := range set.elements {
		newSet.insert(element)
	}
	for element := range otherSet.elements {
		newSet.insert(element)
	}
	return newSet
}

// intersection performs the intersection operation with another set and returns a new set.
func (set *CustomSet) intersection(otherSet *CustomSet) *CustomSet {
	newSet := &CustomSet{elements: make(map[int]bool)}
	for element := range set.elements {
		if otherSet.contains(element) {
			newSet.insert(element)
		}
	}
	return newSet
}

// difference performs the difference operation with another set and returns a new set.
func (set *CustomSet) difference(otherSet *CustomSet) *CustomSet {
	newSet := &CustomSet{elements: make(map[int]bool)}
	for element := range set.elements {
		if !otherSet.contains(element) {
			newSet.insert(element)
		}
	}
	return newSet
}

// isSubsetOf checks if this set is a subset of another set.
func (set *CustomSet) isSubsetOf(otherSet *CustomSet) bool {
	for element := range set.elements {
		if !otherSet.contains(element) {
			return false
		}
	}
	return true
}

// isSupersetOf checks if this set is a superset of another set.
func (set *CustomSet) isSupersetOf(otherSet *CustomSet) bool {
	return otherSet.isSubsetOf(set)
}

// isDisjointWith checks if this set is disjoint with another set.
func (set *CustomSet) isDisjointWith(otherSet *CustomSet) bool {
	for element := range set.elements {
		if otherSet.contains(element) {
			return false
		}
	}
	return true
}

// forEach applies a closure to each element in the set.
func (set *CustomSet) forEach(closure func(int)) {
	for element := range set.elements {
		closure(element)
	}
}

// removeAll removes all elements from the set.
func (set *CustomSet) removeAll() {
	set.elements = make(map[int]bool)
}

// removeAllOccurrences removes all occurrences of an element from the set.
func (set *CustomSet) removeAllOccurrences(element int) {
	delete(set.elements, element)
}

// symmetricDifference returns the set with common elements between this set and another set.
func (set *CustomSet) symmetricDifference(otherSet *CustomSet) *CustomSet {
	diff1 := set.difference(otherSet)
	diff2 := otherSet.difference(set)
	return diff1.union(diff2)
}

// formUnion inserts elements from another set into this set.
func (set *CustomSet) formUnion(otherSet *CustomSet) {
	for element := range otherSet.elements {
		set.insert(element)
	}
}

// formIntersection removes elements of this set that are not in another set.
func (set *CustomSet) formIntersection(otherSet *CustomSet) {
	for element := range set.elements {
		if !otherSet.contains(element) {
			delete(set.elements, element)
		}
	}
}

// subtract removes elements of this set that are in another set.
func (set *CustomSet) subtract(otherSet *CustomSet) {
	for element := range otherSet.elements {
		delete(set.elements, element)
	}
}

// ... Add more set operations as needed ...

// PrintSet prints the elements of the set.
func (set *CustomSet) printSet() {
	var elements []string
	for element := range set.elements {
		elements = append(elements, strconv.Itoa(element))
	}
	fmt.Printf("{%s}\n", strings.Join(elements, ", "))
}
