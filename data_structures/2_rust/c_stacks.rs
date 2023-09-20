struct CustomStack<T> {
    elements: Vec<T>,
}

impl<T> CustomStack<T> {
    // MARK: - Stack Operations

    /// Check if the stack is empty.
    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Get the number of elements in the stack.
    fn count(&self) -> usize {
        self.elements.len()
    }

    /// Push an element onto the stack.
    fn push(&mut self, element: T) {
        self.elements.push(element);
    }

    /// Pop the top element from the stack.
    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    /// Peek at the top element in the stack without removing it.
    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    // MARK: - Stack Operations with Vector

    /// Initialize the stack with a vector of elements.
    fn initialize_with_vector(&mut self, vec: Vec<T>) {
        self.elements = vec;
    }

    /// Push a vector of elements onto the stack.
    fn push_vector(&mut self, vec: Vec<T>) {
        self.elements.extend(vec);
    }

    /// Pop a specified number of elements from the stack.
    fn pop_vector(&mut self, count: usize) -> Vec<T> {
        let mut popped_elements = Vec::new();
        for _ in 0..count {
            if let Some(element) = self.elements.pop() {
                popped_elements.push(element);
            } else {
                break;
            }
        }
        popped_elements
    }

    // MARK: - Clear and Reset

    /// Remove all elements from the stack.
    fn clear(&mut self) {
        self.elements.clear();
    }

    // MARK: - Checking for Element

    /// Check if the stack contains a specific element.
    fn contains(&self, element: &T) -> bool
    where
        T: PartialEq,
    {
        self.elements.iter().any(|e| e == element)
    }

    // MARK: - Filtering

    /// Filter the stack using a given predicate.
    fn filter<F>(&self, predicate: F) -> Vec<T>
    where
        F: Fn(&T) -> bool,
    {
        self.elements.iter().cloned().filter(predicate).collect()
    }

    // MARK: - Conversion to Vector

    /// Convert the stack to a vector.
    fn to_vector(&self) -> Vec<T> {
        self.elements.clone()
    }

    // MARK: - Map

    /// Transform each element in the stack using a provided transform function.
    fn map<U, F>(&self, transform: F) -> Vec<U>
    where
        F: Fn(&T) -> U,
    {
        self.elements.iter().map(transform).collect()
    }

    // MARK: - Reduce

    /// Combine all elements in the stack using a reducer function.
    fn reduce<Result, F>(&self, initial_result: Result, reducer: F) -> Result
    where
        F: Fn(Result, &T) -> Result,
    {
        self.elements.iter().fold(initial_result, |acc, x| reducer(acc, x))
    }

    // MARK: - Concatenate

    /// Concatenate another stack to this stack.
    fn concatenate(&mut self, other_stack: CustomStack<T>) {
        self.elements.extend(other_stack.elements);
    }

    // MARK: - Subscript

    /// Access the element at a specific index in the stack.
    fn get(&self, index: usize) -> Option<&T> {
        self.elements.get(index)
    }

    // ... Add more stack operations as needed ...
}

fn main() {
    let mut stack = CustomStack {
        elements: Vec::new(),
    };
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Stack:");
    for i in (0..stack.count()).rev() {
        if let Some(element) = stack.get(i) {
            println!("{}", element);
        }
    }
}
