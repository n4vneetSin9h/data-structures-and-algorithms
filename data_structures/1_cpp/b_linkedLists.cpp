#include <iostream>
#include <vector>

template <typename T>
class Node {
public:
    T value;
    Node* next;
    
    Node(T val) : value(val), next(nullptr) {}
};

template <typename T>
class LinkedList {
private:
    Node<T>* head;
    Node<T>* tail;

public:
    LinkedList() : head(nullptr), tail(nullptr) {}

    // Check if the linked list is empty.
    bool isEmpty() const {
        return head == nullptr;
    }

    // Get the first node in the linked list.
    Node<T>* first() const {
        return head;
    }

    // Get the last node in the linked list.
    Node<T>* last() const {
        return tail;
    }

    // Append a value to the end of the linked list.
    void append(T value) {
        Node<T>* newNode = new Node<T>(value);

        if (tail != nullptr) {
            tail->next = newNode;
        } else {
            head = newNode;
        }

        tail = newNode;
    }

    // Prepend a value to the beginning of the linked list.
    void prepend(T value) {
        Node<T>* newNode = new Node<T>(value);

        if (head != nullptr) {
            newNode->next = head;
        } else {
            tail = newNode;
        }

        head = newNode;
    }

    // Check if a value exists in the linked list.
    bool contains(T value) const {
        Node<T>* current = head;

        while (current != nullptr) {
            if (current->value == value) {
                return true;
            }
            current = current->next;
        }

        return false;
    }

    // Get the node at a specific index in the linked list.
    Node<T>* nodeAtIndex(int index) const {
        if (index < 0)
            return nullptr;

        Node<T>* current = head;
        int currentIndex = 0;

        while (current != nullptr && currentIndex < index) {
            current = current->next;
            currentIndex++;
        }

        return current;
    }

    // Insert a value at a specific index in the linked list.
    void insert(T value, int index) {
        if (index < 0)
            return;

        Node<T>* newNode = new Node<T>(value);

        if (index == 0) {
            prepend(value);
            return;
        }

        Node<T>* prev = nodeAtIndex(index - 1);
        Node<T>* next = (prev != nullptr) ? prev->next : nullptr;

        if (prev != nullptr)
            prev->next = newNode;

        if (newNode != nullptr)
            newNode->next = next;
    }

    // Remove the node at a specific index in the linked list.
    void remove(int index) {
        if (index < 0)
            return;

        if (index == 0) {
            Node<T>* temp = head;
            head = head->next;
            delete temp;

            if (head == nullptr)
                tail = nullptr;

            return;
        }

        Node<T>* prev = nodeAtIndex(index - 1);
        if (prev == nullptr || prev->next == nullptr)
            return;

        Node<T>* toRemove = prev->next;
        prev->next = toRemove->next;
        delete toRemove;

        if (prev->next == nullptr)
            tail = prev;
    }

    // Print the linked list.
    void printList() const {
        Node<T>* current = head;

        while (current != nullptr) {
            std::cout << current->value << " -> ";
            current = current->next;
        }

        std::cout << "nullptr" << std::endl;
    }

    // Reverse the linked list.
    void reverse() {
        Node<T>* prev = nullptr;
        Node<T>* current = head;

        while (current != nullptr) {
            Node<T>* next = current->next;
            current->next = prev;
            prev = current;
            current = next;
        }

        tail = head;
        head = prev;
    }

    // Return the number of nodes in the linked list.
    int count() const {
        int count = 0;
        Node<T>* current = head;

        while (current != nullptr) {
            count++;
            current = current->next;
        }

        return count;
    }

    // Remove all nodes from the linked list.
    void removeAll() {
        Node<T>* current = head;

        while (current != nullptr) {
            Node<T>* next = current->next;
            delete current;
            current = next;
        }

        head = nullptr;
        tail = nullptr;
    }

    // Return a vector representation of the linked list.
    std::vector<T> toArray() const {
        std::vector<T> array;
        Node<T>* current = head;

        while (current != nullptr) {
            array.push_back(current->value);
            current = current->next;
        }

        return array;
    }

    // Remove all occurrences of a value from the linked list.
    void removeAllOccurrences(T value) {
        Node<T>* current = head;
        Node<T>* prev = nullptr;

        while (current != nullptr) {
            if (current->value == value) {
                if (prev != nullptr) {
                    prev->next = current->next;
                    delete current;
                    current = prev->next;
                } else {
                    Node<T>* temp = head;
                    head = head->next;
                    delete temp;
                    current = head;
                }
            } else {
                prev = current;
                current = current->next;
            }
        }

        if (head == nullptr)
            tail = nullptr;
    }

    // Perform an operation on each element of the linked list.
    void forEach(void (*operation)(T)) {
        Node<T>* current = head;

        while (current != nullptr) {
            operation(current->value);
            current = current->next;
        }
    }

    // Destructor to deallocate memory
    ~LinkedList() {
        Node<T>* current = head;
        Node<T>* next = nullptr;

        while (current != nullptr) {
            next = current->next;
            delete current;
            current = next;
        }
    }
};

int main() {
    LinkedList<int> list;
    list.append(1);
    list.append(2);
    list.append(3);

    std::cout << "Linked List: ";
    list.printList();

    return 0;
}
