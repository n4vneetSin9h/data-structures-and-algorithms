#include <iostream>
#include <vector>
#include <queue>
#include <cstdlib>

template <typename T>
class TreeNode {
public:
    T value;
    TreeNode<T>* left;
    TreeNode<T>* right;

    TreeNode(T val) : value(val), left(nullptr), right(nullptr) {}
};

template <typename T>
class BinaryTree {
private:
    TreeNode<T>* root;

    // Insertion
    TreeNode<T>* insertRec(TreeNode<T>* node, T value) {
        if (node) {
            if (node->left == nullptr) {
                node->left = insertRec(nullptr, value);
            } else if (node->right == nullptr) {
                node->right = insertRec(nullptr, value);
            } else {
                if (std::rand() % 2 == 0) {
                    node->left = insertRec(node->left, value);
                } else {
                    node->right = insertRec(node->right, value);
                }
            }
            return node;
        } else {
            return new TreeNode<T>(value);
        }
    }

    // In-order Traversal
    void inorderTraversalRec(TreeNode<T>* node, std::vector<T>& result) {
        if (node) {
            inorderTraversalRec(node->left, result);
            result.push_back(node->value);
            inorderTraversalRec(node->right, result);
        }
    }

    // Search
    bool searchRec(TreeNode<T>* node, T value) {
        if (!node) {
            return false;
        }

        if (value == node->value) {
            return true;
        }

        return searchRec(node->left, value) || searchRec(node->right, value);
    }

    // Height
    int heightRec(TreeNode<T>* node) {
        if (!node) {
            return 0;
        }

        return std::max(heightRec(node->left), heightRec(node->right)) + 1;
    }

    // Deletion
    TreeNode<T>* deleteRec(TreeNode<T>* node, T value) {
        if (!node) {
            return nullptr;
        }

        if (value == node->value) {
            if (node->left == nullptr) {
                return node->right;
            } else if (node->right == nullptr) {
                return node->left;
            }

            TreeNode<T>* minValueNode = findMin(node->right);
            node->value = minValueNode->value;
            node->right = deleteRec(node->right, minValueNode->value);
        } else if (value < node->value) {
            node->left = deleteRec(node->left, value);
        } else {
            node->right = deleteRec(node->right, value);
        }

        return node;
    }

    TreeNode<T>* findMin(TreeNode<T>* node) {
        while (node->left) {
            node = node->left;
        }
        return node;
    }

    // Mirror
    void mirrorRec(TreeNode<T>* node) {
        if (node) {
            TreeNode<T>* temp = node->left;
            node->left = node->right;
            node->right = temp;

            mirrorRec(node->left);
            mirrorRec(node->right);
        }
    }

    // Node Count
    int nodeCountRec(TreeNode<T>* node) {
        if (!node) {
            return 0;
        }

        return 1 + nodeCountRec(node->left) + nodeCountRec(node->right);
    }

    // Level Order Traversal
    void levelOrderTraversalRec(TreeNode<T>* node, std::vector<T>& result) {
        if (!node) {
            return;
        }

        std::queue<TreeNode<T>*> queue;
        queue.push(node);

        while (!queue.empty()) {
            TreeNode<T>* currNode = queue.front();
            queue.pop();
            result.push_back(currNode->value);

            if (currNode->left) {
                queue.push(currNode->left);
            }

            if (currNode->right) {
                queue.push(currNode->right);
            }
        }
    }

    // Check if Balanced
    int isBalancedRec(TreeNode<T>* node) {
        if (!node) {
            return 0;
        }

        int leftHeight = isBalancedRec(node->left);
        int rightHeight = isBalancedRec(node->right);

        if (leftHeight == -1 || rightHeight == -1 || abs(leftHeight - rightHeight) > 1) {
            return -1;
        }

        return std::max(leftHeight, rightHeight) + 1;
    }

public:
    BinaryTree() : root(nullptr) {}

    // Insertion
    void insert(T value) {
        root = insertRec(root, value);
    }

    // In-order Traversal
    std::vector<T> inorderTraversal() {
        std::vector<T> result;
        inorderTraversalRec(root, result);
        return result;
    }

    // Search
    bool search(T value) {
        return searchRec(root, value);
    }

    // Height
    int height() {
        return heightRec(root);
    }

    // Deletion
    void deleteNode(T value) {
        root = deleteRec(root, value);
    }

    // Mirror
    void mirror() {
        mirrorRec(root);
    }

    // Node Count
    int nodeCount() {
        return nodeCountRec(root);
    }

    // Level Order Traversal
    std::vector<T> levelOrderTraversal() {
        std::vector<T> result;
        levelOrderTraversalRec(root, result);
        return result;
    }

    // Check if Balanced
    bool isBalanced() {
        return isBalancedRec(root) != -1;
    }
};

int main() {
    // Example usage
    BinaryTree<int> tree;
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(4);
    tree.insert(6);
    tree.insert(8);

    std::cout << "In-order traversal: ";
    for (int val : tree.inorderTraversal()) {
        std::cout << val << " ";
    }
    std::cout << "\n";

    std::cout << "Tree height: " << tree.height() << "\n";

    std::cout << "Is 4 present in the tree? " << (tree.search(4) ? "Yes" : "No") << "\n";

    std::cout << "Node count: " << tree.nodeCount() << "\n";

    std::cout << "Level order traversal: ";
    for (int val : tree.levelOrderTraversal()) {
        std::cout << val << " ";
    }
    std::cout << "\n";

    tree.mirror();

    std::cout << "In-order traversal after mirroring: ";
    for (int val : tree.inorderTraversal()) {
        std::cout << val << " ";
    }
    std::cout << "\n";

    std::cout << "Is the tree balanced? " << (tree.isBalanced() ? "Yes" : "No") << "\n";

    return 0;
}
