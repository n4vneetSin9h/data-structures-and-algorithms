use std::cmp::Ord;
use std::cmp::Ordering;
use std::ops::Index;

struct Heap<T> {
    elements: Vec<T>,
    is_min_heap: bool,
    comparator: Box<dyn Fn(&T, &T) -> Ordering>,
}

impl<T: Ord> Heap<T> {
    fn new(is_min_heap: bool) -> Self {
        let comparator: Box<dyn Fn(&T, &T) -> Ordering> = if is_min_heap {
            Box::new(|a, b| a.cmp(b).reverse())
        } else {
            Box::new(|a, b| a.cmp(b))
        };

        Heap {
            elements: Vec::new(),
            is_min_heap,
            comparator,
        }
    }

    // MARK: - Insertion

    fn insert(&mut self, element: T) {
        self.elements.push(element);
        self.heapify_up();
    }

    fn heapify_up(&mut self) {
        let mut current_index = self.elements.len() - 1;
        while current_index > 0 {
            let parent_index = (current_index - 1) / 2;

            if self.should_swap(parent_index, current_index) {
                self.elements.swap(parent_index, current_index);
                current_index = parent_index;
            } else {
                break;
            }
        }
    }

    // MARK: - Removal

    fn remove(&mut self) -> Option<T> {
        if self.elements.is_empty() {
            return None;
        }

        if self.elements.len() == 1 {
            return Some(self.elements.remove(0));
        }

        let root = self.elements.swap_remove(0);
        self.heapify_down();

        Some(root)
    }

    fn heapify_down(&mut self) {
        let mut current_index = 0;

        while self.has_left_child(current_index) {
            let mut smallest_child_index = self.left_child_index(current_index);

            if self.has_right_child(current_index) {
                if self.should_swap(
                    self.right_child_index(current_index),
                    smallest_child_index,
                ) {
                    smallest_child_index = self.right_child_index(current_index);
                }
            }

            if self.should_swap(smallest_child_index, current_index) {
                self.elements.swap(smallest_child_index, current_index);
                current_index = smallest_child_index;
            } else {
                break;
            }
        }
    }

    // MARK: - Peek

    fn peek(&self) -> Option<&T> {
        self.elements.get(0)
    }

    // MARK: - Helper Methods

    fn left_child_index(&self, parent_index: usize) -> usize {
        2 * parent_index + 1
    }

    fn right_child_index(&self, parent_index: usize) -> usize {
        2 * parent_index + 2
    }

    fn has_left_child(&self, index: usize) -> bool {
        self.left_child_index(index) < self.elements.len()
    }

    fn has_right_child(&self, index: usize) -> bool {
        self.right_child_index(index) < self.elements.len()
    }

    fn should_swap(&self, first_index: usize, second_index: usize) -> bool {
        match (self.comparator)(&self.elements[first_index], &self.elements[second_index]) {
            Ordering::Greater => true,
            Ordering::Equal => true,
            Ordering::Less => false,
        }
    }

    // MARK: - Build Heap

    fn build_heap(&mut self, elements: Vec<T>) {
        self.elements = elements;
        for i in (0..self.elements.len() / 2).rev() {
            self.heapify_down(i);
        }
    }

    // MARK: - Replace Root

    fn replace_root(&mut self, element: T) {
        if self.elements.is_empty() {
            self.insert(element);
            return;
        }

        self.elements[0] = element;
        self.heapify_down(0);
    }

    // MARK: - Remove at Index

    fn remove_at_index(&mut self, index: usize) -> Option<T> {
        if index >= self.elements.len() {
            return None;
        }

        if index == self.elements.len() - 1 {
            return Some(self.elements.remove(index));
        }

        self.elements.swap(index, self.elements.len() - 1);
        let removed = self.elements.pop();

        if let Some(removed) = removed {
            self.heapify_down(index);
        }

        Some(removed)
    }

    // MARK: - Sort

    fn sort(&mut self) {
        let len = self.elements.len();
        for i in (0..len).rev() {
            self.elements.swap(0, i);
            self.heapify_down(0);
        }
    }

    // MARK: - Index for Element

    fn index(&self, element: &T) -> Option<usize> {
        self.index_from(element, 0)
    }

    fn index_from(&self, element: &T, start_index: usize) -> Option<usize> {
        if start_index >= self.elements.len() {
            return None;
        }

        if &self.elements[start_index] == element {
            return Some(start_index);
        }

        let left_index = self.left_child_index(start_index);
        let right_index = self.right_child_index(start_index);

        if left_index < self.elements.len() {
            if let Some(result) = self.index_from(element, left_index) {
                return Some(result);
            }
        }

        if right_index < self.elements.len() {
            return self.index_from(element, right_index);
        }

        None
    }

    // ... Add more heap operations as needed ...
}

fn main() {
    let mut max_heap = Heap::new(false);
    max_heap.insert(3);
    max_heap.insert(1);
    max_heap.insert(4);
    max_heap.insert(1);
    max_heap.insert(5);
    max_heap.insert(9);

    println!("Max Heap: {:?}", max_heap.elements);

    let mut min_heap = Heap::new(true);
    min_heap.insert(3);
    min_heap.insert(1);
    min_heap.insert(4);
    min_heap.insert(1);
    min_heap.insert(5);
    min_heap.insert(9);

    println!("Min Heap: {:?}", min_heap.elements);
}
