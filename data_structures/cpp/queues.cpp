#include <iostream>
#include <vector>

template <typename T>
class CustomQueue {
private:
    std::vector<T> elements;

public:
    // MARK: - Queue Operations

    /// Enqueue an element to the back of the queue.
    void enqueue(T element) {
        elements.push_back(element);
    }

    /// Dequeue an element from the front of the queue.
    T dequeue() {
        if (elements.empty()) {
            throw std::out_of_range("Queue is empty.");
        }
        T frontElement = elements.front();
        elements.erase(elements.begin());
        return frontElement;
    }

    /// Peek at the front element in the queue without dequeuing.
    T peek() const {
        if (elements.empty()) {
            throw std::out_of_range("Queue is empty.");
        }
        return elements.front();
    }

    /// Check if the queue is empty.
    bool isEmpty() const {
        return elements.empty();
    }

    /// Get the number of elements in the queue.
    int count() const {
        return elements.size();
    }

    // MARK: - Queue Operations with Array

    /// Initialize the queue with an array of elements.
    void initializeWithArray(std::vector<T> array) {
        elements = array;
    }

    /// Enqueue an array of elements to the back of the queue.
    void enqueueArray(const std::vector<T>& array) {
        elements.insert(elements.end(), array.begin(), array.end());
    }

    /// Dequeue an array of elements from the front of the queue.
    std::vector<T> dequeueArray(int count) {
        if (count < 0 || count > elements.size()) {
            throw std::out_of_range("Invalid count for dequeueArray.");
        }

        std::vector<T> dequeuedElements;
        for (int i = 0; i < count; ++i) {
            dequeuedElements.push_back(elements.front());
            elements.erase(elements.begin());
        }
        return dequeuedElements;
    }

    // MARK: - Clear and Reset

    /// Remove all elements from the queue.
    void clear() {
        elements.clear();
    }

    // MARK: - Checking for Element

    /// Check if the queue contains a specific element.
    bool contains(T element) const {
        return std::find(elements.begin(), elements.end(), element) != elements.end();
    }

    // MARK: - Filtering

    /// Filter the queue using a given predicate.
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

    /// Convert the queue to a vector.
    std::vector<T> toVector() const {
        return elements;
    }

    // MARK: - Map

    /// Transform each element in the queue using a provided transform function.
    template <typename U>
    std::vector<U> map(U (*transform)(T)) const {
        std::vector<U> mapped;
        for (const T& element : elements) {
            mapped.push_back(transform(element));
        }
        return mapped;
    }

    // MARK: - Reduce

    /// Combine all elements in the queue using a reducer function.
    template <typename Result>
    Result reduce(Result initialResult, Result (*reducer)(Result, T)) const {
        Result result = initialResult;
        for (const T& element : elements) {
            result = reducer(result, element);
        }
        return result;
    }

    // MARK: - Concatenate

    /// Concatenate another queue to this queue.
    void concatenate(const CustomQueue<T>& otherQueue) {
        elements.insert(elements.end(), otherQueue.elements.begin(), otherQueue.elements.end());
    }

    // MARK: - Subscript

    /// Access the element at a specific index in the queue.
    T operator[](int index) const {
        if (index >= 0 && index < elements.size()) {
            return elements[index];
        }
        throw std::out_of_range("Index out of range.");
    }

    // ... Add more queue operations as needed ...
};

int main() {
    CustomQueue<int> queue;
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    std::cout << "Queue:" << std::endl;
    while (!queue.isEmpty()) {
        std::cout << queue.dequeue() << std::endl;
    }

    return 0;
}
