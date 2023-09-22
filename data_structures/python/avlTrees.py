class AVLNode:
    def __init__(self, value):
        self.value = value
        self.height = 1
        self.left = None
        self.right = None


class AVLTree:
    def __init__(self):
        self.root = None

    def height(self, node):
        return node.height if node else 0

    def update_height(self, node):
        node.height = max(self.height(node.left), self.height(node.right)) + 1

    def balance_factor(self, node):
        return self.height(node.left) - self.height(node.right)

    def rotate_left(self, y):
        x = y.right
        T2 = x.left

        x.left = y
        y.right = T2

        self.update_height(y)
        self.update_height(x)

        return x

    def rotate_right(self, x):
        y = x.left
        T2 = y.right

        y.right = x
        x.left = T2

        self.update_height(x)
        self.update_height(y)

        return y

    def balance(self, node):
        balance_factor = self.balance_factor(node)

        if balance_factor > 1:
            if self.balance_factor(node.left) < 0:
                node.left = self.rotate_left(node.left)
            return self.rotate_right(node)

        if balance_factor < -1:
            if self.balance_factor(node.right) > 0:
                node.right = self.rotate_right(node.right)
            return self.rotate_left(node)

        return node

    def insert(self, node, value):
        if not node:
            return AVLNode(value)

        if value < node.value:
            node.left = self.insert(node.left, value)
        else:
            node.right = self.insert(node.right, value)

        self.update_height(node)

        return self.balance(node)

    def inorder_traversal(self, node, visit):
        if not node:
            return

        self.inorder_traversal(node.left, visit)
        visit(node.value)
        self.inorder_traversal(node.right, visit)

    def insert_value(self, value):
        self.root = self.insert(self.root, value)

    def inorder_traversal_values(self, visit):
        self.inorder_traversal(self.root, visit)


# Example usage
if __name__ == "__main__":
    avl_tree = AVLTree()

    # Insert some values into the AVL tree
    avl_tree.insert_value(10)
    avl_tree.insert_value(5)
    avl_tree.insert_value(15)
    avl_tree.insert_value(3)
    avl_tree.insert_value(7)

    # Inorder traversal to print the values
    avl_tree.inorder_traversal_values(lambda value: print(value))