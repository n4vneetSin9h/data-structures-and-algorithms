import heapq

class Heap:
    def __init__(self, is_min_heap=False):
        self.elements = []
        self.is_min_heap = is_min_heap
        self.comparator = (lambda a, b: a < b) if is_min_heap else (lambda a, b: a > b)

    # MARK: - Insertion

    def insert(self, element):
        heapq.heappush(self.elements, element)
        self._heapify_up()

    def _heapify_up(self):
        index = len(self.elements) - 1
        while index > 0:
            parent_index = (index - 1) // 2

            if self._should_swap(parent_index, index):
                self.elements[parent_index], self.elements[index] = self.elements[index], self.elements[parent_index]
                index = parent_index
            else:
                break

    # MARK: - Removal

    def remove(self):
        if self.elements:
            root = self.elements[0]
            last_element = self.elements.pop()
            if self.elements:
                self.elements[0] = last_element
                self._heapify_down()
            return root
        return None

    def _heapify_down(self):
        index = 0
        while self._has_left_child(index):
            smallest_child_index = self._left_child_index(index)

            if self._has_right_child(index) and self._should_swap(smallest_child_index, self._right_child_index(index)):
                smallest_child_index = self._right_child_index(index)

            if self._should_swap(index, smallest_child_index):
                self.elements[index], self.elements[smallest_child_index] = self.elements[smallest_child_index], self.elements[index]
                index = smallest_child_index
            else:
                break

    # MARK: - Peek

    def peek(self):
        return self.elements[0] if self.elements else None

    # MARK: - Helper Methods

    def _left_child_index(self, index):
        return 2 * index + 1

    def _right_child_index(self, index):
        return 2 * index + 2

    def _has_left_child(self, index):
        return self._left_child_index(index) < len(self.elements)

    def _has_right_child(self, index):
        return self._right_child_index(index) < len(self.elements)

    def _should_swap(self, index1, index2):
        return self.comparator(self.elements[index1], self.elements[index2])

    # MARK: - Build Heap

    def build_heap(self, elements):
        self.elements = elements[:]
        heapq.heapify(self.elements)

    # MARK: - Replace Root

    def replace_root(self, element):
        if self.elements:
            heapq.heappushpop(self.elements, element)
        else:
            self.insert(element)

    # MARK: - Remove at Index

    def remove_at_index(self, index):
        if 0 <= index < len(self.elements):
            element = self.elements[index]
            self.elements[index] = self.elements[-1]
            self.elements.pop()
            self._heapify_down(index)
            return element
        return None

    # MARK: - Sort

    def sort(self):
        sorted_elements = []
        while self.elements:
            sorted_elements.append(self.remove())
        return sorted_elements

    # MARK: - Index for Element

    def index(self, element):
        try:
            return self.elements.index(element)
        except ValueError:
            return None

    # ... Add more heap operations as needed ...

# Example usage
max_heap = Heap()
max_heap.insert(3)
max_heap.insert(1)
max_heap.insert(4)
max_heap.insert(1)
max_heap.insert(5)
max_heap.insert(9)

print("Max Heap:", max_heap.elements)

min_heap = Heap(is_min_heap=True)
min_heap.insert(3)
min_heap.insert(1)
min_heap.insert(4)
min_heap.insert(1)
min_heap.insert(5)
min_heap.insert(9)

print("Min Heap:", min_heap.elements)
