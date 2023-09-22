package main

import "fmt"

type CustomQueue struct {
	elements []interface{}
}

// MARK: - Queue Operations

// Enqueue an element to the back of the queue.
func (q *CustomQueue) enqueue(element interface{}) {
	q.elements = append(q.elements, element)
}

// Dequeue an element from the front of the queue.
func (q *CustomQueue) dequeue() interface{} {
	if len(q.elements) == 0 {
		return nil
	}
	element := q.elements[0]
	q.elements = q.elements[1:]
	return element
}

// Peek at the front element in the queue without dequeuing.
func (q *CustomQueue) peek() interface{} {
	if len(q.elements) == 0 {
		return nil
	}
	return q.elements[0]
}

// Check if the queue is empty.
func (q *CustomQueue) isEmpty() bool {
	return len(q.elements) == 0
}

// Get the number of elements in the queue.
func (q *CustomQueue) count() int {
	return len(q.elements)
}

// MARK: - Queue Operations with Slice

// Initialize the queue with a slice of elements.
func (q *CustomQueue) initializeWithSlice(slice []interface{}) {
	q.elements = slice
}

// Enqueue a slice of elements to the back of the queue.
func (q *CustomQueue) enqueueSlice(slice []interface{}) {
	q.elements = append(q.elements, slice...)
}

// Dequeue a specified number of elements from the front of the queue.
func (q *CustomQueue) dequeueSlice(count int) []interface{} {
	if count <= 0 || count > len(q.elements) {
		return nil
	}
	dequeuedElements := q.elements[:count]
	q.elements = q.elements[count:]
	return dequeuedElements
}

// MARK: - Clear and Reset

// Clear the queue by removing all elements.
func (q *CustomQueue) clear() {
	q.elements = nil
}

// MARK: - Checking for Element

// Check if the queue contains a specific element.
func (q *CustomQueue) contains(element interface{}) bool {
	for _, e := range q.elements {
		if e == element {
			return true
		}
	}
	return false
}

// MARK: - Filtering

// Filter the queue using a given predicate.
func (q *CustomQueue) filter(predicate func(interface{}) bool) []interface{} {
	var filtered []interface{}
	for _, e := range q.elements {
		if predicate(e) {
			filtered = append(filtered, e)
		}
	}
	return filtered
}

// MARK: - Conversion to Slice

// Convert the queue to a slice.
func (q *CustomQueue) toSlice() []interface{} {
	return q.elements
}

// MARK: - Map

// Map a transformation function over all elements in the queue.
func (q *CustomQueue) mapElements(transform func(interface{}) interface{}) []interface{} {
	mapped := make([]interface{}, len(q.elements))
	for i, e := range q.elements {
		mapped[i] = transform(e)
	}
	return mapped
}

// MARK: - Reduce

// Reduce the queue to a single value using a reducer function and an initial value.
func (q *CustomQueue) reduce(initialValue interface{}, reducer func(interface{}, interface{}) interface{}) interface{} {
	accumulator := initialValue
	for _, e := range q.elements {
		accumulator = reducer(accumulator, e)
	}
	return accumulator
}

// MARK: - Concatenate

// Concatenate another queue to this queue.
func (q *CustomQueue) concatenate(otherQueue CustomQueue) {
	q.elements = append(q.elements, otherQueue.elements...)
}

// MARK: - Subscript

// Access the element at a specific index in the queue.
func (q *CustomQueue) get(index int) interface{} {
	if index < 0 || index >= len(q.elements) {
		return nil
	}
	return q.elements[index]
}

// MARK: - Print

// Print the queue.
func (q *CustomQueue) printQueue() {
	fmt.Println("Queue:", q.elements)
}

// MARK: - Other Methods

// TODO: Add other methods as needed.
