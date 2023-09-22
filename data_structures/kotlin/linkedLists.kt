class Node<T>(val value: T) {
    var next: Node<T>? = null
}

class LinkedList<T> {
    private var head: Node<T>? = null
    private var tail: Node<T>? = null

    // Check if the linked list is empty.
    fun isEmpty(): Boolean {
        return head == null
    }

    // Get the first node in the linked list.
    fun first(): Node<T>? {
        return head
    }

    // Get the last node in the linked list.
    fun last(): Node<T>? {
        return tail
    }

    // Append a value to the end of the linked list.
    fun append(value: T) {
        val newNode = Node(value)

        if (tail != null) {
            tail?.next = newNode
        } else {
            head = newNode
        }

        tail = newNode
    }

    // Prepend a value to the beginning of the linked list.
    fun prepend(value: T) {
        val newNode = Node(value)

        if (head != null) {
            newNode.next = head
        } else {
            tail = newNode
        }

        head = newNode
    }

    // Check if a value exists in the linked list.
    fun contains(value: T): Boolean {
        var current = head

        while (current != null) {
            if (current.value == value) {
                return true
            }
            current = current.next
        }

        return false
    }

    // Get the node at a specific index in the linked list.
    fun nodeAtIndex(index: Int): Node<T>? {
        if (index < 0) {
            return null
        }

        var current = head
        var currentIndex = 0

        while (current != null && currentIndex < index) {
            current = current.next
            currentIndex++
        }

        return current
    }

    // Insert a value at a specific index in the linked list.
    fun insert(value: T, index: Int) {
        if (index < 0) {
            return
        }

        val newNode = Node(value)

        if (index == 0) {
            prepend(value)
            return
        }

        val prev = nodeAtIndex(index - 1)
        val next = prev?.next

        prev?.next = newNode
        newNode.next = next
    }

    // Remove the node at a specific index in the linked list.
    fun remove(index: Int) {
        if (index < 0) {
            return
        }

        if (index == 0) {
            head = head?.next
            if (head == null) {
                tail = null
            }
            return
        }

        val prev = nodeAtIndex(index - 1)
        prev?.next = prev?.next?.next

        if (prev?.next == null) {
            tail = prev
        }
    }

    // Print the linked list.
    fun printList() {
        var current = head

        while (current != null) {
            print("${current.value} -> ")
            current = current.next
        }
        println("null")
    }

    // Reverse the linked list.
    fun reverse() {
        var prev: Node<T>? = null
        var current = head

        while (current != null) {
            val next = current.next
            current.next = prev
            prev = current
            current = next
        }

        tail = head
        head = prev
    }

    // Return the number of nodes in the linked list.
    fun count(): Int {
        var count = 0
        var current = head

        while (current != null) {
            count++
            current = current.next
        }

        return count
    }

    // Remove all nodes from the linked list.
    fun removeAll() {
        head = null
        tail = null
    }

    // Return a list representation of the linked list.
    fun toList(): List<T> {
        val list = mutableListOf<T>()
        var current = head

        while (current != null) {
            list.add(current.value)
            current = current.next
        }

        return list
    }

    // Remove all occurrences of a value from the linked list.
    fun removeAllOccurrences(value: T) {
        var current = head
        var prev: Node<T>? = null

        while (current != null) {
            if (current.value == value) {
                if (prev != null) {
                    prev.next = current.next
                    current = prev.next
                } else {
                    head = head?.next
                    current = head
                }
            } else {
                prev = current
                current = current.next
            }
        }

        if (head == null) {
            tail = null
        }
    }
}

fun main() {
    val list = LinkedList<Int>()
    list.append(1)
    list.append(2)
    list.append(3)

    println("Linked List:")
    list.printList()
}
