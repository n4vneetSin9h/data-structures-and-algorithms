#include <iostream>
#include <vector>

template <typename T>
class TreeNode {
public:
    T value;
    TreeNode* left;
    TreeNode* right;

    TreeNode(T value) : value(value), left(nullptr), right(nullptr) {}
};

template <typename T>
class BinarySearchTree {
private:
    TreeNode<T>* root;

public:
    BinarySearchTree() : root(nullptr) {}

    // MARK: - Insertion

    /// Insert a value into the binary search tree.
    void insert(T value) {
        root = insertRec(root, value);
    }

    TreeNode<T>* insertRec(TreeNode<T>* node, T value) {
        if (node == nullptr)
            return new TreeNode<T>(value);

        if (value < node->value)
            node->left = insertRec(node->left, value);
        else if (value > node->value)
            node->right = insertRec(node->right, value);

        return node;
    }

    // MARK: - Deletion

    /// Remove a value from the binary search tree.
    void deleteValue(T value) {
        root = deleteRec(root, value);
    }

    TreeNode<T>* deleteRec(TreeNode<T>* node, T value) {
        if (node == nullptr)
            return nullptr;

        if (value < node->value)
            node->left = deleteRec(node->left, value);
        else if (value > node->value)
            node->right = deleteRec(node->right, value);
        else {
            if (node->left == nullptr) {
                TreeNode<T>* temp = node->right;
                delete node;
                return temp;
            } else if (node->right == nullptr) {
                TreeNode<T>* temp = node->left;
                delete node;
                return temp;
            }

            TreeNode<T>* minRight = findMin(node->right);
            node->value = minRight->value;
            node->right = deleteRec(node->right, minRight->value);
        }

        return node;
    }

    // Helper method to find the minimum value node in a subtree
    TreeNode<T>* findMin(TreeNode<T>* node) {
        TreeNode<T>* current = node;
        while (current->left != nullptr) {
            current = current->left;
        }
        return current;
    }

    // MARK: - Search

    /// Search for a value in the binary search tree.
    bool search(T value) {
        return searchRec(root, value);
    }

    bool searchRec(TreeNode<T>* node, T value) {
        if (node == nullptr)
            return false;

        if (value == node->value)
            return true;
        else if (value < node->value)
            return searchRec(node->left, value);
        else
            return searchRec(node->right, value);
    }

    // MARK: - Traversal

    /// Perform in-order traversal of the binary search tree.
    std::vector<T> inorderTraversal() {
        std::vector<T> result;
        inorderTraversalRec(root, result);
        return result;
    }

    void inorderTraversalRec(TreeNode<T>* node, std::vector<T>& result) {
        if (node == nullptr)
            return;

        inorderTraversalRec(node->left, result);
        result.push_back(node->value);
        inorderTraversalRec(node->right, result);
    }

    // ... Add more BST operations as needed ...
};

int main() {
    BinarySearchTree<int> bst;

    bst.insert(5);
    bst.insert(3);
    bst.insert(7);
    bst.insert(1);
    bst.insert(4);

    std::cout << "In-order traversal: ";
    std::vector<int> inorder = bst.inorderTraversal();
    for (int val : inorder) {
        std::cout << val << " ";
    }
    std::cout << std::endl;

    std::cout << "Search 4: " << (bst.search(4) ? "Found" : "Not Found") << std::endl;
    std::cout << "Search 6: " << (bst.search(6) ? "Found" : "Not Found") << std::endl;

    bst.deleteValue(3);

    std::cout << "In-order traversal after deletion: ";
    inorder = bst.inorderTraversal();
    for (int val : inorder) {
        std::cout << val << " ";
    }
    std::cout << std::endl;

    return 0;
}
