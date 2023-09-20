class TreeNode<T> {
    var value: T
    var left: TreeNode?
    var right: TreeNode?

    init(value: T) {
        self.value = value
    }
}

class BinaryTree<T> {
    var root: TreeNode<T>?

    // MARK: - Insertion

    /// Insert a value into the binary tree.
    func insert(value: T) {
        root = insertRec(root, value)
    }

    private func insertRec(_ node: TreeNode<T>?, _ value: T) -> TreeNode<T> {
        if let node = node {
            if node.left == nil {
                node.left = insertRec(nil, value)
            } else if node.right == nil {
                node.right = insertRec(nil, value)
            } else {
                if arc4random_uniform(2) == 0 {
                    node.left = insertRec(node.left, value)
                } else {
                    node.right = insertRec(node.right, value)
                }
            }
            return node
        } else {
            return TreeNode(value: value)
        }
    }

    // MARK: - Traversals

    /// Perform in-order traversal of the binary tree.
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

    // MARK: - Search

    /// Search for a value in the binary tree.
    func search(value: T) -> Bool {
        return searchRec(root, value)
    }

    private func searchRec(_ node: TreeNode<T>?, _ value: T) -> Bool {
        guard let node = node else {
            return false
        }

        if value == node.value {
            return true
        }

        return searchRec(node.left, value) || searchRec(node.right, value)
    }

    // MARK: - Height

    /// Calculate the height of the binary tree.
    func height() -> Int {
        return heightRec(root)
    }

    private func heightRec(_ node: TreeNode<T>?) -> Int {
        guard let node = node else {
            return 0
        }

        return max(heightRec(node.left), heightRec(node.right)) + 1
    }

    // MARK: - Deletion

    /// Delete a value from the binary tree.
    func delete(value: T) {
        root = deleteRec(root, value)
    }

    private func deleteRec(_ node: TreeNode<T>?, _ value: T) -> TreeNode<T>? {
        guard let node = node else {
            return nil
        }

        if value == node.value {
            // Node to be deleted found
            // If the node has only one child or no child
            if node.left == nil {
                return node.right
            } else if node.right == nil {
                return node.left
            }

            // Node has two children
            let minValue = findMin(node.right!)
            node.value = minValue.value
            node.right = deleteRec(node.right, minValue.value)
        } else if value < node.value {
            node.left = deleteRec(node.left, value)
        } else {
            node.right = deleteRec(node.right, value)
        }

        return node
    }

    // MARK: - Mirror

    /// Create a mirror image of the binary tree.
    func mirror() {
        mirrorRec(root)
    }

    private func mirrorRec(_ node: TreeNode<T>?) {
        guard let node = node else {
            return
        }

        let temp = node.left
        node.left = node.right
        node.right = temp

        mirrorRec(node.left)
        mirrorRec(node.right)
    }

    // MARK: - Node Count

    /// Count the number of nodes in the binary tree.
    func nodeCount() -> Int {
        return nodeCountRec(root)
    }

    private func nodeCountRec(_ node: TreeNode<T>?) -> Int {
        guard let node = node else {
            return 0
        }

        return 1 + nodeCountRec(node.left) + nodeCountRec(node.right)
    }

    // MARK: - Level Order Traversal

    /// Perform level-order traversal of the binary tree.
    func levelOrderTraversal() -> [T] {
        var result: [T] = []
        if let rootNode = root {
            var queue: [TreeNode<T>] = [rootNode]

            while !queue.isEmpty {
                let node = queue.removeFirst()
                result.append(node.value)

                if let left = node.left {
                    queue.append(left)
                }

                if let right = node.right {
                    queue.append(right)
                }
            }
        }
        return result
    }

    // MARK: - Check if Balanced

    /// Check if the binary tree is balanced.
    func isBalanced() -> Bool {
        return isBalancedRec(root) != -1
    }

    private func isBalancedRec(_ node: TreeNode<T>?) -> Int {
        guard let node = node else {
            return 0
        }

        let leftHeight = isBalancedRec(node.left)
        let rightHeight = isBalancedRec(node.right)

        if leftHeight == -1 || rightHeight == -1 || abs(leftHeight - rightHeight) > 1 {
            return -1
        }

        return max(leftHeight, rightHeight) + 1
    }

    // ... Add more binary tree operations as needed ...
}
