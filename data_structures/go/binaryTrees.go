package main

import (
	"fmt"
	"math"
	"math/rand"
)

type BinaryTree struct {
	root *TreeNode
}

func NewTreeNode(value int) *TreeNode {
	return &TreeNode{value, nil, nil}
}

func NewBinaryTree() *BinaryTree {
	return &BinaryTree{nil}
}

// Insertion
func (bt *BinaryTree) insert(value int) {
	bt.root = bt.insertRec(bt.root, value)
}

func (bt *BinaryTree) insertRec(node *TreeNode, value int) *TreeNode {
	if node != nil {
		if node.left == nil {
			node.left = bt.insertRec(nil, value)
		} else if node.right == nil {
			node.right = bt.insertRec(nil, value)
		} else {
			if rand.Intn(2) == 0 {
				node.left = bt.insertRec(node.left, value)
			} else {
				node.right = bt.insertRec(node.right, value)
			}
		}
		return node
	} else {
		return NewTreeNode(value)
	}
}

// In-order Traversal
func (bt *BinaryTree) inorderTraversal() []int {
	var result []int
	bt.inorderTraversalRec(bt.root, &result)
	return result
}

func (bt *BinaryTree) inorderTraversalRec(node *TreeNode, result *[]int) {
	if node != nil {
		bt.inorderTraversalRec(node.left, result)
		*result = append(*result, node.value)
		bt.inorderTraversalRec(node.right, result)
	}
}

// Search
func (bt *BinaryTree) search(value int) bool {
	return bt.searchRec(bt.root, value)
}

func (bt *BinaryTree) searchRec(node *TreeNode, value int) bool {
	if node == nil {
		return false
	}

	if value == node.value {
		return true
	}

	return bt.searchRec(node.left, value) || bt.searchRec(node.right, value)
}

// Height
func (bt *BinaryTree) height() int {
	return bt.heightRec(bt.root)
}

func (bt *BinaryTree) heightRec(node *TreeNode) int {
	if node == nil {
		return 0
	}

	leftHeight := bt.heightRec(node.left)
	rightHeight := bt.heightRec(node.right)

	if leftHeight > rightHeight {
		return leftHeight + 1
	}
	return rightHeight + 1
}

// Deletion
func (bt *BinaryTree) delete(value int) {
	bt.root = bt.deleteRec(bt.root, value)
}

func (bt *BinaryTree) deleteRec(node *TreeNode, value int) *TreeNode {
	if node == nil {
		return nil
	}

	if value == node.value {
		if node.left == nil {
			return node.right
		} else if node.right == nil {
			return node.left
		}

		minValueNode := bt.findMin(node.right)
		node.value = minValueNode.value
		node.right = bt.deleteRec(node.right, minValueNode.value)
	} else if value < node.value {
		node.left = bt.deleteRec(node.left, value)
	} else {
		node.right = bt.deleteRec(node.right, value)
	}

	return node
}

func (bt *BinaryTree) findMin(node *TreeNode) *TreeNode {
	current := node
	for current.left != nil {
		current = current.left
	}
	return current
}

// Mirror
func (bt *BinaryTree) mirror() {
	bt.mirrorRec(bt.root)
}

func (bt *BinaryTree) mirrorRec(node *TreeNode) {
	if node != nil {
		node.left, node.right = node.right, node.left
		bt.mirrorRec(node.left)
		bt.mirrorRec(node.right)
	}
}

// Node Count
func (bt *BinaryTree) nodeCount() int {
	return bt.nodeCountRec(bt.root)
}

func (bt *BinaryTree) nodeCountRec(node *TreeNode) int {
	if node == nil {
		return 0
	}

	return 1 + bt.nodeCountRec(node.left) + bt.nodeCountRec(node.right)
}

// Level Order Traversal
func (bt *BinaryTree) levelOrderTraversal() []int {
	var result []int
	if bt.root != nil {
		queue := []*TreeNode{bt.root}

		for len(queue) > 0 {
			node := queue[0]
			queue = queue[1:]
			result = append(result, node.value)

			if node.left != nil {
				queue = append(queue, node.left)
			}

			if node.right != nil {
				queue = append(queue, node.right)
			}
		}
	}
	return result
}

// Check if Balanced
func (bt *BinaryTree) isBalanced() bool {
	return bt.isBalancedRec(bt.root) != -1
}

func (bt *BinaryTree) isBalancedRec(node *TreeNode) int {
	if node == nil {
		return 0
	}

	leftHeight := bt.isBalancedRec(node.left)
	rightHeight := bt.isBalancedRec(node.right)

	if leftHeight == -1 || rightHeight == -1 || math.Abs(float64(leftHeight-rightHeight)) > 1 {
		return -1
	}

	if leftHeight > rightHeight {
		return leftHeight + 1
	}
	return rightHeight + 1
}

// PrintBinaryTree prints the binary tree
func (bt *BinaryTree) printBinaryTree() {
	printBinaryTreeRec(bt.root, 0)
}

func printBinaryTreeRec(node *TreeNode, level int) {
	if node == nil {
		return
	}

	printBinaryTreeRec(node.right, level+1)
	for i := 0; i < level; i++ {
		fmt.Print("\t")
	}
	fmt.Println(node.value)
	printBinaryTreeRec(node.left, level+1)
}
