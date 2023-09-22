package main

import (
	"fmt"
	"strings"
)

type TreeNode struct {
	value int
	left  *TreeNode
	right *TreeNode
}

type BinarySearchTree struct {
	root *TreeNode
}

// MARK: - Insertion

func (bst *BinarySearchTree) insert(value int) {
	bst.root = bst.insertRec(bst.root, value)
}

func (bst *BinarySearchTree) insertRec(node *TreeNode, value int) *TreeNode {
	if node == nil {
		return &TreeNode{value: value}
	}

	if value < node.value {
		node.left = bst.insertRec(node.left, value)
	} else if value > node.value {
		node.right = bst.insertRec(node.right, value)
	}

	return node
}

// MARK: - Deletion

func (bst *BinarySearchTree) delete(value int) {
	bst.root = bst.deleteRec(bst.root, value)
}

func (bst *BinarySearchTree) deleteRec(node *TreeNode, value int) *TreeNode {
	if node == nil {
		return nil
	}

	if value < node.value {
		node.left = bst.deleteRec(node.left, value)
	} else if value > node.value {
		node.right = bst.deleteRec(node.right, value)
	} else {
		if node.left == nil {
			return node.right
		} else if node.right == nil {
			return node.left
		}

		minRight := bst.findMin(node.right)
		node.value = minRight.value
		node.right = bst.deleteRec(node.right, minRight.value)
	}

	return node
}

// Helper method to find the minimum value node in a subtree
func (bst *BinarySearchTree) findMin(node *TreeNode) *TreeNode {
	current := node
	for current.left != nil {
		current = current.left
	}
	return current
}

// MARK: - Search

func (bst *BinarySearchTree) search(value int) bool {
	return bst.searchRec(bst.root, value)
}

func (bst *BinarySearchTree) searchRec(node *TreeNode, value int) bool {
	if node == nil {
		return false
	}

	if value == node.value {
		return true
	} else if value < node.value {
		return bst.searchRec(node.left, value)
	} else {
		return bst.searchRec(node.right, value)
	}
}

// MARK: - Traversal

func (bst *BinarySearchTree) inorderTraversal() []int {
	result := []int{}
	bst.inorderTraversalRec(bst.root, &result)
	return result
}

func (bst *BinarySearchTree) inorderTraversalRec(node *TreeNode, result *[]int) {
	if node != nil {
		bst.inorderTraversalRec(node.left, result)
		*result = append(*result, node.value)
		bst.inorderTraversalRec(node.right, result)
	}
}

// MARK: - Printing

func (bst *BinarySearchTree) printBinarySearchTree() {
	builder := strings.Builder{}
	bst.printRec(bst.root, &builder)
	fmt.Println("Binary Search Tree (In-order traversal):", builder.String())
}

func (bst *BinarySearchTree) printRec(node *TreeNode, builder *strings.Builder) {
	if node != nil {
		bst.printRec(node.left, builder)
		builder.WriteString(fmt.Sprintf("%d ", node.value))
		bst.printRec(node.right, builder)
	}
}

// ... Add more BST operations as needed ...
