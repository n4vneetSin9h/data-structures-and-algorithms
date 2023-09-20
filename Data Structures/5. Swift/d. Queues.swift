struct CustomQueue<T> {
    private var elements: [T] = []

    // MARK: - Queue Operations

    /// Enqueue an element to the back of the queue.
    mutating func enqueue(_ element: T) {
        elements.append(element)
    }

    /// Dequeue an element from the front of the queue.
    @discardableResult
    mutating func dequeue() -> T? {
        return elements.isEmpty ? nil : elements.removeFirst()
    }

    /// Peek at the front element in the queue without dequeuing.
    func peek() -> T? {
        return elements.first
    }

    /// Check if the queue is empty.
    func isEmpty() -> Bool {
        return elements.isEmpty
    }

    /// Get the number of elements in the queue.
    func count() -> Int {
        return elements.count
    }

    // MARK: - Queue Operations with Array

    /// Initialize the queue with an array of elements.
    mutating func initialize(withArray array: [T]) {
        elements = array
    }

    /// Enqueue an array of elements to the back of the queue.
    mutating func enqueueArray(_ array: [T]) {
        elements.append(contentsOf: array)
    }

    /// Dequeue an array of elements from the front of the queue.
    mutating func dequeueArray(count: Int) -> [T] {
        var dequeuedElements: [T] = []
        for _ in 0..<count {
            if let element = elements.first {
                dequeuedElements.append(element)
                elements.removeFirst()
            } else {
                break
            }
        }
        return dequeuedElements
    }

    // MARK: - Clear and Reset

    /// Remove all elements from the queue.
    mutating func clear() {
        elements.removeAll()
    }

    // MARK: - Checking for Element

    /// Check if the queue contains a specific element.
    func contains(_ element: T) -> Bool {
        return elements.contains(element)
    }

    // MARK: - Filtering

    /// Filter the queue using a given closure.
    func filter(_ isIncluded: (T) -> Bool) -> [T] {
        return elements.filter(isIncluded)
    }

    // MARK: - Conversion to Array

    /// Convert the queue to an array.
    func toArray() -> [T] {
        return elements
    }

    // MARK: - Map

    /// Transform each element in the queue using a provided closure.
    func map<U>(_ transform: (T) -> U) -> [U] {
        return elements.map(transform)
    }

    // MARK: - Reduce

    /// Combine all elements in the queue using a closure.
    func reduce<Result>(_ initialResult: Result, _ nextPartialResult: (Result, T) -> Result) -> Result {
        return elements.reduce(initialResult, nextPartialResult)
    }

    // MARK: - Concatenate

    /// Concatenate another queue to this queue.
    mutating func concatenate(_ otherQueue: CustomQueue<T>) {
        elements.append(contentsOf: otherQueue.elements)
    }

    // MARK: - Subscript

    /// Access the element at a specific index in the queue.
    subscript(index: Int) -> T? {
        guard index >= 0, index < elements.count else {
            return nil
        }
        return elements[index]
    }

    // ... Add more queue operations as needed ...
}
