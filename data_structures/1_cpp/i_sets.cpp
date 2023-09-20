#include <unordered_map>
#include <iostream>

template <typename T>
class CustomSet {
private:
    std::unordered_map<T, bool> elements;  // Hash table to mimic a set

public:
    // Basic Set Operations

    bool isEmpty() const {
        return elements.empty();
    }

    int count() const {
        return elements.size();
    }

    void insert(const T& element) {
        elements[element] = true;
    }

    bool remove(const T& element) {
        return elements.erase(element) > 0;
    }

    bool contains(const T& element) const {
        return elements.find(element) != elements.end();
    }

    // Set Operations

    CustomSet<T> unionWith(const CustomSet<T>& otherSet) const {
        CustomSet<T> newSet(*this);
        for (const auto& entry : otherSet.elements) {
            newSet.insert(entry.first);
        }
        return newSet;
    }

    CustomSet<T> intersectionWith(const CustomSet<T>& otherSet) const {
        CustomSet<T> newSet;
        for (const auto& entry : elements) {
            if (otherSet.contains(entry.first)) {
                newSet.insert(entry.first);
            }
        }
        return newSet;
    }

    CustomSet<T> differenceWith(const CustomSet<T>& otherSet) const {
        CustomSet<T> newSet(*this);
        for (const auto& entry : otherSet.elements) {
            newSet.remove(entry.first);
        }
        return newSet;
    }

    bool isSubsetOf(const CustomSet<T>& otherSet) const {
        for (const auto& entry : elements) {
            if (!otherSet.contains(entry.first)) {
                return false;
            }
        }
        return true;
    }

    bool isSupersetOf(const CustomSet<T>& otherSet) const {
        return otherSet.isSubsetOf(*this);
    }

    bool isDisjointWith(const CustomSet<T>& otherSet) const {
        for (const auto& entry : elements) {
            if (otherSet.contains(entry.first)) {
                return false;
            }
        }
        return true;
    }

    void forEach(void (*closure)(const T&)) const {
        for (const auto& entry : elements) {
            closure(entry.first);
        }
    }

    void removeAll() {
        elements.clear();
    }

    void removeAllOccurrencesOf(const T& element) {
        elements.erase(element);
    }

    CustomSet<T> symmetricDifferenceWith(const CustomSet<T>& otherSet) const {
        CustomSet<T> diff1 = this->differenceWith(otherSet);
        CustomSet<T> diff2 = otherSet.differenceWith(*this);
        return diff1.unionWith(diff2);
    }

    void formUnionWith(const CustomSet<T>& otherSet) {
        *this = this->unionWith(otherSet);
    }

    void formIntersectionWith(const CustomSet<T>& otherSet) {
        *this = this->intersectionWith(otherSet);
    }

    void subtract(const CustomSet<T>& otherSet) {
        *this = this->differenceWith(otherSet);
    }

    // ... Add more set operations as needed ...
};

// Example usage
int main() {
    CustomSet<int> set1;
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);

    CustomSet<int> set2;
    set2.insert(2);
    set2.insert(3);
    set2.insert(4);

    CustomSet<int> unionSet = set1.unionWith(set2);
    std::cout << "Union set: ";
    unionSet.forEach([](const int& element) {
        std::cout << element << " ";
    });
    std::cout << std::endl;

    CustomSet<int> intersectionSet = set1.intersectionWith(set2);
    std::cout << "Intersection set: ";
    intersectionSet.forEach([](const int& element) {
        std::cout << element << " ";
    });
    std::cout << std::endl;

    return 0;
}
