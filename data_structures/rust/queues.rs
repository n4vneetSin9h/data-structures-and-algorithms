pub struct CustomQueue<T> {
    elements: Vec<T>,
}

impl<T> CustomQueue<T> {
    // MARK: - Queue Operations

    /// Enqueue an element to the back of the queue.
    pub fn enqueue(&mut self, element: T) {
        self.elements.push(element);
    }

    /// Dequeue an element from the front of the queue.
    pub fn dequeue(&mut self) -> Option<T> {
        self.elements.pop()
    }

    /// Peek at the front element in the queue without dequeuing.
    pub fn peek(&self) -> Option<&T> {
        self.elements.first()
    }

    /// Check if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Get the number of elements in the queue.
    pub fn count(&self) -> usize {
        self.elements.len()
    }

    // MARK: - Queue Operations with Vector

    /// Initialize the queue with a vector of elements.
    pub fn initialize_with_vector(&mut self, vec: Vec<T>) {
        self.elements = vec;
    }

    /// Enqueue a vector of elements to the back of the queue.
    pub fn enqueue_vector(&mut self, vec: Vec<T>) {
        self.elements.extend(vec);
    }

    /// Dequeue a specified number of elements from the front of the queue.
    pub fn dequeue_vector(&mut self, count: usize) -> Vec<T> {
        if count == 0 {
            return Vec::new();
        }
        let dequeued_elements: Vec<T> = self.elements.drain(0..count).collect();
        dequeued_elements
    }

    // MARK: - Clear and Reset

    /// Remove all elements from the queue.
    pub fn clear(&mut self) {
        self.elements.clear();
    }

    // MARK: - Checking for Element

    /// Check if the queue contains a specific element.
    pub fn contains(&self, element: &T) -> bool
    where
        T: PartialEq,
    {
        self.elements.contains(element)
    }

    // MARK: - Filtering

    /// Filter the queue using a given predicate.
    pub fn filter<F>(&self, predicate: F) -> Vec<T>
    where
        F: Fn(&T) -> bool,
    {
        self.elements.iter().filter(|&x| predicate(x)).cloned().collect()
    }

    // MARK: - Conversion to Vector

    /// Convert the queue to a vector.
    pub fn to_vector(&self) -> Vec<T> {
        self.elements.clone()
    }

    // MARK: - Map

    /// Transform each element in the queue using a provided transform function.
    pub fn map<U, F>(&self, transform: F) -> Vec<U>
    where
        F: Fn(&T) -> U,
    {
        self.elements.iter().map(transform).collect()
    }

    // MARK: - Reduce

    /// Combine all elements in the queue using a reducer function.
    pub fn reduce<Result, F>(&self, initial_result: Result, reducer: F) -> Result
    where
        F: Fn(Result, &T) -> Result,
    {
        self.elements.iter().fold(initial_result, |acc, x| reducer(acc, x))
    }

    // MARK: - Concatenate

    /// Concatenate another queue to this queue.
    pub fn concatenate(&mut self, other_queue: CustomQueue<T>) {
        self.elements.extend(other_queue.elements);
    }

    // MARK: - Subscript

    /// Access the element at a specific index in the queue.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.elements.get(index)
    }

    // ... Add more queue operations as needed ...
}

fn main() {
    let mut queue = CustomQueue {
        elements: Vec::new(),
    };
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    println!("Queue:");
    while !queue.is_empty() {
        if let Some(element) = queue.dequeue() {
            println!("{}", element);
        }
    }
}
