#[derive(Debug)]
struct TreeNode<T: PartialOrd> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T: PartialOrd> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
struct BinarySearchTree<T: PartialOrd> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: PartialOrd> BinarySearchTree<T> {
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // MARK: - Insertion

    /// Insert a value into the binary search tree.
    fn insert(&mut self, value: T) {
        let new_node = TreeNode::new(value);
        self.root = self.insert_rec(self.root.take(), new_node);
    }

    fn insert_rec(
        &mut self,
        node: Option<Box<TreeNode<T>>>,
        new_node: TreeNode<T>,
    ) -> Option<Box<TreeNode<T>>> {
        match node {
            None => Some(Box::new(new_node)),
            Some(mut node) => {
                if new_node.value < node.value {
                    node.left = self.insert_rec(node.left.take(), new_node);
                } else if new_node.value > node.value {
                    node.right = self.insert_rec(node.right.take(), new_node);
                }
                Some(node)
            }
        }
    }

    // MARK: - Deletion

    /// Remove a value from the binary search tree.
    fn delete(&mut self, value: T) {
        self.root = self.delete_rec(self.root.take(), value);
    }

    fn delete_rec(
        &mut self,
        node: Option<Box<TreeNode<T>>>,
        value: T,
    ) -> Option<Box<TreeNode<T>>> {
        match node {
            None => None,
            Some(mut node) => {
                if value < node.value {
                    node.left = self.delete_rec(node.left.take(), value);
                    Some(node)
                } else if value > node.value {
                    node.right = self.delete_rec(node.right.take(), value);
                    Some(node)
                } else {
                    match (node.left.take(), node.right.take()) {
                        (None, right) => right,
                        (left, None) => left,
                        (left, right) => {
                            let min_right = self.find_min(right.as_deref());
                            node.value = min_right.value;
                            node.right = self.delete_rec(right, min_right.value);
                            Some(node)
                        }
                    }
                }
            }
        }
    }

    // Helper method to find the minimum value node in a subtree
    fn find_min(&self, node: Option<&TreeNode<T>>) -> &TreeNode<T> {
        match node {
            Some(n) => {
                if let Some(left) = &n.left {
                    self.find_min(Some(&*left))
                } else {
                    n
                }
            }
            None => unreachable!(),
        }
    }

    // MARK: - Search

    /// Search for a value in the binary search tree.
    fn search(&self, value: T) -> bool {
        self.search_rec(self.root.as_deref(), value)
    }

    fn search_rec(&self, node: Option<&TreeNode<T>>, value: T) -> bool {
        match node {
            Some(n) => {
                if value == n.value {
                    true
                } else if value < n.value {
                    self.search_rec(n.left.as_deref(), value)
                } else {
                    self.search_rec(n.right.as_deref(), value)
                }
            }
            None => false,
        }
    }

    // MARK: - Traversal

    /// Perform in-order traversal of the binary search tree.
    fn inorder_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        self.inorder_traversal_rec(self.root.as_deref(), &mut result);
        result
    }

    fn inorder_traversal_rec(&self, node: Option<&TreeNode<T>>, result: &mut Vec<T>) {
        match node {
            Some(n) => {
                self.inorder_traversal_rec(n.left.as_deref(), result);
                result.push(n.value);
                self.inorder_traversal_rec(n.right.as_deref(), result);
            }
            None => {}
        }
    }

    // ... Add more BST operations as needed ...
}

fn main() {
    let mut bst = BinarySearchTree::new();

    bst.insert(5);
    bst.insert(3);
    bst.insert(7);
    bst.insert(1);
    bst.insert(4);

    println!("In-order traversal: {:?}", bst.inorder_traversal());

    println!("Search 4: {}", bst.search(4));
    println!("Search 6: {}", bst.search(6));

    bst.delete(3);

    println!("In-order traversal after deletion: {:?}", bst.inorder_traversal());
}
