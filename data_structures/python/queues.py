from typing import List, Optional

class CustomQueue:
    def __init__(self):
        self.elements = []

    # MARK: - Queue Operations

    # Enqueue an element to the back of the queue.
    def enqueue(self, element):
        self.elements.append(element)

    # Dequeue an element from the front of the queue.
    def dequeue(self) -> Optional[list]:
        if self.is_empty():
            return None
        return self.elements.pop(0)

    # Peek at the front element in the queue without dequeuing.
    def peek(self) -> Optional[list]:
        if self.is_empty():
            return None
        return self.elements[0]

    # Check if the queue is empty.
    def is_empty(self) -> bool:
        return len(self.elements) == 0

    # Get the number of elements in the queue.
    def count(self) -> int:
        return len(self.elements)

    # MARK: - Queue Operations with List

    # Initialize the queue with a list of elements.
    def initialize_with_list(self, lst: List):
        self.elements = lst.copy()

    # Enqueue a list of elements to the back of the queue.
    def enqueue_list(self, lst: List):
        self.elements.extend(lst)

    # Dequeue a specified number of elements from the front of the queue.
    def dequeue_list(self, count: int) -> List:
        if count <= 0:
            return []
        dequeued_elements = self.elements[:count]
        self.elements = self.elements[count:]
        return dequeued_elements

    # MARK: - Clear and Reset

    # Remove all elements from the queue.
    def clear(self):
        self.elements = []

    # MARK: - Checking for Element

    # Check if the queue contains a specific element.
    def contains(self, element) -> bool:
        return element in self.elements

    # MARK: - Filtering

    # Filter the queue using a given predicate.
    def filter(self, predicate):
        return list(filter(predicate, self.elements))

    # MARK: - Conversion to List

    # Convert the queue to a list.
    def to_list(self) -> List:
        return self.elements.copy()

    # MARK: - Map

    # Transform each element in the queue using a provided transform function.
    def map(self, transform):
        return [transform(element) for element in self.elements]

    # MARK: - Reduce

    # Combine all elements in the queue using a reducer function.
    def reduce(self, initial_result, reducer):
        result = initial_result
        for element in self.elements:
            result = reducer(result, element)
        return result

    # MARK: - Concatenate

    # Concatenate another queue to this queue.
    def concatenate(self, other_queue):
        self.elements.extend(other_queue.elements)

    # MARK: - Subscript

    # Access the element at a specific index in the queue.
    def __getitem__(self, index):
        if 0 <= index < len(self.elements):
            return self.elements[index]
        else:
            return None

    # ... Add more queue operations as needed ...


# Test the CustomQueue class
if __name__ == "__main__":
    queue = CustomQueue()
    queue.enqueue(1)
    queue.enqueue(2)
    queue.enqueue(3)

    print("Queue:")
    while not queue.is_empty():
        print(queue.dequeue())
