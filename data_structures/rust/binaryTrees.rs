use std::cmp::Ord;

#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
struct BinaryTree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T: Ord> BinaryTree<T> {
    /// Insert a value into the binary tree.
    fn insert(&mut self, value: T) {
        self.root = Some(self.insert_rec(self.root.take(), value));
    }

    fn insert_rec(&mut self, node: Option<Box<TreeNode<T>>>, value: T) -> Box<TreeNode<T>> {
        if let Some(mut node) = node {
            if node.left.is_none() {
                node.left = Some(Box::new(TreeNode::new(value)));
            } else if node.right.is_none() {
                node.right = Some(Box::new(TreeNode::new(value)));
            } else {
                if rand::random() {
                    node.left = Some(self.insert_rec(node.left.take(), value));
                } else {
                    node.right = Some(self.insert_rec(node.right.take(), value));
                }
            }
            node
        } else {
            Box::new(TreeNode::new(value))
        }
    }

    /// Perform in-order traversal of the binary tree.
    fn inorder_traversal(&self) -> Vec<&T> {
        let mut result = Vec::new();
        self.inorder_traversal_rec(self.root.as_ref(), &mut result);
        result
    }

    fn inorder_traversal_rec<'a>(&self, node: Option<&'a Box<TreeNode<T>>>, result: &mut Vec<&'a T>) {
        if let Some(node) = node {
            self.inorder_traversal_rec(node.left.as_ref(), result);
            result.push(&node.value);
            self.inorder_traversal_rec(node.right.as_ref(), result);
        }
    }

    /// Search for a value in the binary tree.
    fn search(&self, value: T) -> bool {
        self.search_rec(self.root.as_ref(), value)
    }

    fn search_rec(&self, node: Option<&Box<TreeNode<T>>>, value: T) -> bool {
        if let Some(node) = node {
            if value == node.value {
                return true;
            }
            self.search_rec(node.left.as_ref(), value) || self.search_rec(node.right.as_ref(), value)
        } else {
            false
        }
    }

    /// Calculate the height of the binary tree.
    fn height(&self) -> usize {
        self.height_rec(self.root.as_ref())
    }

    fn height_rec(&self, node: Option<&Box<TreeNode<T>>>) -> usize {
        if let Some(node) = node {
            std::cmp::max(
                self.height_rec(node.left.as_ref()),
                self.height_rec(node.right.as_ref()),
            ) + 1
        } else {
            0
        }
    }

    /// Delete a value from the binary tree.
    fn delete(&mut self, value: T) {
        self.root = self.delete_rec(self.root.take(), value);
    }

    fn delete_rec(&mut self, node: Option<Box<TreeNode<T>>>, value: T) -> Option<Box<TreeNode<T>>> {
        if let Some(mut node) = node {
            if value == node.value {
                // Node to be deleted found
                // If the node has only one child or no child
                if node.left.is_none() {
                    return node.right.take();
                } else if node.right.is_none() {
                    return node.left.take();
                }

                // Node has two children
                let min_value_node = self.find_min(node.right.as_mut().unwrap());
                node.value = min_value_node.value;
                node.right = self.delete_rec(node.right.take(), min_value_node.value);
            } else if value < node.value {
                node.left = self.delete_rec(node.left.take(), value);
            } else {
                node.right = self.delete_rec(node.right.take(), value);
            }

            Some(node)
        } else {
            None
        }
    }

    fn find_min(&mut self, mut node: &mut Box<TreeNode<T>>) -> &mut Box<TreeNode<T>> {
        while node.left.is_some() {
            node = &mut node.left.as_mut().unwrap();
        }
        node
    }

    /// Create a mirror image of the binary tree.
    fn mirror(&mut self) {
        self.mirror_rec(self.root.as_mut());
    }

    fn mirror_rec(&mut self, node: Option<&mut Box<TreeNode<T>>>) {
        if let Some(node) = node {
            std::mem::swap(&mut node.left, &mut node.right);
            self.mirror_rec(node.left.as_mut());
            self.mirror_rec(node.right.as_mut());
        }
    }

    /// Count the number of nodes in the binary tree.
    fn node_count(&self) -> usize {
        self.node_count_rec(self.root.as_ref())
    }

    fn node_count_rec(&self, node: Option<&Box<TreeNode<T>>>) -> usize {
        if let Some(node) = node {
            1 + self.node_count_rec(node.left.as_ref()) + self.node_count_rec(node.right.as_ref())
        } else {
            0
        }
    }

    /// Perform level-order traversal of the binary tree.
    fn level_order_traversal(&self) -> Vec<&T> {
        let mut result = Vec::new();
        if let Some(root_node) = self.root.as_ref() {
            let mut queue = vec![root_node];

            while let Some(node) = queue.pop() {
                result.push(&node.value);

                if let Some(left) = node.left.as_ref() {
                    queue.push(left);
                }

                if let Some(right) = node.right.as_ref() {
                    queue.push(right);
                }
            }
        }
        result
    }

    /// Check if the binary tree is balanced.
    fn is_balanced(&self) -> bool {
        self.is_balanced_rec(self.root.as_ref()) != -1
    }

    fn is_balanced_rec(&self, node: Option<&Box<TreeNode<T>>>) -> i32 {
        if let Some(node) = node {
            let left_height = self.is_balanced_rec(node.left.as_ref());
            let right_height = self.is_balanced_rec(node.right.as_ref());

            if left_height == -1 || right_height == -1 || (left_height - right_height).abs() > 1 {
                return -1;
            }

            std::cmp::max(left_height, right_height) + 1
        } else {
            0
        }
    }
}

fn main() {
    // Example usage
    let mut tree = BinaryTree { root: None };
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(4);
    tree.insert(6);
    tree.insert(8);

    println!("In-order traversal: {:?}", tree.inorder_traversal());
    println!("Tree height: {}", tree.height());
    println!("Is 4 present in the tree? {}", tree.search(4));
    println!("Node count: {}", tree.node_count());
    println!("Level order traversal: {:?}", tree.level_order_traversal());

    tree.mirror();

    println!("In-order traversal after mirroring: {:?}", tree.inorder_traversal());
    println!("Is the tree balanced? {}", tree.is_balanced());
}
