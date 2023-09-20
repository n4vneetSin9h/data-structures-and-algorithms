package main

import (
	"fmt"
	"sort"
)

type comparator func(int, int) bool

type Heap struct {
	elements   []int
	isMinHeap  bool
	comparator comparator
}

// NewHeap initializes a new heap with the specified configuration.
func NewHeap(isMinHeap bool, comparator comparator) *Heap {
	return &Heap{
		elements:   []int{},
		isMinHeap:  isMinHeap,
		comparator: comparator,
	}
}

// heapifyUp maintains the heap property while inserting an element.
func (h *Heap) heapifyUp() {
	currentIndex := len(h.elements) - 1
	for currentIndex > 0 {
		parentIndex := (currentIndex - 1) / 2

		if h.shouldSwap(parentIndex, currentIndex) {
			h.elements[parentIndex], h.elements[currentIndex] = h.elements[currentIndex], h.elements[parentIndex]
			currentIndex = parentIndex
		} else {
			break
		}
	}
}

// Insert inserts an element into the heap.
func (h *Heap) Insert(element int) {
	h.elements = append(h.elements, element)
	h.heapifyUp()
}

// heapifyDown maintains the heap property while removing an element.
func (h *Heap) heapifyDown() {
	currentIndex := 0

	for h.hasLeftChild(currentIndex) {
		smallestChildIndex := h.leftChildIndex(currentIndex)

		if h.hasRightChild(currentIndex) && h.shouldSwap(h.rightChildIndex(currentIndex), smallestChildIndex) {
			smallestChildIndex = h.rightChildIndex(currentIndex)
		}

		if h.shouldSwap(smallestChildIndex, currentIndex) {
			h.elements[smallestChildIndex], h.elements[currentIndex] = h.elements[currentIndex], h.elements[smallestChildIndex]
			currentIndex = smallestChildIndex
		} else {
			break
		}
	}
}

// Remove removes the root element from the heap.
func (h *Heap) Remove() int {
	if len(h.elements) == 0 {
		return 0
	}

	root := h.elements[0]
	h.elements[0] = h.elements[len(h.elements)-1]
	h.elements = h.elements[:len(h.elements)-1]
	h.heapifyDown()

	return root
}

// Peek returns the root element without removing it.
func (h *Heap) Peek() int {
	if len(h.elements) > 0 {
		return h.elements[0]
	}
	return 0
}

// leftChildIndex calculates the left child index of a parent index.
func (h *Heap) leftChildIndex(parentIndex int) int {
	return 2*parentIndex + 1
}

// rightChildIndex calculates the right child index of a parent index.
func (h *Heap) rightChildIndex(parentIndex int) int {
	return 2*parentIndex + 2
}

// hasLeftChild checks if a node has a left child.
func (h *Heap) hasLeftChild(index int) bool {
	return h.leftChildIndex(index) < len(h.elements)
}

// hasRightChild checks if a node has a right child.
func (h *Heap) hasRightChild(index int) bool {
	return h.rightChildIndex(index) < len(h.elements)
}

// shouldSwap determines if elements at two indices should be swapped based on min or max heap property.
func (h *Heap) shouldSwap(firstIndex, secondIndex int) bool {
	if h.isMinHeap {
		return h.comparator(h.elements[secondIndex], h.elements[firstIndex])
	} else {
		return h.comparator(h.elements[firstIndex], h.elements[secondIndex])
	}
}

// BuildHeap builds a heap from an array of elements.
func (h *Heap) BuildHeap(elements []int) {
	h.elements = elements
	for i := len(elements)/2 - 1; i >= 0; i-- {
		h.heapifyDown()
	}
}

// ReplaceRoot replaces the root element with a new element.
func (h *Heap) ReplaceRoot(element int) {
	if len(h.elements) == 0 {
		h.Insert(element)
		return
	}

	h.elements[0] = element
	h.heapifyDown()
}

// RemoveAtIndex removes an element at a specific index from the heap.
func (h *Heap) RemoveAtIndex(index int) int {
	if index < 0 || index >= len(h.elements) {
		return 0
	}

	if index == len(h.elements)-1 {
		return h.elements[index]
	}

	h.elements[index], h.elements[len(h.elements)-1] = h.elements[len(h.elements)-1], h.elements[index]
	removedElement := h.elements[len(h.elements)-1]
	h.elements = h.elements[:len(h.elements)-1]
	h.heapifyDown()

	return removedElement
}

// Sort sorts the heap in-place (ascending order).
func (h *Heap) Sort() {
	for i := len(h.elements) - 1; i >= 1; i-- {
		h.elements[0], h.elements[i] = h.elements[i], h.elements[0]
		h.heapifyDown()
	}
}

// Index returns the index of a specific element, if it exists in the heap.
func (h *Heap) Index(element int) int {
	return sort.SearchInts(h.elements, element)
}

// PrintHeap prints the elements of the heap.
func (h *Heap) PrintHeap() {
	fmt.Println("Heap elements:", h.elements)
}
