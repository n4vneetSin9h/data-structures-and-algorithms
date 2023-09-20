class AVLNode<T : Comparable<T>>(val value: T) {
    var height: Int = 1
    var left: AVLNode<T>? = null
    var right: AVLNode<T>? = null
}

class AVLTree<T : Comparable<T>> {
    private var root: AVLNode<T>? = null

    // Get the height of a node
    private fun height(node: AVLNode<T>?): Int {
        return node?.height ?: 0
    }

    // Update the height of a node based on its children's heights
    private fun updateHeight(node: AVLNode<T>) {
        node.height = maxOf(height(node.left), height(node.right)) + 1
    }

    // Get the balance factor for a node (difference in height of left and right subtrees)
    private fun balanceFactor(node: AVLNode<T>): Int {
        return height(node.left) - height(node.right)
    }

    // Rotate a node to the left
    private fun rotateLeft(y: AVLNode<T>): AVLNode<T> {
        val x = y.right!!
        val T2 = x.left

        x.left = y
        y.right = T2

        updateHeight(y)
        updateHeight(x)

        return x
    }

    // Rotate a node to the right
    private fun rotateRight(x: AVLNode<T>): AVLNode<T> {
        val y = x.left!!
        val T2 = y.right

        y.right = x
        x.left = T2

        updateHeight(x)
        updateHeight(y)

        return y
    }

    // Balance the tree by rotating nodes if needed
    private fun balance(node: AVLNode<T>): AVLNode<T> {
        val balanceFactor = balanceFactor(node)

        if (balanceFactor > 1) {
            if (node.left != null && balanceFactor(node.left!!) < 0) {
                node.left = rotateLeft(node.left!!)
            }
            return rotateRight(node)
        }

        if (balanceFactor < -1) {
            if (node.right != null && balanceFactor(node.right!!) > 0) {
                node.right = rotateRight(node.right!!)
            }
            return rotateLeft(node)
        }

        return node
    }

    // Recursive function to insert a value into the AVL tree
    private fun insert(node: AVLNode<T>?, value: T): AVLNode<T> {
        if (node == null) {
            return AVLNode(value)
        }

        if (value < node.value) {
            node.left = insert(node.left, value)
        } else {
            node.right = insert(node.right, value)
        }

        updateHeight(node)

        return balance(node)
    }

    // Public function to insert a value into the AVL tree
    fun insert(value: T) {
        root = insert(root, value)
    }

    // Recursive inorder traversal and execute the visit function on each node's value
    private fun inorderTraversal(node: AVLNode<T>?, visit: (T) -> Unit) {
        if (node == null) {
            return
        }

        inorderTraversal(node.left, visit)
        visit(node.value)
        inorderTraversal(node.right, visit)
    }

    // Public function to perform inorder traversal and execute the visit function on each node's value
    fun inorderTraversal(visit: (T) -> Unit) {
        inorderTraversal(root, visit)
    }
}

// Example usage
fun main() {
    val avlTree = AVLTree<Int>()

    // Insert some values into the AVL tree
    avlTree.insert(10)
    avlTree.insert(5)
    avlTree.insert(15)
    avlTree.insert(3)
    avlTree.insert(7)

    // Inorder traversal to print the values
    avlTree.inorderTraversal { println(it) }
}
