package main

import (
	"fmt"
)

type CustomStack struct {
	elements []interface{}
}

// MARK: - Stack Operations

// isEmpty checks if the stack is empty.
func (s *CustomStack) isEmpty() bool {
	return len(s.elements) == 0
}

// count returns the number of elements in the stack.
func (s *CustomStack) count() int {
	return len(s.elements)
}

// push adds an element onto the stack.
func (s *CustomStack) push(element interface{}) {
	s.elements = append(s.elements, element)
}

// pop removes and returns the top element from the stack.
func (s *CustomStack) pop() interface{} {
	if len(s.elements) == 0 {
		return nil
	}
	index := len(s.elements) - 1
	element := s.elements[index]
	s.elements = s.elements[:index]
	return element
}

// peek returns the top element in the stack without removing it.
func (s *CustomStack) peek() interface{} {
	if len(s.elements) == 0 {
		return nil
	}
	return s.elements[len(s.elements)-1]
}

// MARK: - Stack Operations with Slice

// initializeWithSlice initializes the stack with a slice of elements.
func (s *CustomStack) initializeWithSlice(slice []interface{}) {
	s.elements = slice
}

// pushSlice adds a slice of elements onto the stack.
func (s *CustomStack) pushSlice(slice []interface{}) {
	s.elements = append(s.elements, slice...)
}

// popSlice removes and returns a specified number of elements from the stack.
func (s *CustomStack) popSlice(count int) []interface{} {
	if count <= 0 || count > len(s.elements) {
		return nil
	}

	poppedElements := s.elements[len(s.elements)-count:]
	s.elements = s.elements[:len(s.elements)-count]
	return poppedElements
}

// MARK: - Clear and Reset

// clear removes all elements from the stack.
func (s *CustomStack) clear() {
	s.elements = nil
}

// MARK: - Checking for Element

// contains checks if the stack contains a specific element.
func (s *CustomStack) contains(element interface{}) bool {
	for _, e := range s.elements {
		if e == element {
			return true
		}
	}
	return false
}

// MARK: - Filtering

// filter filters the stack using a given predicate.
func (s *CustomStack) filter(predicate func(interface{}) bool) []interface{} {
	var filtered []interface{}
	for _, element := range s.elements {
		if predicate(element) {
			filtered = append(filtered, element)
		}
	}
	return filtered
}

// MARK: - Conversion to Slice

// toSlice converts the stack to a slice.
func (s *CustomStack) toSlice() []interface{} {
	return s.elements
}

// MARK: - Map

// mapElements transforms each element in the stack using a provided transform function.
func (s *CustomStack) mapElements(transform func(interface{}) interface{}) []interface{} {
	var mapped []interface{}
	for _, element := range s.elements {
		mapped = append(mapped, transform(element))
	}
	return mapped
}

// MARK: - Reduce

// reduce combines all elements in the stack using a reducer function.
func (s *CustomStack) reduce(initialResult interface{}, reducer func(interface{}, interface{}) interface{}) interface{} {
	result := initialResult
	for _, element := range s.elements {
		result = reducer(result, element)
	}
	return result
}

// MARK: - Concatenate

// concatenate concatenates another stack to this stack.
func (s *CustomStack) concatenate(otherStack CustomStack) {
	s.elements = append(s.elements, otherStack.elements...)
}

// MARK: - Subscript

// getAtIndex returns the element at a specific index in the stack.
func (s *CustomStack) getAtIndex(index int) interface{} {
	if index < 0 || index >= len(s.elements) {
		return nil
	}
	return s.elements[index]
}

// ... Add more stack operations as needed ...

// printStack prints the stack elements.
func (s *CustomStack) printStack() {
	if len(s.elements) == 0 {
		fmt.Println("Stack is empty.")
	} else {
		fmt.Println("Stack:")
		for i := len(s.elements) - 1; i >= 0; i-- {
			fmt.Println(s.elements[i])
		}
	}
}
