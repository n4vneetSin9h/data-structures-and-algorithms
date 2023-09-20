class CustomQueue<T> {
    private val elements: MutableList<T> = mutableListOf()

    // MARK: - Queue Operations

    /// Enqueue an element to the back of the queue.
    fun enqueue(element: T) {
        elements.add(element)
    }

    /// Dequeue an element from the front of the queue.
    fun dequeue(): T? {
        return if (elements.isNotEmpty()) elements.removeAt(0) else null
    }

    /// Peek at the front element in the queue without dequeuing.
    fun peek(): T? {
        return elements.firstOrNull()
    }

    /// Check if the queue is empty.
    fun isEmpty(): Boolean {
        return elements.isEmpty()
    }

    /// Get the number of elements in the queue.
    fun count(): Int {
        return elements.size
    }

    // MARK: - Queue Operations with List

    /// Initialize the queue with a list of elements.
    fun initializeWithList(list: List<T>) {
        elements.clear()
        elements.addAll(list)
    }

    /// Enqueue a list of elements to the back of the queue.
    fun enqueueList(list: List<T>) {
        elements.addAll(list)
    }

    /// Dequeue a specified number of elements from the front of the queue.
    fun dequeueList(count: Int): List<T> {
        val dequeuedElements = mutableListOf<T>()
        repeat(count) {
            if (elements.isNotEmpty()) {
                dequeuedElements.add(elements.removeAt(0))
            }
        }
        return dequeuedElements
    }

    // MARK: - Clear and Reset

    /// Remove all elements from the queue.
    fun clear() {
        elements.clear()
    }

    // MARK: - Checking for Element

    /// Check if the queue contains a specific element.
    fun contains(element: T): Boolean {
        return elements.contains(element)
    }

    // MARK: - Filtering

    /// Filter the queue using a given predicate.
    fun filter(predicate: (T) -> Boolean): List<T> {
        return elements.filter(predicate)
    }

    // MARK: - Conversion to List

    /// Convert the queue to a list.
    fun toList(): List<T> {
        return elements.toList()
    }

    // MARK: - Map

    /// Transform each element in the queue using a provided transform function.
    fun <R> map(transform: (T) -> R): List<R> {
        return elements.map(transform)
    }

    // MARK: - Reduce

    /// Combine all elements in the queue using a reducer function.
    fun <R> reduce(initialResult: R, operation: (acc: R, T) -> R): R {
        return elements.fold(initialResult, operation)
    }

    // MARK: - Concatenate

    /// Concatenate another queue to this queue.
    fun concatenate(otherQueue: CustomQueue<T>) {
        elements.addAll(otherQueue.elements)
    }

    // MARK: - Subscript

    /// Access the element at a specific index in the queue.
    operator fun get(index: Int): T? {
        return elements.getOrNull(index)
    }

    // ... Add more queue operations as needed ...
}

fun main() {
    val queue = CustomQueue<Int>()
    queue.enqueue(1)
    queue.enqueue(2)
    queue.enqueue(3)

    println("Queue:")
    while (!queue.isEmpty()) {
        println(queue.dequeue())
    }
}
