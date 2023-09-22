class KeyValue:
    def __init__(self, key, value):
        self.key = key
        self.value = value


class HashMap:
    def __init__(self, capacity):
        self.capacity = max(1, capacity)
        self.buckets = [[] for _ in range(self.capacity)]

    # Insertion
    def set_value(self, value, key):
        index = self.bucket_index(key)
        for i, kv in enumerate(self.buckets[index]):
            if kv.key == key:
                self.buckets[index][i].value = value
                return
        self.buckets[index].append(KeyValue(key, value))

    # Retrieval
    def get_value(self, key):
        index = self.bucket_index(key)
        for kv in self.buckets[index]:
            if kv.key == key:
                return kv.value
        return None

    # Removal
    def remove_value(self, key):
        index = self.bucket_index(key)
        self.buckets[index] = [kv for kv in self.buckets[index] if kv.key != key]

    # Count
    def count(self):
        return sum(len(bucket) for bucket in self.buckets)

    # Keys and Values
    def all_keys(self):
        return [kv.key for bucket in self.buckets for kv in bucket]

    def all_values(self):
        return [kv.value for bucket in self.buckets for kv in bucket]

    # Helper Functions
    def is_empty(self):
        return all(not bucket for bucket in self.buckets)

    def remove_all(self):
        self.buckets = [[] for _ in range(self.capacity)]

    def contains(self, key):
        index = self.bucket_index(key)
        return any(kv.key == key for kv in self.buckets[index])

    def load_factor(self):
        return self.count() / self.capacity

    def key_value_pairs(self):
        return {kv.key: kv.value for bucket in self.buckets for kv in bucket}

    # Update Values
    def update_value(self, value, key):
        index = self.bucket_index(key)
        for i, kv in enumerate(self.buckets[index]):
            if kv.key == key:
                self.buckets[index][i].value = value
                return
        self.buckets[index].append(KeyValue(key, value))

    def update_values(self, dictionary):
        for key, value in dictionary.items():
            self.update_value(value, key)

    # ... Add more hashmap operations as needed ...

    def bucket_index(self, key):
        # Simplified hash function for demonstration purposes
        return hash(key) % self.capacity


if __name__ == "__main__":
    hashmap = HashMap(10)
    hashmap.set_value(5, "five")
    hashmap.set_value(10, "ten")

    print("Value for key 'five':", hashmap.get_value("five"))
    print("Value for key 'ten':", hashmap.get_value("ten"))

    hashmap.remove_value("five")
    print("Contains key 'five':", hashmap.contains("five"))
    print("Total count:", hashmap.count())

    keys = hashmap.all_keys()
    print("All keys:", keys)

    values = hashmap.all_values()
    print("All values:", values)

    print("Is empty:", hashmap.is_empty())
    print("Load factor:", hashmap.load_factor())
