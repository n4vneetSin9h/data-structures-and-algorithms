#include <iostream>
#include <algorithm>

template <typename T>
class AVLNode {
public:
    T value;
    int height;
    AVLNode* left;
    AVLNode* right;

    AVLNode(T val) : value(val), height(1), left(nullptr), right(nullptr) {}
};

template <typename T>
class AVLTree {
private:
    AVLNode<T>* root;

    // Get the height of a node
    int height(AVLNode<T>* node) {
        return (node == nullptr) ? 0 : node->height;
    }

    // Update the height of a node based on its children's heights
    void updateHeight(AVLNode<T>* node) {
        node->height = std::max(height(node->left), height(node->right)) + 1;
    }

    // Get the balance factor for a node (difference in height of left and right subtrees)
    int balanceFactor(AVLNode<T>* node) {
        return height(node->left) - height(node->right);
    }

    // Rotate a node to the left
    AVLNode<T>* rotateLeft(AVLNode<T>* y) {
        AVLNode<T>* x = y->right;
        AVLNode<T>* T2 = x->left;

        x->left = y;
        y->right = T2;

        updateHeight(y);
        updateHeight(x);

        return x;
    }

    // Rotate a node to the right
    AVLNode<T>* rotateRight(AVLNode<T>* x) {
        AVLNode<T>* y = x->left;
        AVLNode<T>* T2 = y->right;

        y->right = x;
        x->left = T2;

        updateHeight(x);
        updateHeight(y);

        return y;
    }

    // Balance the tree by rotating nodes if needed
    AVLNode<T>* balance(AVLNode<T>* node) {
        int balanceFactor = this->balanceFactor(node);

        if (balanceFactor > 1) {
            if (balanceFactor(node->left) < 0) {
                node->left = rotateLeft(node->left);
            }
            return rotateRight(node);
        }

        if (balanceFactor < -1) {
            if (balanceFactor(node->right) > 0) {
                node->right = rotateRight(node->right);
            }
            return rotateLeft(node);
        }

        return node;
    }

    // Recursive function to insert a value into the AVL tree
    AVLNode<T>* insert(AVLNode<T>* node, T value) {
        if (node == nullptr) {
            return new AVLNode<T>(value);
        }

        if (value < node->value) {
            node->left = insert(node->left, value);
        } else {
            node->right = insert(node->right, value);
        }

        updateHeight(node);

        return balance(node);
    }

    // Recursive inorder traversal and print the values
    void inorderTraversal(AVLNode<T>* node, void (*visit)(T)) {
        if (node == nullptr) {
            return;
        }

        inorderTraversal(node->left, visit);
        visit(node->value);
        inorderTraversal(node->right, visit);
    }

public:
    AVLTree() : root(nullptr) {}

    // Public function to insert a value into the AVL tree
    void insert(T value) {
        root = insert(root, value);
    }

    // Public function to perform inorder traversal and print the values
    void inorderTraversal(void (*visit)(T)) {
        inorderTraversal(root, visit);
    }
};

// Example usage
int main() {
    AVLTree<int> avlTree;

    // Insert some values into the AVL tree
    avlTree.insert(10);
    avlTree.insert(5);
    avlTree.insert(15);
    avlTree.insert(3);
    avlTree.insert(7);

    // Inorder traversal to print the values
    avlTree.inorderTraversal([](int value) {
        std::cout << value << " ";
    });

    return 0;
}
