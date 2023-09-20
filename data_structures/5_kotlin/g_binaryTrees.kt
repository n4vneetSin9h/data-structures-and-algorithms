import kotlin.random.Random

class TreeNode<T>(val value: T) {
    var left: TreeNode<T>? = null
    var right: TreeNode<T>? = null
}

class BinaryTree<T : Comparable<T>> {
    var root: TreeNode<T>? = null

    // Insertion
    fun insert(value: T) {
        root = insertRec(root, value)
    }

    private fun insertRec(node: TreeNode<T>?, value: T): TreeNode<T> {
        if (node != null) {
            if (node.left == null) {
                node.left = insertRec(null, value)
            } else if (node.right == null) {
                node.right = insertRec(null, value)
            } else {
                if (Random.nextBoolean()) {
                    node.left = insertRec(node.left, value)
                } else {
                    node.right = insertRec(node.right, value)
                }
            }
            return node
        } else {
            return TreeNode(value)
        }
    }

    // In-order Traversal
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

    // Search
    fun search(value: T): Boolean {
        return searchRec(root, value)
    }

    private fun searchRec(node: TreeNode<T>?, value: T): Boolean {
        if (node == null) {
            return false
        }

        if (value == node.value) {
            return true
        }

        return searchRec(node.left, value) || searchRec(node.right, value)
    }

    // Height
    fun height(): Int {
        return heightRec(root)
    }

    private fun heightRec(node: TreeNode<T>?): Int {
        if (node == null) {
            return 0
        }

        return maxOf(heightRec(node.left), heightRec(node.right)) + 1
    }

    // Deletion
    fun delete(value: T) {
        root = deleteRec(root, value)
    }

    private fun deleteRec(node: TreeNode<T>?, value: T): TreeNode<T>? {
        if (node == null) {
            return null
        }

        if (value == node.value) {
            if (node.left == null) {
                return node.right
            } else if (node.right == null) {
                return node.left
            }

            val minValue = findMin(node.right!!)
            node.value = minValue.value
            node.right = deleteRec(node.right, minValue.value)
        } else if (value < node.value) {
            node.left = deleteRec(node.left, value)
        } else {
            node.right = deleteRec(node.right, value)
        }

        return node
    }

    private fun findMin(node: TreeNode<T>): TreeNode<T> {
        var current = node
        while (current.left != null) {
            current = current.left!!
        }
        return current
    }

    // Mirror
    fun mirror() {
        mirrorRec(root)
    }

    private fun mirrorRec(node: TreeNode<T>?) {
        if (node != null) {
            val temp = node.left
            node.left = node.right
            node.right = temp

            mirrorRec(node.left)
            mirrorRec(node.right)
        }
    }

    // Node Count
    fun nodeCount(): Int {
        return nodeCountRec(root)
    }

    private fun nodeCountRec(node: TreeNode<T>?): Int {
        if (node == null) {
            return 0
        }

        return 1 + nodeCountRec(node.left) + nodeCountRec(node.right)
    }

    // Level Order Traversal
    fun levelOrderTraversal(): List<T> {
        val result = mutableListOf<T>()
        if (root != null) {
            val queue: MutableList<TreeNode<T>> = mutableListOf(root!!)

            while (queue.isNotEmpty()) {
                val node = queue.removeAt(0)
                result.add(node.value)

                if (node.left != null) {
                    queue.add(node.left!!)
                }

                if (node.right != null) {
                    queue.add(node.right!!)
                }
            }
        }
        return result
    }

    // Check if Balanced
    fun isBalanced(): Boolean {
        return isBalancedRec(root) != -1
    }

    private fun isBalancedRec(node: TreeNode<T>?): Int {
        if (node == null) {
            return 0
        }

        val leftHeight = isBalancedRec(node.left)
        val rightHeight = isBalancedRec(node.right)

        if (leftHeight == -1 || rightHeight == -1 || Math.abs(leftHeight - rightHeight) > 1) {
            return -1
        }

        return maxOf(leftHeight, rightHeight) + 1
    }
}

fun main() {
    // Example usage
    val tree = BinaryTree<Int>()
    tree.insert(5)
    tree.insert(3)
    tree.insert(7)
    tree.insert(1)
    tree.insert(4)
    tree.insert(6)
    tree.insert(8)

    println("In-order traversal: ${tree.inorderTraversal()}")
    println("Tree height: ${tree.height()}")
    println("Is 4 present in the tree? ${tree.search(4)}")
    println("Node count: ${tree.nodeCount()}")
    println("Level order traversal: ${tree.levelOrderTraversal()}")

    tree.mirror()

    println("In-order traversal after mirroring: ${tree.inorderTraversal()}")
    println("Is the tree balanced? ${tree.isBalanced()}")
}
