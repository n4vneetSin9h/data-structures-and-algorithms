#include <iostream>
#include <vector>
#include <functional>

template <typename T>
class Heap {
private:
    std::vector<T> elements;
    bool isMinHeap;
    std::function<bool(T, T)> comparator;

public:
    Heap(bool isMinHeap = false, std::function<bool(T, T)> comparator = [](T a, T b) { return a < b; }) {
        this->isMinHeap = isMinHeap;
        this->comparator = comparator;
    }

    // MARK: - Insertion

    void insert(T element) {
        elements.push_back(element);
        heapifyUp();
    }

    void heapifyUp() {
        int currentIndex = elements.size() - 1;
        while (currentIndex > 0) {
            int parentIndex = (currentIndex - 1) / 2;

            if (shouldSwap(parentIndex, currentIndex)) {
                std::swap(elements[parentIndex], elements[currentIndex]);
                currentIndex = parentIndex;
            } else {
                break;
            }
        }
    }

    // MARK: - Removal

    T remove() {
        if (elements.empty()) {
            throw std::runtime_error("Heap is empty");
        }

        T root = elements[0];
        elements[0] = elements.back();
        elements.pop_back();
        heapifyDown();

        return root;
    }

    void heapifyDown() {
        int currentIndex = 0;

        while (hasLeftChild(currentIndex)) {
            int smallestChildIndex = leftChildIndex(currentIndex);

            if (hasRightChild(currentIndex) && shouldSwap(rightChildIndex(currentIndex), smallestChildIndex)) {
                smallestChildIndex = rightChildIndex(currentIndex);
            }

            if (shouldSwap(smallestChildIndex, currentIndex)) {
                std::swap(elements[smallestChildIndex], elements[currentIndex]);
                currentIndex = smallestChildIndex;
            } else {
                break;
            }
        }
    }

    // MARK: - Peek

    T peek() const {
        if (elements.empty()) {
            throw std::runtime_error("Heap is empty");
        }

        return elements[0];
    }

    // MARK: - Helper Methods

    int leftChildIndex(int parentIndex) const {
        return 2 * parentIndex + 1;
    }

    int rightChildIndex(int parentIndex) const {
        return 2 * parentIndex + 2;
    }

    bool hasLeftChild(int index) const {
        return leftChildIndex(index) < elements.size();
    }

    bool hasRightChild(int index) const {
        return rightChildIndex(index) < elements.size();
    }

    bool shouldSwap(int firstIndex, int secondIndex) const {
        if (isMinHeap) {
            return comparator(elements[secondIndex], elements[firstIndex]);
        } else {
            return comparator(elements[firstIndex], elements[secondIndex]);
        }
    }

    // MARK: - Build Heap

    void buildHeap(const std::vector<T>& elements) {
        this->elements = elements;
        for (int i = (elements.size() / 2) - 1; i >= 0; --i) {
            heapifyDown(i);
        }
    }

    // MARK: - Replace Root

    void replaceRoot(T element) {
        if (elements.empty()) {
            insert(element);
            return;
        }

        elements[0] = element;
        heapifyDown();
    }

    // MARK: - Remove at Index

    T removeAtIndex(int index) {
        if (index >= elements.size()) {
            throw std::out_of_range("Index out of range");
        }

        if (index == elements.size() - 1) {
            T removed = elements.back();
            elements.pop_back();
            return removed;
        }

        std::swap(elements[index], elements.back());
        T removed = elements.back();
        elements.pop_back();
        heapifyDown(index);

        return removed;
    }

    // MARK: - Sort

    void sort() {
        for (int i = elements.size() - 1; i >= 1; --i) {
            std::swap(elements[0], elements[i]);
            heapifyDown(0, i);
        }
    }

    // MARK: - Index for Element

    int index(const T& element) const {
        return index(element, 0);
    }

    int index(const T& element, int startIndex) const {
        if (startIndex >= elements.size()) {
            return -1;
        }

        if (elements[startIndex] == element) {
            return startIndex;
        }

        int leftIndex = leftChildIndex(startIndex);
        int rightIndex = rightChildIndex(startIndex);

        if (leftIndex < elements.size()) {
            int result = index(element, leftIndex);
            if (result != -1) {
                return result;
            }
        }

        if (rightIndex < elements.size()) {
            return index(element, rightIndex);
        }

        return -1;
    }

    // ... Add more heap operations as needed ...
};
