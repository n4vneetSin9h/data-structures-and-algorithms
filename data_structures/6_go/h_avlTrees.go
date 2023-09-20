package main

import (
	"fmt"
)

// AVLNode represents a node in the AVL tree
type AVLNode struct {
	value  int
	height int
	left   *AVLNode
	right  *AVLNode
}

// AVLTree represents the AVL tree structure
type AVLTree struct {
	root *AVLNode
}

// newAVLNode creates a new AVL node with the given value
func newAVLNode(value int) *AVLNode {
	return &AVLNode{
		value:  value,
		height: 1,
		left:   nil,
		right:  nil,
	}
}

// height returns the height of the node
func height(node *AVLNode) int {
	if node == nil {
		return 0
	}
	return node.height
}

// max returns the maximum of two integers
func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}

// updateHeight updates the height of a node based on its children's heights
func updateHeight(node *AVLNode) {
	node.height = max(height(node.left), height(node.right)) + 1
}

// balanceFactor calculates the balance factor of a node
func balanceFactor(node *AVLNode) int {
	return height(node.left) - height(node.right)
}

// rotateLeft performs a left rotation on the given node
func rotateLeft(y *AVLNode) *AVLNode {
	x := y.right
	T2 := x.left

	x.left = y
	y.right = T2

	updateHeight(y)
	updateHeight(x)

	return x
}

// rotateRight performs a right rotation on the given node
func rotateRight(x *AVLNode) *AVLNode {
	y := x.left
	T2 := y.right

	y.right = x
	x.left = T2

	updateHeight(x)
	updateHeight(y)

	return y
}

// balance balances the AVL tree starting from the given node
func balance(node *AVLNode) *AVLNode {
	balanceFactorInt := balanceFactor(node)

	if balanceFactorInt > 1 {
		if balanceFactor(node.left) < 0 {
			node.left = rotateLeft(node.left)
		}
		return rotateRight(node)
	}

	if balanceFactorInt < -1 {
		if balanceFactor(node.right) > 0 {
			node.right = rotateRight(node.right)
		}
		return rotateLeft(node)
	}

	return node
}

// insert inserts a value into the AVL tree and returns the new root of the subtree
func insert(node *AVLNode, value int) *AVLNode {
	if node == nil {
		return newAVLNode(value)
	}

	if value < node.value {
		node.left = insert(node.left, value)
	} else {
		node.right = insert(node.right, value)
	}

	updateHeight(node)

	return balance(node)
}

// Insert inserts a value into the AVL tree
func (avl *AVLTree) Insert(value int) {
	avl.root = insert(avl.root, value)
}

// inorderTraversal performs an inorder traversal of the AVL tree and executes the visit function on each node's value
func inorderTraversal(node *AVLNode, visit func(int)) {
	if node == nil {
		return
	}

	inorderTraversal(node.left, visit)
	visit(node.value)
	inorderTraversal(node.right, visit)
}

// InorderTraversal performs an inorder traversal of the AVL tree and executes the visit function on each node's value
func (avl *AVLTree) InorderTraversal(visit func(int)) {
	inorderTraversal(avl.root, visit)
}

// printAVLTree performs an inorder traversal of the AVL tree and prints the values
func printAVLTree(node *AVLNode) {
	if node == nil {
		return
	}

	printAVLTree(node.left)
	fmt.Printf("%d ", node.value)
	printAVLTree(node.right)
}
