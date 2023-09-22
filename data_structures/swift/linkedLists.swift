class Node<T> {
    var value: T
    var next: Node?
    
    init(value: T) {
        self.value = value
    }
}

class LinkedList<T> {
    private var head: Node<T>? // Head of the linked list
    private var tail: Node<T>? // Tail of the linked list
    
    // MARK: - Basic Operations
    
    /// Check if the linked list is empty.
    func isEmpty() -> Bool {
        return head == nil
    }
    
    /// Get the first node in the linked list.
    func first() -> Node<T>? {
        return head
    }
    
    /// Get the last node in the linked list.
    func last() -> Node<T>? {
        return tail
    }
    
    /// Append a value to the end of the linked list.
    func append(value: T) {
        let newNode = Node(value: value)
        
        if let tailNode = tail {
            tailNode.next = newNode
        } else {
            head = newNode
        }
        
        tail = newNode
    }
    
    /// Prepend a value to the beginning of the linked list.
    func prepend(value: T) {
        let newNode = Node(value: value)
        
        if let headNode = head {
            newNode.next = headNode
        } else {
            tail = newNode
        }
        
        head = newNode
    }
    
    // MARK: - Search
    
    /// Check if a value exists in the linked list.
    func contains(value: T) -> Bool {
        var current = head
        
        while current != nil {
            if current?.value == value {
                return true
            }
            current = current?.next
        }
        
        return false
    }
    
    /// Get the node at a specific index in the linked list.
    func node(atIndex index: Int) -> Node<T>? {
        guard index >= 0 else { return nil }
        
        var current = head
        var currentIndex = 0
        
        while current != nil && currentIndex < index {
            current = current?.next
            currentIndex += 1
        }
        
        return current
    }
    
    // MARK: - Insertion and Deletion
    
    /// Insert a value at a specific index in the linked list.
    func insert(value: T, at index: Int) {
        guard index >= 0 else { return }
        
        let newNode = Node(value: value)
        
        if index == 0 {
            prepend(value: value)
            return
        }
        
        let prev = node(atIndex: index - 1)
        let next = prev?.next
        
        prev?.next = newNode
        newNode.next = next
    }
    
    /// Remove the node at a specific index in the linked list.
    func remove(at index: Int) {
        guard index >= 0 else { return }
        
        if index == 0 {
            head = head?.next
            if head == nil {
                tail = nil
            }
            return
        }
        
        let prev = node(atIndex: index - 1)
        prev?.next = prev?.next?.next
        
        if prev?.next == nil {
            tail = prev
        }
    }
    
    // MARK: - Display
    
    /// Print the linked list.
    func printList() {
        var current = head
        while current != nil {
            print("\(current!.value)", terminator: " -> ")
            current = current?.next
        }
        print("nil")
    }
    
    // MARK: - Other Methods
    
    /// Reverse the linked list.
    func reverse() {
        var prev: Node<T>? = nil
        var current = head
        
        while current != nil {
            let next = current?.next
            current?.next = prev
            prev = current
            current = next
        }
        
        tail = head
        head = prev
    }
    
    /// Return the number of nodes in the linked list.
    func count() -> Int {
        var count = 0
        var current = head
        
        while current != nil {
            count += 1
            current = current?.next
        }
        
        return count
    }
    
    /// Remove all nodes from the linked list.
    func removeAll() {
        head = nil
        tail = nil
    }
    
    /// Return an array representation of the linked list.
    func toArray() -> [T] {
        var array = [T]()
        var current = head
        
        while current != nil {
            array.append(current!.value)
            current = current?.next
        }
        
        return array
    }
    
    /// Remove all occurrences of a value from the linked list.
    func removeAllOccurrences(of value: T) {
        var current = head
        var prev: Node<T>? = nil
        
        while current != nil {
            if current?.value == value {
                if prev != nil {
                    prev?.next = current?.next
                    current = prev?.next
                } else {
                    head = head?.next
                    current = head
                }
            } else {
                prev = current
                current = current?.next
            }
        }
        
        if head == nil {
            tail = nil
        }
    }

    /// - Parameter closure: A closure that takes an element of the linked list as a parameter.
    func forEach(_ closure: (T) -> Void) {
        var current = head

        while let node = current {
            closure(node.value)
            current = node.next
        }
    }
}
