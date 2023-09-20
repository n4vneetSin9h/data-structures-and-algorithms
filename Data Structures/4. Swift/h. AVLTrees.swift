class AVLNode<T> {
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

    // MARK: - Insertion

    /// Insert a value into the AVL tree.
    func insert(value: T) {
        root = insertRec(root, value)
    }

    private func insertRec(_ node: AVLNode<T>?, _ value: T) -> AVLNode<T> {
        // ... Implementation of AVL insertion ...
    }

    // MARK: - Deletion

    /// Delete a value from the AVL tree.
    func delete(value: T) {
        root = deleteRec(root, value)
    }

    private func deleteRec(_ node: AVLNode<T>?, _ value: T) -> AVLNode<T>? {
        // ... Implementation of AVL deletion ...
    }

    // MARK: - Rotation

    /// Perform a left rotation at the given node.
    private func leftRotate(_ y: AVLNode<T>?) -> AVLNode<T>? {
        // ... Implementation of left rotation ...
    }

    /// Perform a right rotation at the given node.
    private func rightRotate(_ x: AVLNode<T>?) -> AVLNode<T>? {
        // ... Implementation of right rotation ...
    }

    // MARK: - Height and Balance Factor

    /// Get the height of the node.
    private func height(_ node: AVLNode<T>?) -> Int {
        return node?.height ?? 0
    }

    /// Get the balance factor of the node.
    private func getBalance(_ node: AVLNode<T>?) -> Int {
        // ... Implementation of balance factor calculation ...
    }

    // MARK: - Traversals

    /// Perform in-order traversal of the AVL tree.
    func inorderTraversal() -> [T] {
        // ... Implementation of in-order traversal ...
    }

    // ... Add more AVL tree operations as needed ...
}
