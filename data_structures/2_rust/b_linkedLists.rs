struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node { value, next: None }
    }
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList {
            head: None,
            tail: std::ptr::null_mut(),
        }
    }

    // Check if the linked list is empty.
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // Get the first node in the linked list.
    fn first(&self) -> Option<&Node<T>> {
        self.head.as_ref().map(|node| &**node)
    }

    // Get the last node in the linked list.
    fn last(&self) -> Option<&Node<T>> {
        if !self.tail.is_null() {
            unsafe { Some(&*self.tail) }
        } else {
            None
        }
    }

    // Append a value to the end of the linked list.
    fn append(&mut self, value: T) {
        let new_node = Box::new(Node::new(value));

        let raw_node = Box::into_raw(new_node);
        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(Box::from_raw(raw_node));
            }
        } else {
            self.head = Some(Box::from_raw(raw_node));
        }

        self.tail = raw_node;
    }

    // Prepend a value to the beginning of the linked list.
    fn prepend(&mut self, value: T) {
        let new_node = Box::new(Node::new(value));
        let raw_node = Box::into_raw(new_node);

        if !self.head.is_null() {
            unsafe {
                (*raw_node).next = self.head.take();
            }
        } else {
            self.tail = raw_node;
        }

        self.head = Some(unsafe { Box::from_raw(raw_node) });
    }

    // Check if a value exists in the linked list.
    fn contains(&self, value: T) -> bool {
        let mut current = &self.head;

        while let Some(node) = current {
            if node.value == value {
                return true;
            }
            current = &node.next;
        }

        false
    }

    // Get the node at a specific index in the linked list.
    fn node_at_index(&self, index: usize) -> Option<&Node<T>> {
        let mut current = &self.head;
        let mut current_index = 0;

        while let Some(node) = current {
            if current_index == index {
                return Some(node);
            }

            current_index += 1;
            current = &node.next;
        }

        None
    }

    // Insert a value at a specific index in the linked list.
    fn insert(&mut self, value: T, index: usize) {
        if index == 0 {
            self.prepend(value);
            return;
        }

        let prev_node = self.node_at_index(index - 1);

        if let Some(prev) = prev_node {
            let new_node = Box::new(Node::new(value));

            if prev.next.is_some() {
                let mut next_node = prev.next.take().unwrap();
                new_node.next = Some(next_node);
                prev.next = Some(new_node);
            } else {
                prev.next = Some(new_node);
                self.tail = &mut *prev.next.as_mut().unwrap();
            }
        }
    }

    // Remove the node at a specific index in the linked list.
    fn remove(&mut self, index: usize) {
        if index == 0 {
            if let Some(mut head_node) = self.head.take() {
                if head_node.next.is_some() {
                    let new_head = head_node.next.take().unwrap();
                    self.head = Some(new_head);
                } else {
                    self.tail = std::ptr::null_mut();
                }
            }
            return;
        }

        let prev_node = self.node_at_index(index - 1);

        if let Some(prev) = prev_node {
            if prev.next.is_some() {
                let mut next_node = prev.next.take().unwrap();
                if next_node.next.is_some() {
                    prev.next = next_node.next.take();
                } else {
                    prev.next = None;
                    self.tail = &mut *prev;
                }
            }
        }
    }

    // Print the linked list.
    fn print_list(&self) {
        let mut current = &self.head;

        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = &node.next;
        }

        println!("None");
    }

    // Reverse the linked list.
    fn reverse(&mut self) {
        let mut prev_node: Option<Box<Node<T>>> = None;
        let mut current_node = self.head.take();

        while let Some(mut current) = current_node {
            let next_node = current.next.take();
            current.next = prev_node.take();
            prev_node = Some(current);

            current_node = next_node;
        }

        self.head = prev_node.take();
        self.tail = std::ptr::null_mut();

        let mut current = &mut self.head;

        while let Some(node) = current {
            if node.next.is_none() {
                self.tail = &mut **node;
            }
            current = &mut node.next;
        }
    }

    // Return the number of nodes in the linked list.
    fn count(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;

        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }

        count
    }

    // Remove all nodes from the linked list.
    fn remove_all(&mut self) {
        let mut current = self.head.take();

        while let Some(mut node) = current {
            current = node.next.take();
        }

        self.tail = std::ptr::null_mut();
    }

    // Return a vector representation of the linked list.
    fn to_vec(&self) -> Vec<T> {
        let mut vec = Vec::new();
        let mut current = &self.head;

        while let Some(node) = current {
            vec.push(node.value.clone());
            current = &node.next;
        }

        vec
    }

    // Remove all occurrences of a value from the linked list.
    fn remove_all_occurrences(&mut self, value: T) {
        let mut prev_node: Option<&mut Box<Node<T>>> = None;
        let mut current_node = &mut self.head;

        while let Some(node) = current_node {
            if node.value == value {
                let next_node = &mut node.next;
                if next_node.is_some() {
                    let new_next = next_node.take().unwrap().next.take();
                    *next_node = new_next;
                } else {
                    self.tail = std::ptr::null_mut();
                }
            } else {
                prev_node = Some(&mut *node);
            }

            current_node = &mut node.next;
        }
    }
}

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.append(1);
    list.append(2);
    list.append(3);

    println!("Linked List:");
    list.print_list();
}
