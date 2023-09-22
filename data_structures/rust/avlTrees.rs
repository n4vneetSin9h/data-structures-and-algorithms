use std::cmp::Ord;
use std::cmp::Ordering;

#[derive(Debug)]
struct AVLNode<T: Ord> {
    value: T,
    height: i32,
    left: Option<Box<AVLNode<T>>>,
    right: Option<Box<AVLNode<T>>>,
}

impl<T: Ord> AVLNode<T> {
    fn new(value: T) -> Self {
        AVLNode {
            value,
            height: 1,
            left: None,
            right: None,
        }
    }

    fn update_height(&mut self) {
        let left_height = self.left.as_ref().map_or(0, |node| node.height);
        let right_height = self.right.as_ref().map_or(0, |node| node.height);
        self.height = 1 + i32::max(left_height, right_height);
    }

    fn balance_factor(&self) -> i32 {
        let left_height = self.left.as_ref().map_or(0, |node| node.height);
        let right_height = self.right.as_ref().map_or(0, |node| node.height);
        left_height - right_height
    }

    fn rotate_left(mut y: Box<AVLNode<T>>) -> Box<AVLNode<T>> {
        let mut x = y.right.take().unwrap();
        let t2 = x.left.take();

        x.left = Some(y);
        y.right = t2;

        y.update_height();
        x.update_height();

        x
    }

    fn rotate_right(mut x: Box<AVLNode<T>>) -> Box<AVLNode<T>> {
        let mut y = x.left.take().unwrap();
        let t2 = y.right.take();

        y.right = Some(x);
        x.left = t2;

        x.update_height();
        y.update_height();

        y
    }

    fn balance(mut node: Box<AVLNode<T>>) -> Box<AVLNode<T>> {
        let balance_factor = node.balance_factor();

        if balance_factor > 1 {
            if let Some(mut left) = node.left {
                if left.balance_factor() < 0 {
                    node.left = Some(Self::rotate_left(left));
                }
            }
            Self::rotate_right(node)
        } else if balance_factor < -1 {
            if let Some(mut right) = node.right {
                if right.balance_factor() > 0 {
                    node.right = Some(Self::rotate_right(right));
                }
            }
            Self::rotate_left(node)
        } else {
            node
        }
    }

    fn insert(mut node: Box<AVLNode<T>>, value: T) -> Box<AVLNode<T>> {
        match value.cmp(&node.value) {
            Ordering::Less => {
                if let Some(left) = node.left {
                    node.left = Some(Self::insert(left, value));
                } else {
                    node.left = Some(Box::new(Self::new(value)));
                }
            }
            Ordering::Greater => {
                if let Some(right) = node.right {
                    node.right = Some(Self::insert(right, value));
                } else {
                    node.right = Some(Box::new(Self::new(value)));
                }
            }
            Ordering::Equal => (), // Duplicate values are not allowed
        }

        node.update_height();
        Box::new(Self::balance(node))
    }
}

#[derive(Debug)]
struct AVLTree<T: Ord> {
    root: Option<Box<AVLNode<T>>>,
}

impl<T: Ord> AVLTree<T> {
    fn new() -> Self {
        AVLTree { root: None }
    }

    fn insert(&mut self, value: T) {
        if let Some(root) = self.root.take() {
            self.root = Some(AVLNode::insert(root, value));
        } else {
            self.root = Some(Box::new(AVLNode::new(value)));
        }
    }

    fn inorder_traversal<F>(&self, visit: F)
    where
        F: Fn(&T),
    {
        Self::inorder_traversal_internal(&self.root, &visit);
    }

    fn inorder_traversal_internal<F>(node: &Option<Box<AVLNode<T>>>, visit: &F)
    where
        F: Fn(&T),
    {
        if let Some(node) = node {
            Self::inorder_traversal_internal(&node.left, visit);
            visit(&node.value);
            Self::inorder_traversal_internal(&node.right, visit);
        }
    }
}

// Example usage
fn main() {
    let mut avl_tree = AVLTree::new();

    // Insert some values into the AVL tree
    avl_tree.insert(10);
    avl_tree.insert(5);
    avl_tree.insert(15);
    avl_tree.insert(3);
    avl_tree.insert(7);

    // Inorder traversal to print the values
    avl_tree.inorder_traversal(|value| println!("{}", value));
}
