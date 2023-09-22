class HashTable:
    def __init__(self, capacity):
        assert capacity > 0, "Capacity should be greater than 0"
        self.buckets = [[] for _ in range(capacity)]

    def bucket_index(self, key):
        return hash(key) % len(self.buckets)

    def get_value(self, key):
        index = self.bucket_index(key)
        for k, v in self.buckets[index]:
            if k == key:
                return v
        return None

    def set_value(self, key, value):
        index = self.bucket_index(key)

        # Check if the key already exists, and update the value
        for i, (k, _) in enumerate(self.buckets[index]):
            if k == key:
                self.buckets[index][i] = (key, value)
                return

        # Key doesn't exist, add a new entry
        self.buckets[index].append((key, value))

    def remove_value(self, key):
        index = self.bucket_index(key)
        self.buckets[index] = [(k, v) for k, v in self.buckets[index] if k != key]

    def remove_all(self):
        self.buckets = [[] for _ in range(len(self.buckets))]

    def count(self):
        return sum(len(bucket) for bucket in self.buckets)

    def contains(self, key):
        index = self.bucket_index(key)
        return any(k == key for k, _ in self.buckets[index])

    def all_keys(self):
        keys = [k for bucket in self.buckets for k, _ in bucket]
        return keys

    def all_values(self):
        values = [v for bucket in self.buckets for _, v in bucket]
        return values

    def merge(self, other_table):
        for bucket in other_table.buckets:
            for key, value in bucket:
                self.set_value(key, value)

    def resize(self, new_capacity):
        new_buckets = [[] for _ in range(new_capacity)]
        for bucket in self.buckets:
            for key, value in bucket:
                index = hash(key) % new_capacity
                new_buckets[index].append((key, value))
        self.buckets = new_buckets

    # ... Add more hash table operations as needed ...


# Example usage
table = HashTable(10)

table.set_value(1, "Value1")
table.set_value(2, "Value2")
table.set_value(3, "Value3")

print("Value for key 2:", table.get_value(2))

table.remove_value(2)
print("Count after removal:", table.count())

print("Keys in the table:", table.all_keys())
