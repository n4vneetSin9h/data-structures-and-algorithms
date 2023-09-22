package main

import (
	"fmt"
)

type CustomArray struct {
	elements []interface{} // Internal storage for elements
}

func NewCustomArray(initialSize int) *CustomArray {
	return &CustomArray{
		elements: make([]interface{}, initialSize),
	}
}

// Append an element to the end of the custom array
func (ca *CustomArray) append(element interface{}) {
	ca.elements = append(ca.elements, element)
}

// Insert an element at the specified index
func (ca *CustomArray) insert(element interface{}, index int) {
	ca.elements = append(ca.elements[:index], append([]interface{}{element}, ca.elements[index:]...)...)
}

// Remove the first occurrence of the specified element
func (ca *CustomArray) remove(element interface{}) {
	for i, e := range ca.elements {
		if e == element {
			ca.elements = append(ca.elements[:i], ca.elements[i+1:]...)
			break
		}
	}
}

// Remove and return the element at the specified index
func (ca *CustomArray) pop(index int) interface{} {
	if index >= 0 && index < len(ca.elements) {
		element := ca.elements[index]
		ca.elements = append(ca.elements[:index], ca.elements[index+1:]...)
		return element
	}
	return nil
}

// Get the element at the specified index
func (ca *CustomArray) get(index int) interface{} {
	if index >= 0 && index < len(ca.elements) {
		return ca.elements[index]
	}
	return nil
}

// Set the element at the specified index to the given element
func (ca *CustomArray) set(element interface{}, index int) {
	if index >= 0 && index < len(ca.elements) {
		ca.elements[index] = element
	}
}

// Return the current size of the custom array
func (ca *CustomArray) size() int {
	return len(ca.elements)
}

// Check if the custom array is empty
func (ca *CustomArray) isEmpty() bool {
	return len(ca.elements) == 0
}

// Return the index of the first occurrence of the specified element, or -1 if not found
func (ca *CustomArray) index(element interface{}) int {
	for i, e := range ca.elements {
		if e == element {
			return i
		}
	}
	return -1
}

// Return the number of occurrences of the specified element in the custom array
func (ca *CustomArray) count(element interface{}) int {
	count := 0
	for _, e := range ca.elements {
		if e == element {
			count++
		}
	}
	return count
}

// Reverse the order of elements in the custom array
func (ca *CustomArray) reverse() {
	for i, j := 0, len(ca.elements)-1; i < j; i, j = i+1, j-1 {
		ca.elements[i], ca.elements[j] = ca.elements[j], ca.elements[i]
	}
}

// Sort the elements in the custom array in ascending order
func (ca *CustomArray) sort() {
	// TODO: Implement sorting based on element type (if Comparable)
}

// Return a new slice containing elements from the specified start index to the end index
func (ca *CustomArray) slice(start, end int) []interface{} {
	if start >= 0 && end < len(ca.elements) && start <= end {
		return ca.elements[start : end+1]
	}
	return []interface{}{}
}

func main() {
	customArray := NewCustomArray(5)
	customArray.append(10)
	customArray.append(20)
	customArray.insert(15, 1)

	fmt.Printf("Size of array: %d\n", customArray.size())

	// Print each element
	for _, element := range customArray.elements {
		fmt.Println(element)
	}
}
