class TreeNode<T: Comparable> {
    var value: T
    var left: TreeNode?
    var right: TreeNode?

    init(value: T) {
        self.value = value
    }
}

class BinarySearchTree<T: Comparable> {
    private var root: TreeNode<T>?

    // MARK: - Insertion

    /// Insert a value into the binary search tree.
    func insert(_ value: T) {
        root = insertRec(root, value)
    }

    private func insertRec(_ node: TreeNode<T>?, _ value: T) -> TreeNode<T> {
        guard let node = node else {
            return TreeNode(value: value)
        }

        if value < node.value {
            node.left = insertRec(node.left, value)
        } else if value > node.value {
            node.right = insertRec(node.right, value)
        }

        return node
    }

    // MARK: - Deletion

    /// Remove a value from the binary search tree.
    func delete(_ value: T) {
        root = deleteRec(root, value)
    }

    private func deleteRec(_ node: TreeNode<T>?, _ value: T) -> TreeNode<T>? {
        guard let node = node else {
            return nil
        }

        if value < node.value {
            node.left = deleteRec(node.left, value)
        } else if value > node.value {
            node.right = deleteRec(node.right, value)
        } else {
            if node.left == nil {
                return node.right
            } else if node.right == nil {
                return node.left
            }

            let minRight = findMin(node.right!)
            node.value = minRight.value
            node.right = deleteRec(node.right, minRight.value)
        }

        return node
    }

    // Helper method to find the minimum value node in a subtree
    private func findMin(_ node: TreeNode<T>) -> TreeNode<T> {
        var current = node
        while let left = current.left {
            current = left
        }
        return current
    }

    // MARK: - Search

    /// Search for a value in the binary search tree.
    func search(_ value: T) -> Bool {
        return searchRec(root, value)
    }

    private func searchRec(_ node: TreeNode<T>?, _ value: T) -> Bool {
        guard let node = node else {
            return false
        }

        if value == node.value {
            return true
        } else if value < node.value {
            return searchRec(node.left, value)
        } else {
            return searchRec(node.right, value)
        }
    }

    // MARK: - Traversal

    /// Perform in-order traversal of the binary search tree.
    func inorderTraversal() -> [T] {
        var result: [T] = []
        inorderTraversalRec(root, &result)
        return result
    }

    private func inorderTraversalRec(_ node: TreeNode<T>?, _ result: inout [T]) {
        guard let node = node else {
            return
        }

        inorderTraversalRec(node.left, &result)
        result.append(node.value)
        inorderTraversalRec(node.right, &result)
    }

    // ... Add more BST operations as needed ...
}
