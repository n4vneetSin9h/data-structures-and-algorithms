class CustomStack<T> {
    private val elements: MutableList<T> = mutableListOf()

    // MARK: - Stack Operations

    /// Check if the stack is empty.
    fun isEmpty(): Boolean {
        return elements.isEmpty()
    }

    /// Get the number of elements in the stack.
    fun count(): Int {
        return elements.size
    }

    /// Push an element onto the stack.
    fun push(element: T) {
        elements.add(element)
    }

    /// Pop the top element from the stack.
    fun pop(): T? {
        return if (elements.isNotEmpty()) elements.removeAt(elements.size - 1) else null
    }

    /// Peek at the top element in the stack without removing it.
    fun peek(): T? {
        return elements.lastOrNull()
    }

    // MARK: - Stack Operations with List

    /// Initialize the stack with a list of elements.
    fun initializeWithList(list: List<T>) {
        elements.clear()
        elements.addAll(list)
    }

    /// Push a list of elements onto the stack.
    fun pushList(list: List<T>) {
        elements.addAll(list)
    }

    /// Pop a list of elements from the stack.
    fun popList(count: Int): List<T> {
        val poppedElements = mutableListOf<T>()
        repeat(count) {
            if (elements.isNotEmpty()) {
                poppedElements.add(elements.removeAt(elements.size - 1))
            }
        }
        return poppedElements
    }

    // MARK: - Clear and Reset

    /// Remove all elements from the stack.
    fun clear() {
        elements.clear()
    }

    // MARK: - Checking for Element

    /// Check if the stack contains a specific element.
    fun contains(element: T): Boolean {
        return elements.contains(element)
    }

    // MARK: - Filtering

    /// Filter the stack using a given predicate.
    fun filter(predicate: (T) -> Boolean): List<T> {
        return elements.filter(predicate)
    }

    // MARK: - Conversion to List

    /// Convert the stack to a list.
    fun toList(): List<T> {
        return elements.toList()
    }

    // MARK: - Map

    /// Transform each element in the stack using a provided transform function.
    fun <U> map(transform: (T) -> U): List<U> {
        return elements.map(transform)
    }

    // MARK: - Reduce

    /// Combine all elements in the stack using a reducer function.
    fun <Result> reduce(initialResult: Result, operation: (Result, T) -> Result): Result {
        return elements.fold(initialResult, operation)
    }

    // MARK: - Concatenate

    /// Concatenate another stack to this stack.
    fun concatenate(otherStack: CustomStack<T>) {
        elements.addAll(otherStack.elements)
    }

    // MARK: - Subscript

    /// Access the element at a specific index in the stack.
    operator fun get(index: Int): T? {
        return if (index in 0 until elements.size) elements[index] else null
    }

    // ... Add more stack operations as needed ...
}

fun main() {
    val stack = CustomStack<Int>()
    stack.push(1)
    stack.push(2)
    stack.push(3)

    println("Stack:")
    stack.toList().forEach { println(it) }
}
