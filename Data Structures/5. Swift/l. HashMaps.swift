struct KeyValue<T: Hashable, U> {
    let key: T
    var value: U
}

struct HashMap<T: Hashable, U> {
    private var buckets: [[KeyValue<T, U>]]
    private var capacity: Int

    init(capacity: Int) {
        self.capacity = max(1, capacity)
        self.buckets = Array(repeating: [], count: self.capacity)
    }

    // MARK: - Insertion

    /// Insert a value for a given key.
    mutating func setValue(_ value: U, forKey key: T) {
        let index = bucketIndex(for: key)
        for (i, kv) in buckets[index].enumerated() {
            if kv.key == key {
                buckets[index][i].value = value
                return
            }
        }
        buckets[index].append(KeyValue(key: key, value: value))
    }

    // MARK: - Retrieval

    /// Get the value for a given key.
    func getValue(forKey key: T) -> U? {
        let index = bucketIndex(for: key)
        for kv in buckets[index] {
            if kv.key == key {
                return kv.value
            }
        }
        return nil
    }

    // MARK: - Removal

    /// Remove the value for a given key.
    mutating func removeValue(forKey key: T) {
        let index = bucketIndex(for: key)
        buckets[index] = buckets[index].filter { $0.key != key }
    }

    // MARK: - Count

    /// Get the total count of elements in the hashmap.
    func count() -> Int {
        return buckets.reduce(0) { $0 + $1.count }
    }

    // MARK: - Keys and Values

    /// Get an array of all keys in the hashmap.
    func allKeys() -> [T] {
        return buckets.flatMap { $0.map { $0.key } }
    }

    /// Get an array of all values in the hashmap.
    func allValues() -> [U] {
        return buckets.flatMap { $0.map { $0.value } }
    }

    // MARK: - Helper Functions

    /// Check if the hashmap is empty.
    func isEmpty() -> Bool {
        return buckets.allSatisfy { $0.isEmpty }
    }

    /// Clear all key-value pairs in the hashmap.
    mutating func removeAll() {
        buckets = Array(repeating: [], count: capacity)
    }

    /// Check if a key exists in the hashmap.
    func contains(key: T) -> Bool {
        return buckets[bucketIndex(for: key)].contains { $0.key == key }
    }

    /// Get the load factor of the hashmap.
    func loadFactor() -> Double {
        return Double(count()) / Double(capacity)
    }

    /// Get a set of key-value pairs in the hashmap.
    func keyValuePairs() -> Set<KeyValue<T, U>> {
        var set = Set<KeyValue<T, U>>()
        for bucket in buckets {
            set.formUnion(bucket)
        }
        return set
    }

    // MARK: - Update Values

    /// Update a value for a given key. If the key doesn't exist, it inserts a new key-value pair.
    mutating func updateValue(_ value: U, forKey key: T) {
        let index = bucketIndex(for: key)
        for (i, kv) in buckets[index].enumerated() {
            if kv.key == key {
                buckets[index][i].value = value
                return
            }
        }
        buckets[index].append(KeyValue(key: key, value: value))
    }

    /// Update values for multiple keys using a dictionary.
    mutating func updateValues(_ dictionary: [T: U]) {
        for (key, value) in dictionary {
            updateValue(value, forKey: key)
        }
    }

    // ... Add more hashmap operations as needed ...
}
