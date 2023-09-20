class DisjointSets:
    def __init__(self):
        self.parent = {}
        self.rank = {}

    def make_set(self, element):
        if element not in self.parent:
            self.parent[element] = element
            self.rank[element] = 0

    def find(self, element):
        if self.parent[element] == element:
            return element

        self.parent[element] = self.find(self.parent[element])  # Path compression
        return self.parent[element]

    def union(self, element1, element2):
        parent1 = self.find(element1)
        parent2 = self.find(element2)

        if parent1 != parent2:
            rank1 = self.rank.get(parent1, 0)
            rank2 = self.rank.get(parent2, 0)

            if rank1 > rank2:
                self.parent[parent2] = parent1
            elif rank1 < rank2:
                self.parent[parent1] = parent2
            else:
                self.parent[parent2] = parent1
                self.rank[parent1] = rank1 + 1

    def contains(self, element):
        return element in self.parent

    def set_size(self, element):
        representative = self.find(element)
        return sum(1 for val in self.parent.values() if self.find(val) == representative)

    def elements_in_same_set(self, element):
        representative = self.find(element)
        return [key for key, val in self.parent.items() if self.find(val) == representative]

    def all_sets(self):
        sets = {}
        set_representatives = set()

        for element in self.parent:
            representative = self.find(element)
            if representative not in set_representatives:
                set_elements = [key for key, val in self.parent.items() if self.find(val) == representative]
                sets[representative] = set_elements
                set_representatives.add(representative)

        return list(sets.values())

    def reset(self):
        self.parent = {}
        self.rank = {}

    def is_same_set(self, element1, element2):
        parent1 = self.find(element1)
        parent2 = self.find(element2)
        return parent1 == parent2

    def remove_set(self, element):
        representative = self.find(element)
        self.parent = {key: val for key, val in self.parent.items() if self.find(val) != representative}
        self.rank = {key: val for key, val in self.rank.items() if self.find(val) != representative}

    def path_to_root(self, element):
        path = []
        current_node = element
        representative = self.find(element)

        while current_node != representative:
            path.append(current_node)
            parent = self.find(current_node)
            current_node = parent

        path.append(representative)
        return path

    def get_representatives(self):
        representatives_set = set()
        representatives = []

        for element in self.parent:
            representative = self.find(element)
            if representative not in representatives_set:
                representatives.append(representative)
                representatives_set.add(representative)

        return representatives

    def find_and_compress(self, element):
        representative = self.find(element)
        self.compress_path(element, representative)
        return representative

    def compress_path(self, element, representative):
        current_node = element

        while current_node != representative:
            parent = self.find(current_node)
            self.parent[current_node] = representative
            current_node = parent

    def print_sets(self):
        sets = {}

        for element in self.parent:
            representative = self.find(element)
            if representative not in sets:
                sets[representative] = []

            sets[representative].append(element)

        for representative, elements in sets.items():
            print(f"Set with representative {representative}: {elements}")


# Example usage
disjoint_sets = DisjointSets()

disjoint_sets.make_set(1)
disjoint_sets.make_set(2)
disjoint_sets.make_set(3)
disjoint_sets.make_set(4)

disjoint_sets.union(1, 2)
disjoint_sets.union(3, 4)
disjoint_sets.union(2, 3)

print("Is 1 in the same set as 2:", disjoint_sets.is_same_set(1, 2))  # True
print("Is 1 in the same set as 3:", disjoint_sets.is_same_set(1, 3))  # True
print("Is 1 in the same set as 4:", disjoint_sets.is_same_set(1, 4))  # True
print("Is 2 in the same set as 4:", disjoint_sets.is_same_set(2, 4))  # True
print("Is 1 in the same set as 5:", disjoint_sets.is_same_set(1, 5))  # False

disjoint_sets.remove_set(3)

print("Is 1 in the same set as 2:", disjoint_sets.is_same_set(1, 2))  # True
print("Is 1 in the same set as 3:", disjoint_sets.is_same_set(1, 3))  # False
print("Is 1 in the same set as 4:", disjoint_sets.is_same_set(1, 4))  # True
print("Is 2 in the same set as 4:", disjoint_sets.is_same_set(2, 4))  # True

disjoint_sets.print_sets()
