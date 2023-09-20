struct CustomArray<T> {
    private var elements: [T]  // Internal storage for elements

    // Initialize the custom array with a specified initial size
    init(initialSize: Int) {
        elements = Array(repeating: nil, count: initialSize)
    }

    // Append an element to the end of the custom array
    mutating func append(_ element: T) {
        elements.append(element)
    }

    // Insert an element at the specified index
    mutating func insert(_ element: T, at index: Int) {
        elements.insert(element, at: index)
    }

    // Remove the first occurrence of the specified element
    mutating func remove(_ element: T) {
        if let index = elements.firstIndex(of: element) {
            elements.remove(at: index)
        }
    }

    // Remove and return the element at the specified index
    mutating func pop(at index: Int) -> T? {
        guard index >= 0, index < elements.count else { return nil }
        return elements.remove(at: index)
    }

    // Get the element at the specified index
    func get(at index: Int) -> T? {
        guard index >= 0, index < elements.count else { return nil }
        return elements[index]
    }

    // Set the element at the specified index to the given element
    mutating func set(_ element: T, at index: Int) {
        guard index >= 0, index < elements.count else { return }
        elements[index] = element
    }

    // Return the current size of the custom array
    func size() -> Int {
        return elements.count
    }

    // Check if the custom array is empty
    func isEmpty() -> Bool {
        return elements.isEmpty
    }

    // Return the index of the first occurrence of the specified element, or nil if not found
    func index(of element: T) -> Int? {
        return elements.firstIndex(of: element)
    }

    // Return the number of occurrences of the specified element in the custom array
    func count(of element: T) -> Int {
        return elements.filter { $0 == element }.count
    }

    // Reverse the order of elements in the custom array
    mutating func reverse() {
        elements.reverse()
    }

    // Sort the elements in the custom array in ascending order
    mutating func sort() {
        elements.sort(by: { (first, second) -> Bool in
            if let firstComparable = first as? Comparable, let secondComparable = second as? Comparable {
                return firstComparable < secondComparable
            }
            return false
        })
    }

    // Return a new array containing elements from the specified start index to the end index
    func slice(start: Int, end: Int) -> [T] {
        guard start >= 0, end < elements.count, start <= end else { return [] }
        return Array(elements[start...end])
    }

    // Append elements from another custom array to the end of this custom array
    mutating func extend(_ otherArray: CustomArray) {
        elements.append(contentsOf: otherArray.elements)
    }

    // Remove all elements from the custom array
    mutating func clear() {
        elements.removeAll()
    }

    // Check if the custom array contains the specified element
    func contains(_ element: T) -> Bool {
        return elements.contains(element)
    }

    // Return a new array with only unique elements
    func unique() -> [T] {
        return Array(Set(elements))
    }

    // Apply a transformation function to each element of the custom array
    mutating func map<U>(_ transform: (T) -> U) -> [U] {
        return elements.map(transform)
    }

    // Return a new array with elements that satisfy the given filtering function
    func filter(_ isIncluded: (T) -> Bool) -> [T] {
        return elements.filter(isIncluded)
    }

    // Parameter closure: A closure that takes an element of the array as a parameter.
    func forEach(_ closure: (T) -> Void) {
        for element in elements {
            closure(element)
        }
    }
}
