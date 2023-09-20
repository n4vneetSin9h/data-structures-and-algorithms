class TreeNode<T : Comparable<T>>(val value: T) {
    var left: TreeNode<T>? = null
    var right: TreeNode<T>? = null
}

class BinarySearchTree<T : Comparable<T>> {
    private var root: TreeNode<T>? = null

    // MARK: - Insertion

    fun insert(value: T) {
        root = insertRec(root, value)
    }

    private fun insertRec(node: TreeNode<T>?, value: T): TreeNode<T> {
        if (node == null) {
            return TreeNode(value)
        }

        if (value < node.value) {
            node.left = insertRec(node.left, value)
        } else if (value > node.value) {
            node.right = insertRec(node.right, value)
        }

        return node
    }

    // MARK: - Deletion

    fun delete(value: T) {
        root = deleteRec(root, value)
    }

    private fun deleteRec(node: TreeNode<T>?, value: T): TreeNode<T>? {
        if (node == null) {
            return null
        }

        if (value < node.value) {
            node.left = deleteRec(node.left, value)
        } else if (value > node.value) {
            node.right = deleteRec(node.right, value)
        } else {
            if (node.left == null) {
                return node.right
            } else if (node.right == null) {
                return node.left
            }

            val minRight = findMin(node.right!!)
            node.value = minRight.value
            node.right = deleteRec(node.right, minRight.value)
        }

        return node
    }

    // Helper method to find the minimum value node in a subtree
    private fun findMin(node: TreeNode<T>): TreeNode<T> {
        var current = node
        while (current.left != null) {
            current = current.left!!
        }
        return current
    }

    // MARK: - Search

    fun search(value: T): Boolean {
        return searchRec(root, value)
    }

    private fun searchRec(node: TreeNode<T>?, value: T): Boolean {
        if (node == null) {
            return false
        }

        if (value == node.value) {
            return true
        } else if (value < node.value) {
            return searchRec(node.left, value)
        } else {
            return searchRec(node.right, value)
        }
    }

    // MARK: - Traversal

    fun inorderTraversal(): List<T> {
        val result = mutableListOf<T>()
        inorderTraversalRec(root, result)
        return result
    }

    private fun inorderTraversalRec(node: TreeNode<T>?, result: MutableList<T>) {
        if (node != null) {
            inorderTraversalRec(node.left, result)
            result.add(node.value)
            inorderTraversalRec(node.right, result)
        }
    }

    // ... Add more BST operations as needed ...
}

// Example usage
fun main() {
    val bst = BinarySearchTree<Int>()

    bst.insert(5)
    bst.insert(3)
    bst.insert(7)
    bst.insert(1)
    bst.insert(4)

    println("In-order traversal: ${bst.inorderTraversal()}")
    println("Search 4: ${bst.search(4)}")
    println("Search 6: ${bst.search(6)}")

    bst.delete(3)

    println("In-order traversal after deletion: ${bst.inorderTraversal()}")
}
