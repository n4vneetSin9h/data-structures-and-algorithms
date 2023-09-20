#include <iostream>
#include <vector>
#include <functional>

template <typename Key, typename Value>
class HashTable {
private:
    using Element = std::pair<Key, Value>;
    using Bucket = std::vector<Element>;

    std::vector<Bucket> buckets;

    size_t totalBuckets() const {
        return buckets.size();
    }

public:
    // Initialization
    HashTable(size_t capacity) {
        assert(capacity > 0 && "Capacity should be greater than 0");
        buckets = std::vector<Bucket>(capacity);
    }

    // Access
    Value* getValue(const Key& key) {
        size_t index = bucketIndex(key);
        for (const Element& element : buckets[index]) {
            if (element.first == key) {
                return &buckets[index][element];
            }
        }
        return nullptr;
    }

    void setValue(const Value& value, const Key& key) {
        size_t index = bucketIndex(key);

        // Check if the key already exists, and update the value
        for (auto& element : buckets[index]) {
            if (element.first == key) {
                element.second = value;
                return;
            }
        }

        // Key doesn't exist, add a new entry
        buckets[index].emplace_back(key, value);
    }

    // Removal
    void removeValue(const Key& key) {
        size_t index = bucketIndex(key);
        buckets[index].erase(std::remove_if(buckets[index].begin(), buckets[index].end(),
            [key](const Element& element) { return element.first == key; }), buckets[index].end());
    }

    void removeAll() {
        for (auto& bucket : buckets) {
            bucket.clear();
        }
    }

    // Helpers
    size_t bucketIndex(const Key& key) const {
        return std::hash<Key>{}(key) % totalBuckets();
    }

    // Count
    size_t count() const {
        size_t totalCount = 0;
        for (const auto& bucket : buckets) {
            totalCount += bucket.size();
        }
        return totalCount;
    }

    // Check Existence
    bool contains(const Key& key) const {
        size_t index = bucketIndex(key);
        for (const auto& element : buckets[index]) {
            if (element.first == key) {
                return true;
            }
        }
        return false;
    }

    // Keys and Values
    std::vector<Key> allKeys() const {
        std::vector<Key> keys;
        for (const auto& bucket : buckets) {
            for (const auto& element : bucket) {
                keys.push_back(element.first);
            }
        }
        return keys;
    }

    std::vector<Value> allValues() const {
        std::vector<Value> values;
        for (const auto& bucket : buckets) {
            for (const auto& element : bucket) {
                values.push_back(element.second);
            }
        }
        return values;
    }

    // Merging
    void merge(const HashTable<Key, Value>& otherTable) {
        for (const auto& bucket : otherTable.buckets) {
            for (const auto& element : bucket) {
                setValue(element.second, element.first);
            }
        }
    }

    // Resizing
    void resize(size_t newCapacity) {
        // Create a new hash table with the desired capacity
        HashTable<Key, Value> newHashTable(newCapacity);

        // Merge the elements from the current hash table
        newHashTable.merge(*this);

        // Assign the new hash table
        *this = newHashTable;
    }
};

// ... Add more hash table operations as needed ...

int main() {
    HashTable<int, std::string> table(10);

    table.setValue("Value1", 1);
    table.setValue("Value2", 2);
    table.setValue("Value3", 3);

    std::cout << "Value for key 2: " << *(table.getValue(2)) << "\n";

    table.removeValue(2);
    std::cout << "Count after removal: " << table.count() << "\n";

    std::vector<int> keys = table.allKeys();
    std::cout << "Keys in the table:";
    for (const auto& key : keys) {
        std::cout << " " << key;
    }
    std::cout << "\n";

    return 0;
}
