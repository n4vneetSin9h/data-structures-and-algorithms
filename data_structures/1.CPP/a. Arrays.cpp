#include <iostream>
#include <vector>

template <typename T>
class CustomArray {
private:
    std::vector<T> elements;  // Internal storage for elements

public:
    // Initialize the custom array with a specified initial size
    CustomArray(int initialSize) {
        elements.resize(initialSize);
    }

    // Append an element to the end of the custom array
    void append(T element) {
        elements.push_back(element);
    }

    // Insert an element at the specified index
    void insert(T element, int index) {
        elements.insert(elements.begin() + index, element);
    }

    // Remove the first occurrence of the specified element
    void remove(T element) {
        auto it = std::find(elements.begin(), elements.end(), element);
        if (it != elements.end()) {
            elements.erase(it);
        }
    }

    // Remove and return the element at the specified index
    T pop(int index) {
        if (index >= 0 && index < elements.size()) {
            T element = elements[index];
            elements.erase(elements.begin() + index);
            return element;
        }
        return nullptr; // Returning nullptr in C++ isn't valid, use appropriate handling for the type T
    }

    // Get the element at the specified index
    T get(int index) {
        if (index >= 0 && index < elements.size()) {
            return elements[index];
        }
        return nullptr; // Returning nullptr in C++ isn't valid, use appropriate handling for the type T
    }

    // Set the element at the specified index to the given element
    void set(T element, int index) {
        if (index >= 0 && index < elements.size()) {
            elements[index] = element;
        }
    }

    // Return the current size of the custom array
    int size() {
        return elements.size();
    }

    // Check if the custom array is empty
    bool isEmpty() {
        return elements.empty();
    }

    // Return the index of the first occurrence of the specified element, or -1 if not found
    int index(T element) {
        auto it = std::find(elements.begin(), elements.end(), element);
        if (it != elements.end()) {
            return std::distance(elements.begin(), it);
        }
        return -1;
    }

    // Return the number of occurrences of the specified element in the custom array
    int count(T element) {
        return std::count(elements.begin(), elements.end(), element);
    }

    // Reverse the order of elements in the custom array
    void reverse() {
        std::reverse(elements.begin(), elements.end());
    }

    // Sort the elements in the custom array in ascending order
    void sort() {
        std::sort(elements.begin(), elements.end());
    }

    // Return a new array containing elements from the specified start index to the end index
    std::vector<T> slice(int start, int end) {
        if (start >= 0 && end < elements.size() && start <= end) {
            return std::vector<T>(elements.begin() + start, elements.begin() + end + 1);
        }
        return std::vector<T>();
    }

    // Append elements from another custom array to the end of this custom array
    void extend(const CustomArray& otherArray) {
        elements.insert(elements.end(), otherArray.elements.begin(), otherArray.elements.end());
    }

    // Remove all elements from the custom array
    void clear() {
        elements.clear();
    }

    // Check if the custom array contains the specified element
    bool contains(T element) {
        return std::find(elements.begin(), elements.end(), element) != elements.end();
    }

    // Return a new array with only unique elements
    std::vector<T> unique() {
        std::vector<T> uniqueElements(elements.begin(), elements.end());
        std::sort(uniqueElements.begin(), uniqueElements.end());
        auto last = std::unique(uniqueElements.begin(), uniqueElements.end());
        uniqueElements.erase(last, uniqueElements.end());
        return uniqueElements;
    }

    // Apply a transformation function to each element of the custom array
    template <typename U>
    std::vector<U> map(U (*transform)(T)) {
        std::vector<U> result;
        for (const auto& element : elements) {
            result.push_back(transform(element));
        }
        return result;
    }

    // Return a new array with elements that satisfy the given filtering function
    std::vector<T> filter(bool (*isIncluded)(T)) {
        std::vector<T> result;
        for (const auto& element : elements) {
            if (isIncluded(element)) {
                result.push_back(element);
            }
        }
        return result;
    }

    // Parameter closure: A function that takes an element of the array as a parameter.
    void forEach(void (*closure)(T)) {
        for (const auto& element : elements) {
            closure(element);
        }
    }
};

int main() {
    CustomArray<int> customArray(5);
    customArray.append(10);
    customArray.append(20);
    customArray.insert(15, 1);

    std::cout << "Size of array: " << customArray.size() << "\n";

    // Print each element
    customArray.forEach([](int element) {
        std::cout << element << " ";
    });

    return 0;
}
