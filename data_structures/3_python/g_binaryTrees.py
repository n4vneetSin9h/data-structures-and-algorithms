import random

class TreeNode:
    def __init__(self, value):
        self.value = value
        self.left = None
        self.right = None

class BinaryTree:
    def __init__(self):
        self.root = None

    # Insertion
    def insert(self, value):
        self.root = self._insert_rec(self.root, value)

    def _insert_rec(self, node, value):
        if node:
            if node.left is None:
                node.left = self._insert_rec(None, value)
            elif node.right is None:
                node.right = self._insert_rec(None, value)
            else:
                if random.randint(0, 1) == 0:
                    node.left = self._insert_rec(node.left, value)
                else:
                    node.right = self._insert_rec(node.right, value)
            return node
        else:
            return TreeNode(value)

    # In-order Traversal
    def inorder_traversal(self):
        result = []
        self._inorder_traversal_rec(self.root, result)
        return result

    def _inorder_traversal_rec(self, node, result):
        if node:
            self._inorder_traversal_rec(node.left, result)
            result.append(node.value)
            self._inorder_traversal_rec(node.right, result)

    # Search
    def search(self, value):
        return self._search_rec(self.root, value)

    def _search_rec(self, node, value):
        if not node:
            return False

        if value == node.value:
            return True

        return self._search_rec(node.left, value) or self._search_rec(node.right, value)

    # Height
    def height(self):
        return self._height_rec(self.root)

    def _height_rec(self, node):
        if not node:
            return 0

        return max(self._height_rec(node.left), self._height_rec(node.right)) + 1

    # Deletion
    def delete(self, value):
        self.root = self._delete_rec(self.root, value)

    def _delete_rec(self, node, value):
        if not node:
            return None

        if value == node.value:
            if not node.left:
                return node.right
            elif not node.right:
                return node.left

            min_right = self._find_min(node.right)
            node.value = min_right.value
            node.right = self._delete_rec(node.right, min_right.value)
        elif value < node.value:
            node.left = self._delete_rec(node.left, value)
        else:
            node.right = self._delete_rec(node.right, value)

        return node

    def _find_min(self, node):
        while node.left:
            node = node.left
        return node

    # Mirror
    def mirror(self):
        self._mirror_rec(self.root)

    def _mirror_rec(self, node):
        if node:
            node.left, node.right = node.right, node.left
            self._mirror_rec(node.left)
            self._mirror_rec(node.right)

    # Node Count
    def node_count(self):
        return self._node_count_rec(self.root)

    def _node_count_rec(self, node):
        if not node:
            return 0

        return 1 + self._node_count_rec(node.left) + self._node_count_rec(node.right)

    # Level Order Traversal
    def level_order_traversal(self):
        result = []
        if not self.root:
            return result

        queue = [self.root]
        while queue:
            node = queue.pop(0)
            result.append(node.value)

            if node.left:
                queue.append(node.left)

            if node.right:
                queue.append(node.right)

        return result

    # Check if Balanced
    def is_balanced(self):
        return self._is_balanced_rec(self.root) != -1

    def _is_balanced_rec(self, node):
        if not node:
            return 0

        left_height = self._is_balanced_rec(node.left)
        right_height = self._is_balanced_rec(node.right)

        if left_height == -1 or right_height == -1 or abs(left_height - right_height) > 1:
            return -1

        return max(left_height, right_height) + 1

# Example usage
tree = BinaryTree()
tree.insert(5)
tree.insert(3)
tree.insert(7)
tree.insert(1)
tree.insert(4)
tree.insert(6)
tree.insert(8)

print("In-order traversal:", tree.inorder_traversal())
print("Tree height:", tree.height())
print("Is 4 present in the tree?", tree.search(4))
print("Node count:", tree.node_count())
print("Level order traversal:", tree.level_order_traversal())

tree.mirror()

print("In-order traversal after mirroring:", tree.inorder_traversal())
print("Is the tree balanced?", tree.is_balanced())
