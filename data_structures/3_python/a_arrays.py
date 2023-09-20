class CustomArray:
    def __init__(self, initial_size):
        # Internal storage for elements
        self.elements = [None] * initial_size

    # Append an element to the end of the custom array
    def append(self, element):
        self.elements.append(element)

    # Insert an element at the specified index
    def insert(self, element, index):
        self.elements.insert(index, element)

    # Remove the first occurrence of the specified element
    def remove(self, element):
        if element in self.elements:
            self.elements.remove(element)

    # Remove and return the element at the specified index
    def pop(self, index):
        if 0 <= index < len(self.elements):
            return self.elements.pop(index)
        else:
            return None

    # Get the element at the specified index
    def get(self, index):
        if 0 <= index < len(self.elements):
            return self.elements[index]
        else:
            return None

    # Set the element at the specified index to the given element
    def set(self, element, index):
        if 0 <= index < len(self.elements):
            self.elements[index] = element

    # Return the current size of the custom array
    def size(self):
        return len(self.elements)

    # Check if the custom array is empty
    def is_empty(self):
        return len(self.elements) == 0

    # Return the index of the first occurrence of the specified element, or None if not found
    def index(self, element):
        if element in self.elements:
            return self.elements.index(element)
        else:
            return None

    # Return the number of occurrences of the specified element in the custom array
    def count(self, element):
        return self.elements.count(element)

    # Reverse the order of elements in the custom array
    def reverse(self):
        self.elements.reverse()

    # Sort the elements in the custom array in ascending order
    def sort(self):
        self.elements.sort()

    # Return a new list containing elements from the specified start index to the end index
    def slice(self, start, end):
        if 0 <= start <= end < len(self.elements):
            return self.elements[start:end + 1]
        else:
            return []

    # Append elements from another custom array to the end of this custom array
    def extend(self, other_array):
        self.elements.extend(other_array.elements)

    # Remove all elements from the custom array
    def clear(self):
        self.elements = []

    # Check if the custom array contains the specified element
    def contains(self, element):
        return element in self.elements

    # Return a new list with only unique elements
    def unique(self):
        return list(set(self.elements))

    # Apply a transformation function to each element of the custom array
    def map(self, transform):
        return [transform(element) for element in self.elements]

    # Return a new list with elements that satisfy the given filtering function
    def filter(self, is_included):
        return [element for element in self.elements if is_included(element)]

    # Parameter closure: A function that takes an element of the array as a parameter.
    def for_each(self, closure):
        for element in self.elements:
            closure(element)


# Example usage
custom_array = CustomArray(5)
custom_array.append(10)
custom_array.append(20)
custom_array.insert(15, 1)

print("Size of array:", custom_array.size())

# Print each element
custom_array.for_each(lambda element: print(element))
