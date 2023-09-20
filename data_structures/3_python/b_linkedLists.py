class Node:
    def __init__(self, value):
        self.value = value
        self.next = None


class LinkedList:
    def __init__(self):
        self.head = None
        self.tail = None

    # Check if the linked list is empty.
    def is_empty(self):
        return self.head is None

    # Get the first node in the linked list.
    def first(self):
        return self.head

    # Get the last node in the linked list.
    def last(self):
        return self.tail

    # Append a value to the end of the linked list.
    def append(self, value):
        new_node = Node(value)

        if self.tail:
            self.tail.next = new_node
        else:
            self.head = new_node

        self.tail = new_node

    # Prepend a value to the beginning of the linked list.
    def prepend(self, value):
        new_node = Node(value)

        if self.head:
            new_node.next = self.head
        else:
            self.tail = new_node

        self.head = new_node

    # Check if a value exists in the linked list.
    def contains(self, value):
        current = self.head

        while current:
            if current.value == value:
                return True
            current = current.next

        return False

    # Get the node at a specific index in the linked list.
    def node_at_index(self, index):
        current = self.head
        current_index = 0

        while current and current_index < index:
            current = current.next
            current_index += 1

        return current

    # Insert a value at a specific index in the linked list.
    def insert(self, value, index):
        if index < 0:
            return

        new_node = Node(value)

        if index == 0:
            self.prepend(value)
            return

        prev = self.node_at_index(index - 1)
        next_node = prev.next if prev else None

        if prev:
            prev.next = new_node

        if next_node:
            new_node.next = next_node

        if not next_node:
            self.tail = new_node

    # Remove the node at a specific index in the linked list.
    def remove(self, index):
        if index < 0:
            return

        if index == 0:
            if self.head:
                self.head = self.head.next
                if not self.head:
                    self.tail = None
            return

        prev = self.node_at_index(index - 1)
        if prev and prev.next:
            prev.next = prev.next.next

        if prev and not prev.next:
            self.tail = prev

    # Print the linked list.
    def print_list(self):
        current = self.head

        while current:
            print(f"{current.value} -> ", end="")
            current = current.next

        print("None")

    # Reverse the linked list.
    def reverse(self):
        prev = None
        current = self.head

        while current:
            next_node = current.next
            current.next = prev
            prev = current
            current = next_node

        self.tail, self.head = self.head, prev

    # Return the number of nodes in the linked list.
    def count(self):
        count = 0
        current = self.head

        while current:
            count += 1
            current = current.next

        return count

    # Remove all nodes from the linked list.
    def remove_all(self):
        self.head = None
        self.tail = None

    # Return a list representation of the linked list.
    def to_list(self):
        result = []
        current = self.head

        while current:
            result.append(current.value)
            current = current.next

        return result

    # Remove all occurrences of a value from the linked list.
    def remove_all_occurrences(self, value):
        current = self.head
        prev = None

        while current:
            if current.value == value:
                if prev:
                    prev.next = current.next
                    current = current.next
                else:
                    self.head = self.head.next
                    current = self.head
            else:
                prev = current
                current = current.next

        if not self.head:
            self.tail = None


# Test the LinkedList class
if __name__ == "__main__":
    linked_list = LinkedList()
    linked_list.append(1)
    linked_list.append(2)
    linked_list.append(3)

    print("Linked List:")
    linked_list.print_list()
