class TreeNode:
    def __init__(self, value):
        self.value = value
        self.left = None
        self.right = None


class BinarySearchTree:
    def __init__(self):
        self.root = None

    # MARK: - Insertion

    def insert(self, value):
        self.root = self._insert_rec(self.root, value)

    def _insert_rec(self, node, value):
        if node is None:
            return TreeNode(value)

        if value < node.value:
            node.left = self._insert_rec(node.left, value)
        elif value > node.value:
            node.right = self._insert_rec(node.right, value)

        return node

    # MARK: - Deletion

    def delete(self, value):
        self.root = self._delete_rec(self.root, value)

    def _delete_rec(self, node, value):
        if node is None:
            return None

        if value < node.value:
            node.left = self._delete_rec(node.left, value)
        elif value > node.value:
            node.right = self._delete_rec(node.right, value)
        else:
            if node.left is None:
                return node.right
            elif node.right is None:
                return node.left

            min_right = self._find_min(node.right)
            node.value = min_right.value
            node.right = self._delete_rec(node.right, min_right.value)

        return node

    def _find_min(self, node):
        current = node
        while current.left:
            current = current.left
        return current

    # MARK: - Search

    def search(self, value):
        return self._search_rec(self.root, value)

    def _search_rec(self, node, value):
        if node is None:
            return False

        if value == node.value:
            return True
        elif value < node.value:
            return self._search_rec(node.left, value)
        else:
            return self._search_rec(node.right, value)

    # MARK: - Traversal

    def inorder_traversal(self):
        result = []
        self._inorder_traversal_rec(self.root, result)
        return result

    def _inorder_traversal_rec(self, node, result):
        if node:
            self._inorder_traversal_rec(node.left, result)
            result.append(node.value)
            self._inorder_traversal_rec(node.right, result)

    # ... Add more BST operations as needed ...


# Example usage
if __name__ == "__main__":
    bst = BinarySearchTree()

    bst.insert(5)
    bst.insert(3)
    bst.insert(7)
    bst.insert(1)
    bst.insert(4)

    print("In-order traversal:", bst.inorder_traversal())
    print("Search 4:", bst.search(4))
    print("Search 6:", bst.search(6))

    bst.delete(3)

    print("In-order traversal after deletion:", bst.inorder_traversal())
