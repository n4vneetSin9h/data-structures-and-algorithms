class CustomStack:
    def __init__(self):
        self.elements = []

    # MARK: - Stack Operations

    # Check if the stack is empty.
    def is_empty(self):
        return len(self.elements) == 0

    # Get the number of elements in the stack.
    def count(self):
        return len(self.elements)

    # Push an element onto the stack.
    def push(self, element):
        self.elements.append(element)

    # Pop the top element from the stack.
    def pop(self):
        if self.elements:
            return self.elements.pop()
        else:
            return None

    # Peek at the top element in the stack without removing it.
    def peek(self):
        if self.elements:
            return self.elements[-1]
        else:
            return None

    # MARK: - Stack Operations with List

    # Initialize the stack with a list of elements.
    def initialize_with_list(self, lst):
        self.elements = lst

    # Push a list of elements onto the stack.
    def push_list(self, lst):
        self.elements.extend(lst)

    # Pop a specified number of elements from the stack.
    def pop_list(self, count):
        popped_elements = []
        for _ in range(count):
            if self.elements:
                popped_elements.append(self.elements.pop())
            else:
                break
        return popped_elements

    # MARK: - Clear and Reset

    # Remove all elements from the stack.
    def clear(self):
        self.elements = []

    # MARK: - Checking for Element

    # Check if the stack contains a specific element.
    def contains(self, element):
        return element in self.elements

    # MARK: - Filtering

    # Filter the stack using a given predicate.
    def filter(self, predicate):
        return list(filter(predicate, self.elements))

    # MARK: - Conversion to List

    # Convert the stack to a list.
    def to_list(self):
        return self.elements

    # MARK: - Map

    # Transform each element in the stack using a provided transform function.
    def map(self, transform):
        return [transform(element) for element in self.elements]

    # MARK: - Reduce

    # Combine all elements in the stack using a reducer function.
    def reduce(self, initial_result, reducer):
        result = initial_result
        for element in self.elements:
            result = reducer(result, element)
        return result

    # MARK: - Concatenate

    # Concatenate another stack to this stack.
    def concatenate(self, other_stack):
        self.elements.extend(other_stack.elements)

    # MARK: - Subscript

    # Access the element at a specific index in the stack.
    def __getitem__(self, index):
        if 0 <= index < len(self.elements):
            return self.elements[index]
        else:
            return None

    # ... Add more stack operations as needed ...


# Test the CustomStack class
if __name__ == "__main__":
    stack = CustomStack()
    stack.push(1)
    stack.push(2)
    stack.push(3)

    print("Stack:")
    for i in range(stack.count() - 1, -1, -1):
        element = stack[i]
        if element is not None:
            print(element)
