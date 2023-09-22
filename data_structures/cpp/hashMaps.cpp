#include <iostream>
#include <vector>
#include <unordered_map>

template <typename T, typename U>
struct KeyValue {
    T key;
    U value;
};

template <typename T, typename U>
class HashMap {
private:
    std::vector<std::vector<KeyValue<T, U>>> buckets;
    int capacity;

    int bucketIndex(const T& key) const {
        // Simplified hash function for demonstration purposes
        return std::hash<T>{}(key) % capacity;
    }

public:
    HashMap(int capacity) : capacity(std::max(1, capacity)) {
        buckets = std::vector<std::vector<KeyValue<T, U>>>(this->capacity);
    }

    // Insertion
    void setValue(const U& value, const T& key) {
        int index = bucketIndex(key);
        for (size_t i = 0; i < buckets[index].size(); ++i) {
            if (buckets[index][i].key == key) {
                buckets[index][i].value = value;
                return;
            }
        }
        buckets[index].push_back({key, value});
    }

    // Retrieval
    U* getValue(const T& key) {
        int index = bucketIndex(key);
        for (const auto& kv : buckets[index]) {
            if (kv.key == key) {
                return &kv.value;
            }
        }
        return nullptr;
    }

    // Removal
    void removeValue(const T& key) {
        int index = bucketIndex(key);
        auto& bucket = buckets[index];
        bucket.erase(std::remove_if(bucket.begin(), bucket.end(),
                                    [key](const KeyValue<T, U>& kv) {
                                        return kv.key == key;
                                    }),
                     bucket.end());
    }

    // Count
    int count() const {
        int total_count = 0;
        for (const auto& bucket : buckets) {
            total_count += bucket.size();
        }
        return total_count;
    }

    // Keys and Values
    std::vector<T> allKeys() const {
        std::vector<T> keys;
        for (const auto& bucket : buckets) {
            for (const auto& kv : bucket) {
                keys.push_back(kv.key);
            }
        }
        return keys;
    }

    std::vector<U> allValues() const {
        std::vector<U> values;
        for (const auto& bucket : buckets) {
            for (const auto& kv : bucket) {
                values.push_back(kv.value);
            }
        }
        return values;
    }

    // Helper Functions
    bool isEmpty() const {
        return std::all_of(buckets.begin(), buckets.end(),
                           [](const std::vector<KeyValue<T, U>>& bucket) {
                               return bucket.empty();
                           });
    }

    void removeAll() {
        buckets = std::vector<std::vector<KeyValue<T, U>>>(capacity);
    }

    bool contains(const T& key) const {
        int index = bucketIndex(key);
        for (const auto& kv : buckets[index]) {
            if (kv.key == key) {
                return true;
            }
        }
        return false;
    }

    double loadFactor() const {
        return static_cast<double>(count()) / static_cast<double>(capacity);
    }

    std::unordered_set<KeyValue<T, U>> keyValuePairs() const {
        std::unordered_set<KeyValue<T, U>> set;
        for (const auto& bucket : buckets) {
            for (const auto& kv : bucket) {
                set.insert(kv);
            }
        }
        return set;
    }

    // Update Values
    void updateValue(const U& value, const T& key) {
        int index = bucketIndex(key);
        for (size_t i = 0; i < buckets[index].size(); ++i) {
            if (buckets[index][i].key == key) {
                buckets[index][i].value = value;
                return;
            }
        }
        buckets[index].push_back({key, value});
    }

    void updateValues(const std::unordered_map<T, U>& dictionary) {
        for (const auto& [key, value] : dictionary) {
            updateValue(value, key);
        }
    }
};

int main() {
    HashMap<std::string, int> hashMap(10);
    hashMap.setValue(5, "five");
    hashMap.setValue(10, "ten");
    std::cout << "Value for key 'five': " << *hashMap.getValue("five") << std::endl;
    std::cout << "Value for key 'ten': " << *hashMap.getValue("ten") << std::endl;
    hashMap.removeValue("five");
    std::cout << "Contains key 'five': " << hashMap.contains("five") << std::endl;
    std::cout << "Total count: " << hashMap.count() << std::endl;
    std::vector<std::string> keys = hashMap.allKeys();
    std::cout << "All keys: ";
    for (const auto& key : keys) {
        std::cout << key << " ";
    }
    std::cout << std::endl;
    std::vector<int> values = hashMap.allValues();
    std::cout << "All values: ";
    for (const auto& value : values) {
        std::cout << value << " ";
    }
    std::cout << std::endl;
    std::cout << "Is empty: " << hashMap.isEmpty() << std::endl;
    std::cout << "Load factor: " << hashMap.loadFactor() << std::endl;

    return 0;
}
