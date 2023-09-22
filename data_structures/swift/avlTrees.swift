class AVLNode<T: Comparable> {
    var value: T
    var height: Int
    var left: AVLNode?
    var right: AVLNode?

    init(value: T) {
        self.value = value
        self.height = 1
    }
}

class AVLTree<T: Comparable> {
    private var root: AVLNode<T>?

    // Get the height of a node
    private func height(_ node: AVLNode<T>?) -> Int {
        return node?.height ?? 0
    }

    // Update the height of a node based on its children's heights
    private func updateHeight(_ node: AVLNode<T>) {
        node.height = max(height(node.left), height(node.right)) + 1
    }

    // Get the balance factor for a node (difference in height of left and right subtrees)
    private func balanceFactor(_ node: AVLNode<T>) -> Int {
        return height(node.left) - height(node.right)
    }

    // Rotate a node to the left
    private func rotateLeft(_ y: AVLNode<T>) -> AVLNode<T> {
        let x = y.right!
        let T2 = x.left

        x.left = y
        y.right = T2

        updateHeight(y)
        updateHeight(x)

        return x
    }

    // Rotate a node to the right
    private func rotateRight(_ x: AVLNode<T>) -> AVLNode<T> {
        let y = x.left!
        let T2 = y.right

        y.right = x
        x.left = T2

        updateHeight(x)
        updateHeight(y)

        return y
    }

    // Balance the tree by rotating nodes if needed
    private func balance(_ node: AVLNode<T>) -> AVLNode<T> {
        let balanceFactor = self.balanceFactor(node)

        if balanceFactor > 1 {
            if let leftChild = node.left, balanceFactor(leftChild) < 0 {
                node.left = rotateLeft(leftChild)
            }
            return rotateRight(node)
        }

        if balanceFactor < -1 {
            if let rightChild = node.right, balanceFactor(rightChild) > 0 {
                node.right = rotateRight(rightChild)
            }
            return rotateLeft(node)
        }

        return node
    }

    // Recursive function to insert a value into the AVL tree
    private func insert(_ node: AVLNode<T>?, value: T) -> AVLNode<T> {
        guard let node = node else {
            return AVLNode(value: value)
        }

        if value < node.value {
            node.left = insert(node.left, value: value)
        } else {
            node.right = insert(node.right, value: value)
        }

        updateHeight(node)

        return balance(node)
    }

    // Public function to insert a value into the AVL tree
    func insert(value: T) {
        root = insert(root, value: value)
    }

    // Recursive inorder traversal and print the values
    private func inorderTraversal(_ node: AVLNode<T>?, visit: (T) -> Void) {
        guard let node = node else { return }

        inorderTraversal(node.left, visit: visit)
        visit(node.value)
        inorderTraversal(node.right, visit: visit)
    }

    // Public function to perform inorder traversal and print the values
    func inorderTraversal(visit: (T) -> Void) {
        inorderTraversal(root, visit: visit)
    }
}

// Example usage
let avlTree = AVLTree<Int>()
avlTree.insert(value: 10)
avlTree.insert(value: 5)
avlTree.insert(value: 15)
avlTree.insert(value: 3)
avlTree.insert(value: 7)

avlTree.inorderTraversal { print($0) }
