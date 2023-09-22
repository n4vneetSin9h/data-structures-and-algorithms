struct CustomStack<T> {
    private var elements: [T] = []

    // MARK: - Stack Operations

    /// Check if the stack is empty.
    func isEmpty() -> Bool {
        return elements.isEmpty
    }

    /// Get the number of elements in the stack.
    func count() -> Int {
        return elements.count
    }

    /// Push an element onto the stack.
    mutating func push(_ element: T) {
        elements.append(element)
    }

    /// Pop the top element from the stack.
    @discardableResult
    mutating func pop() -> T? {
        return elements.popLast()
    }

    /// Peek at the top element in the stack without removing it.
    func peek() -> T? {
        return elements.last
    }

    // MARK: - Stack Operations with Array

    /// Initialize the stack with an array of elements.
    mutating func initialize(withArray array: [T]) {
        elements = array
    }

    /// Push an array of elements onto the stack.
    mutating func pushArray(_ array: [T]) {
        elements += array
    }

    /// Pop an array of elements from the stack.
    mutating func popArray(count: Int) -> [T] {
        var poppedElements: [T] = []
        for _ in 0..<count {
            if let element = elements.popLast() {
                poppedElements.append(element)
            } else {
                break
            }
        }
        return poppedElements
    }

    // MARK: - Clear and Reset

    /// Remove all elements from the stack.
    mutating func clear() {
        elements.removeAll()
    }

    // MARK: - Checking for Element

    /// Check if the stack contains a specific element.
    func contains(_ element: T) -> Bool {
        return elements.contains(element)
    }

    // MARK: - Filtering

    /// Filter the stack using a given closure.
    func filter(_ isIncluded: (T) -> Bool) -> [T] {
        return elements.filter(isIncluded)
    }

    // MARK: - Conversion to Array

    /// Convert the stack to an array.
    func toArray() -> [T] {
        return elements
    }

    // MARK: - Map

    /// Transform each element in the stack using a provided closure.
    func map<U>(_ transform: (T) -> U) -> [U] {
        return elements.map(transform)
    }

    // MARK: - Reduce

    /// Combine all elements in the stack using a closure.
    func reduce<Result>(_ initialResult: Result, _ nextPartialResult: (Result, T) -> Result) -> Result {
        return elements.reduce(initialResult, nextPartialResult)
    }

    // MARK: - Concatenate

    /// Concatenate another stack to this stack.
    mutating func concatenate(_ otherStack: CustomStack<T>) {
        elements += otherStack.elements
    }

    // MARK: - Subscript

    /// Access the element at a specific index in the stack.
    subscript(index: Int) -> T? {
        guard index >= 0, index < elements.count else {
            return nil
        }
        return elements[index]
    }

    // ... Add more stack operations as needed ...
}

