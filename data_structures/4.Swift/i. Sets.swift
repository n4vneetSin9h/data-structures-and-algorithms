struct CustomSet<T: Hashable> {
    private var elements: [T: Bool]  // Dictionary to mimic a set

    // MARK: - Basic Set Operations

    /// Check if the set is empty.
    func isEmpty() -> Bool {
        return elements.isEmpty
    }

    /// Get the number of elements in the set.
    func count() -> Int {
        return elements.count
    }

    /// Insert an element into the set.
    func insert(_ element: T) {
        elements[element] = true
    }

    /// Remove an element from the set.
    @discardableResult
    func remove(_ element: T) -> T? {
        return elements.removeValue(forKey: element) != nil ? element : nil
    }

    /// Check if the set contains a specific element.
    func contains(_ element: T) -> Bool {
        return elements[element] != nil
    }

    // MARK: - Set Operations

    /// Perform the union operation with another set and return a new set.
    func union(with otherSet: CustomSet<T>) -> CustomSet<T> {
        var newSet = self
        for element in otherSet.elements.keys {
            newSet.insert(element)
        }
        return newSet
    }

    /// Perform the intersection operation with another set and return a new set.
    func intersection(with otherSet: CustomSet<T>) -> CustomSet<T> {
        var newSet = CustomSet<T>()
        for element in elements.keys {
            if otherSet.contains(element) {
                newSet.insert(element)
            }
        }
        return newSet
    }

    /// Perform the difference operation with another set and return a new set.
    func difference(with otherSet: CustomSet<T>) -> CustomSet<T> {
        var newSet = self
        for element in otherSet.elements.keys {
            newSet.remove(element)
        }
        return newSet
    }

    /// Check if this set is a subset of another set.
    func isSubset(of otherSet: CustomSet<T>) -> Bool {
        for element in elements.keys {
            if !otherSet.contains(element) {
                return false
            }
        }
        return true
    }

    /// Check if this set is a superset of another set.
    func isSuperset(of otherSet: CustomSet<T>) -> Bool {
        return otherSet.isSubset(of: self)
    }

    /// Check if this set is disjoint with another set.
    func isDisjoint(with otherSet: CustomSet<T>) -> Bool {
        for element in elements.keys {
            if otherSet.contains(element) {
                return false
            }
        }
        return true
    }

    /// Apply a closure to each element in the set.
    func forEach(_ closure: (T) -> Void) {
        for element in elements.keys {
            closure(element)
        }
    }

    /// Remove all elements from the set.
    mutating func removeAll() {
        elements.removeAll()
    }

    /// Remove all occurrences of an element from the set.
    mutating func removeAllOccurrences(of element: T) {
        elements[element] = nil
    }

    /// Get the set with common elements between this set and another set.
    func symmetricDifference(with otherSet: CustomSet<T>) -> CustomSet<T> {
        let diff1 = self.difference(with: otherSet)
        let diff2 = otherSet.difference(with: self)
        return diff1.union(with: diff2)
    }

    /// Insert elements from another set into this set.
    mutating func formUnion(with otherSet: CustomSet<T>) {
        self = self.union(with: otherSet)
    }

    /// Remove elements of this set that are not in another set.
    mutating func formIntersection(with otherSet: CustomSet<T>) {
        self = self.intersection(with: otherSet)
    }

    /// Remove elements of this set that are in another set.
    mutating func subtract(_ otherSet: CustomSet<T>) {
        self = self.difference(with: otherSet)
    }

    // ... Add more set operations as needed ...
}
