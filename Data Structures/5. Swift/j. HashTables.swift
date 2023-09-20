
struct HashTable<Key: Hashable, Value> {
    private typealias Element = (key: Key, value: Value)
    private typealias Bucket = [Element]

    private var buckets: [Bucket]

    private var totalBuckets: Int {
        return buckets.count
    }

    // MARK: - Initialization

    init(capacity: Int) {
        assert(capacity > 0, "Capacity should be greater than 0")
        self.buckets = Array(repeating: [], count: capacity)
    }

    // MARK: - Access

    /// Get the value associated with the given key.
    func getValue(forKey key: Key) -> Value? {
        let index = bucketIndex(for: key)
        for element in buckets[index] where element.key == key {
            return element.value
        }
        return nil
    }

    /// Set the value for the given key.
    mutating func setValue(_ value: Value, forKey key: Key) {
        let index = bucketIndex(for: key)

        // Check if the key already exists, and update the value
        for (i, element) in buckets[index].enumerated() {
            if element.key == key {
                buckets[index][i].value = value
                return
            }
        }

        // Key doesn't exist, add a new entry
        buckets[index].append((key: key, value: value))
    }

    // MARK: - Removal

    /// Remove the value associated with the given key.
    mutating func removeValue(forKey key: Key) {
        let index = bucketIndex(for: key)
        buckets[index] = buckets[index].filter { $0.key != key }
    }

    /// Remove all elements from the hash table.
    mutating func removeAll() {
        buckets = Array(repeating: [], count: totalBuckets)
    }

    // MARK: - Helpers

    /// Get the index of the bucket for the given key.
    private func bucketIndex(for key: Key) -> Int {
        return abs(key.hashValue) % totalBuckets
    }

    // MARK: - Count

    /// Get the total count of elements in the hash table.
    func count() -> Int {
        return buckets.reduce(0) { $0 + $1.count }
    }

    // MARK: - Check Existence

    /// Check if the hash table contains a value for the given key.
    func contains(key: Key) -> Bool {
        let index = bucketIndex(for: key)
        return buckets[index].contains { $0.key == key }
    }

    // MARK: - Keys and Values

    /// Get an array of all keys in the hash table.
    func allKeys() -> [Key] {
        let keys = buckets.flatMap { $0.map { $0.key } }
        return keys
    }

    /// Get an array of all values in the hash table.
    func allValues() -> [Value] {
        let values = buckets.flatMap { $0.map { $0.value } }
        return values
    }

    // MARK: - Merging

    /// Merge the contents of another hash table into this hash table.
    mutating func merge(with otherTable: HashTable<Key, Value>) {
        for bucket in otherTable.buckets {
            for element in bucket {
                setValue(element.value, forKey: element.key)
            }
        }
    }

    // MARK: - Resizing

    /// Resize the hash table to the new capacity.
    mutating func resize(to newCapacity: Int) {
        // Create a new hash table with the desired capacity
        var newHashTable = HashTable<Key, Value>(capacity: newCapacity)

        // Merge the elements from the current hash table
        newHashTable.merge(with: self)

        // Assign the new hash table
        self = newHashTable
    }

    // ... Add more hash table operations as needed ...
}
