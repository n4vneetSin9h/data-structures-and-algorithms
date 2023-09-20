package main

import (
	"fmt"
)

type Node struct {
	value interface{} // Value of the node
	next  *Node       // Pointer to the next node in the list
}

type LinkedList struct {
	head *Node // Pointer to the first node in the list
	tail *Node // Pointer to the last node in the list
}

// Get the value of the node
func (n *Node) Value() interface{} {
	return n.value
}

// Get the next node
func (n *Node) Next() *Node {
	return n.next
}

// Create a new node with the given value
func NewNode(value interface{}) *Node {
	return &Node{value: value, next: nil}
}

// Create a new empty linked list
func NewLinkedList() *LinkedList {
	return &LinkedList{head: nil, tail: nil}
}

// Check if the linked list is empty
func (l *LinkedList) IsEmpty() bool {
	return l.head == nil
}

// Get the first node in the linked list
func (l *LinkedList) First() *Node {
	return l.head
}

// Get the last node in the linked list
func (l *LinkedList) Last() *Node {
	return l.tail
}

// Append a new node with the given value to the end of the linked list
func (l *LinkedList) Append(value interface{}) {
	newNode := NewNode(value)

	if l.tail != nil {
		l.tail.next = newNode
	} else {
		l.head = newNode
	}

	l.tail = newNode
}

// Prepend a new node with the given value to the beginning of the linked list
func (l *LinkedList) Prepend(value interface{}) {
	newNode := NewNode(value)

	if l.head != nil {
		newNode.next = l.head
	} else {
		l.tail = newNode
	}

	l.head = newNode
}

// Check if the linked list contains a node with the given value
func (l *LinkedList) Contains(value interface{}) bool {
	current := l.head

	for current != nil {
		if current.value == value {
			return true
		}
		current = current.next
	}

	return false
}

// Get the node at the specified index in the linked list
func (l *LinkedList) NodeAt(index int) *Node {
	if index < 0 {
		return nil
	}

	current := l.head
	currentIndex := 0

	for current != nil && currentIndex < index {
		current = current.next
		currentIndex++
	}

	return current
}

// Print the linked list
func (l *LinkedList) PrintList() {
	current := l.head

	for current != nil {
		fmt.Printf("%v -> ", current.value)
		current = current.next
	}
	fmt.Println("nil")
}
