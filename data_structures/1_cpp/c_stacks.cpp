#include <iostream>
#include <vector>

template<typename T>
class CustomStack {
private:
    std::vector<T> elements;

public:
    // MARK: - Stack Operations

    /// Check if the stack is empty.
    bool isEmpty() const {
        return elements.empty();
    }

    /// Get the number of elements in the stack.
    int count() const {
        return elements.size();
    }

    /// Push an element onto the stack.
    void push(T element) {
        elements.push_back(element);
    }

    /// Pop the top element from the stack.
    T pop() {
        T poppedElement = T();
        if (!elements.empty()) {
            poppedElement = elements.back();
            elements.pop_back();
        }
        return poppedElement;
    }

    /// Peek at the top element in the stack without removing it.
    T peek() const {
        return (elements.empty()) ? T() : elements.back();
    }

    // MARK: - Stack Operations with Vector

    /// Initialize the stack with a vector of elements.
    void initializeWithVector(const std::vector<T>& vec) {
        elements = vec;
    }

    /// Push a vector of elements onto the stack.
    void pushVector(const std::vector<T>& vec) {
        elements.insert(elements.end(), vec.begin(), vec.end());
    }

    /// Pop a specified number of elements from the stack.
    std::vector<T> popVector(int count) {
        std::vector<T> poppedElements;
        for (int i = 0; i < count && !elements.empty(); ++i) {
            poppedElements.push_back(elements.back());
            elements.pop_back();
        }
        return poppedElements;
    }

    // MARK: - Clear and Reset

    /// Remove all elements from the stack.
    void clear() {
        elements.clear();
    }

    // MARK: - Checking for Element

    /// Check if the stack contains a specific element.
    bool contains(T element) const {
        return std::find(elements.begin(), elements.end(), element) != elements.end();
    }

    // MARK: - Filtering

    /// Filter the stack using a given predicate.
    std::vector<T> filter(bool (*predicate)(T)) const {
        std::vector<T> filtered;
        for (const T& element : elements) {
            if (predicate(element)) {
                filtered.push_back(element);
            }
        }
        return filtered;
    }

    // MARK: - Conversion to Vector

    /// Convert the stack to a vector.
    std::vector<T> toVector() const {
        return elements;
    }

    // MARK: - Map

    /// Transform each element in the stack using a provided transform function.
    template<typename U>
    std::vector<U> map(U (*transform)(T)) const {
        std::vector<U> mapped;
        for (const T& element : elements) {
            mapped.push_back(transform(element));
        }
        return mapped;
    }

    // MARK: - Reduce

    /// Combine all elements in the stack using a reducer function.
    template<typename Result>
    Result reduce(Result initialResult, Result (*reducer)(Result, T)) const {
        Result result = initialResult;
        for (const T& element : elements) {
            result = reducer(result, element);
        }
        return result;
    }

    // MARK: - Concatenate

    /// Concatenate another stack to this stack.
    void concatenate(const CustomStack<T>& otherStack) {
        elements.insert(elements.end(), otherStack.elements.begin(), otherStack.elements.end());
    }

    // MARK: - Subscript

    /// Access the element at a specific index in the stack.
    T operator[](int index) const {
        if (index >= 0 && index < elements.size()) {
            return elements[index];
        }
        return T();
    }

    // ... Add more stack operations as needed ...
};

int main() {
    CustomStack<int> stack;
    stack.push(1);
    stack.push(2);
    stack.push(3);

    std::cout << "Stack:" << std::endl;
    for (int i = stack.count() - 1; i >= 0; --i) {
        std::cout << stack[i] << std::endl;
    }

    return 0;
}
