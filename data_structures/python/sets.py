class CustomSet:
    def __init__(self):
        self.elements = {}  # Dictionary to mimic a set

    # Basic Set Operations

    def is_empty(self):
        return len(self.elements) == 0

    def count(self):
        return len(self.elements)

    def insert(self, element):
        self.elements[element] = True

    def remove(self, element):
        return self.elements.pop(element, None)

    def contains(self, element):
        return element in self.elements

    # Set Operations

    def union(self, other_set):
        new_set = CustomSet()
        new_set.elements.update(self.elements)
        for element in other_set.elements:
            new_set.elements[element] = True
        return new_set

    def intersection(self, other_set):
        new_set = CustomSet()
        for element in self.elements:
            if element in other_set.elements:
                new_set.elements[element] = True
        return new_set

    def difference(self, other_set):
        new_set = CustomSet()
        new_set.elements.update(self.elements)
        for element in other_set.elements:
            new_set.elements.pop(element, None)
        return new_set

    def is_subset_of(self, other_set):
        return all(element in other_set.elements for element in self.elements)

    def is_superset_of(self, other_set):
        return other_set.is_subset_of(self)

    def is_disjoint_with(self, other_set):
        return all(element not in other_set.elements for element in self.elements)

    def for_each(self, closure):
        for element in self.elements:
            closure(element)

    def remove_all(self):
        self.elements.clear()

    def remove_all_occurrences(self, element):
        self.elements.pop(element, None)

    def symmetric_difference(self, other_set):
        diff1 = self.difference(other_set)
        diff2 = other_set.difference(self)
        return diff1.union(diff2)

    def form_union(self, other_set):
        for element in other_set.elements:
            self.elements[element] = True

    def form_intersection(self, other_set):
        self.elements = {element: True for element in self.intersection(other_set).elements}

    def subtract(self, other_set):
        for element in other_set.elements:
            self.elements.pop(element, None)

    # ... Add more set operations as needed ...


# Example usage
if __name__ == "__main__":
    set1 = CustomSet()
    set1.insert(1)
    set1.insert(2)
    set1.insert(3)

    set2 = CustomSet()
    set2.insert(2)
    set2.insert(3)
    set2.insert(4)

    union_set = set1.union(set2)
    print("Union set:", list(union_set.elements.keys()))

    intersection_set = set1.intersection(set2)
    print("Intersection set:", list(intersection_set.elements.keys()))
