struct Heap<T> {
    private var elements: [T]
    private var isMinHeap: Bool
    private var comparator: (T, T) -> Bool

    init(isMinHeap: Bool = false, comparator: @escaping (T, T) -> Bool) {
        self.elements = []
        self.isMinHeap = isMinHeap
        self.comparator = comparator
    }

    // MARK: - Insertion

    /// Insert an element into the heap.
    mutating func insert(_ element: T) {
        elements.append(element)
        heapifyUp()
    }

    private mutating func heapifyUp() {
        var currentIndex = elements.count - 1
        while currentIndex > 0 {
            let parentIndex = (currentIndex - 1) / 2

            if shouldSwap(parentIndex, currentIndex) {
                elements.swapAt(parentIndex, currentIndex)
                currentIndex = parentIndex
            } else {
                break
            }
        }
    }

    // MARK: - Removal

    /// Remove the root element from the heap.
    mutating func remove() -> T? {
        guard !elements.isEmpty else { return nil }

        if elements.count == 1 {
            return elements.removeLast()
        }

        let root = elements[0]
        elements[0] = elements.removeLast()
        heapifyDown()

        return root
    }

    private mutating func heapifyDown() {
        var currentIndex = 0

        while hasLeftChild(currentIndex) {
            var smallestChildIndex = leftChildIndex(currentIndex)

            if hasRightChild(currentIndex) && shouldSwap(rightChildIndex(currentIndex), smallestChildIndex) {
                smallestChildIndex = rightChildIndex(currentIndex)
            }

            if shouldSwap(smallestChildIndex, currentIndex) {
                elements.swapAt(smallestChildIndex, currentIndex)
                currentIndex = smallestChildIndex
            } else {
                break
            }
        }
    }

    // MARK: - Peek

    /// Peek at the root element without removing it.
    func peek() -> T? {
        return elements.first
    }

    // MARK: - Helper Methods

    private func leftChildIndex(_ parentIndex: Int) -> Int {
        return 2 * parentIndex + 1
    }

    private func rightChildIndex(_ parentIndex: Int) -> Int {
        return 2 * parentIndex + 2
    }

    private func hasLeftChild(_ index: Int) -> Bool {
        return leftChildIndex(index) < elements.count
    }

    private func hasRightChild(_ index: Int) -> Bool {
        return rightChildIndex(index) < elements.count
    }

    private func shouldSwap(_ firstIndex: Int, _ secondIndex: Int) -> Bool {
        if isMinHeap {
            return comparator(elements[secondIndex], elements[firstIndex])
        } else {
            return comparator(elements[firstIndex], elements[secondIndex])
        }
    }

    // MARK: - Build Heap

    /// Builds a heap from an array of elements.
    mutating func buildHeap(_ elements: [T]) {
        self.elements = elements
        for i in stride(from: (elements.count / 2) - 1, through: 0, by: -1) {
            heapifyDown(i)
        }
    }

    // MARK: - Replace Root

    /// Replace the root element with a new element.
    mutating func replaceRoot(with element: T) {
        guard !elements.isEmpty else {
            insert(element)
            return
        }

        elements[0] = element
        heapifyDown()
    }

    // MARK: - Remove at Index

    /// Remove an element at a specific index from the heap.
    mutating func remove(at index: Int) -> T? {
        guard index < elements.count else { return nil }

        if index == elements.count - 1 {
            return elements.removeLast()
        }

        elements.swapAt(index, elements.count - 1)
        let removedElement = elements.removeLast()
        heapifyDown(index)

        return removedElement
    }

    // MARK: - Sort

    /// Sort the heap in-place (ascending order).
    mutating func sort() {
        for i in stride(from: elements.count - 1, through: 1, by: -1) {
            elements.swapAt(0, i)
            heapifyDown(0, endIndex: i)
        }
    }

    // MARK: - Index for Element

    /// Get the index of a specific element, if it exists in the heap.
    func index(of element: T) -> Int? {
        return index(of: element, startingAt: 0)
    }

    private func index(of element: T, startingAt index: Int) -> Int? {
        guard index < elements.count else { return nil }

        if elements[index] == element {
            return index
        }

        if let leftChildIndex = leftChildIndex(index), let result = index(of: element, startingAt: leftChildIndex) {
            return result
        }

        if let rightChildIndex = rightChildIndex(index), let result = index(of: element, startingAt: rightChildIndex) {
            return result
        }

        return nil
    }

    // ... Add more heap operations as needed ...
}
