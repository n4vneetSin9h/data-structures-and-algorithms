import java.util.*

class Heap<T>(private val isMinHeap: Boolean = false, private val comparator: Comparator<T>) {
    private val elements: MutableList<T> = mutableListOf()

    // MARK: - Insertion

    fun insert(element: T) {
        elements.add(element)
        heapifyUp()
    }

    private fun heapifyUp() {
        var currentIndex = elements.size - 1
        while (currentIndex > 0) {
            val parentIndex = (currentIndex - 1) / 2

            if (shouldSwap(parentIndex, currentIndex)) {
                Collections.swap(elements, parentIndex, currentIndex)
                currentIndex = parentIndex
            } else {
                break
            }
        }
    }

    // MARK: - Removal

    fun remove(): T? {
        if (elements.isEmpty()) {
            return null
        }

        if (elements.size == 1) {
            return elements.removeAt(0)
        }

        val root = elements[0]
        elements[0] = elements.removeAt(elements.size - 1)
        heapifyDown()

        return root
    }

    private fun heapifyDown() {
        var currentIndex = 0

        while (hasLeftChild(currentIndex)) {
            var smallestChildIndex = leftChildIndex(currentIndex)

            if (hasRightChild(currentIndex) && shouldSwap(rightChildIndex(currentIndex), smallestChildIndex)) {
                smallestChildIndex = rightChildIndex(currentIndex)
            }

            if (shouldSwap(smallestChildIndex, currentIndex)) {
                Collections.swap(elements, smallestChildIndex, currentIndex)
                currentIndex = smallestChildIndex
            } else {
                break
            }
        }
    }

    // MARK: - Peek

    fun peek(): T? {
        return elements.firstOrNull()
    }

    // MARK: - Helper Methods

    private fun leftChildIndex(parentIndex: Int): Int {
        return 2 * parentIndex + 1
    }

    private fun rightChildIndex(parentIndex: Int): Int {
        return 2 * parentIndex + 2
    }

    private fun hasLeftChild(index: Int): Boolean {
        return leftChildIndex(index) < elements.size
    }

    private fun hasRightChild(index: Int): Boolean {
        return rightChildIndex(index) < elements.size
    }

    private fun shouldSwap(firstIndex: Int, secondIndex: Int): Boolean {
        return if (isMinHeap) {
            comparator.compare(elements[firstIndex], elements[secondIndex]) > 0
        } else {
            comparator.compare(elements[firstIndex], elements[secondIndex]) < 0
        }
    }

    // MARK: - Build Heap

    fun buildHeap(elements: List<T>) {
        this.elements.clear()
        this.elements.addAll(elements)
        for (i in (elements.size / 2) - 1 downTo 0) {
            heapifyDown(i)
        }
    }

    // MARK: - Replace Root

    fun replaceRoot(element: T) {
        if (elements.isEmpty()) {
            insert(element)
            return
        }

        elements[0] = element
        heapifyDown(0)
    }

    // MARK: - Remove at Index

    fun removeAtIndex(index: Int): T? {
        if (index < 0 || index >= elements.size) {
            return null
        }

        if (index == elements.size - 1) {
            return elements.removeAt(elements.size - 1)
        }

        Collections.swap(elements, index, elements.size - 1)
        val removedElement = elements.removeAt(elements.size - 1)
        heapifyDown(index)

        return removedElement
    }

    // MARK: - Sort

    fun sort(): List<T> {
        val sortedElements = mutableListOf<T>()
        while (elements.isNotEmpty()) {
            sortedElements.add(remove()!!)
        }
        return sortedElements
    }

    // MARK: - Index for Element

    fun index(element: T): Int? {
        return index(element, 0)
    }

    private fun index(element: T, startIndex: Int): Int? {
        if (startIndex < elements.size) {
            if (elements[startIndex] == element) {
                return startIndex
            }

            leftChildIndex(startIndex).let {
                if (it < elements.size) {
                    val result = index(element, it)
                    if (result != null) return result
                }
            }

            rightChildIndex(startIndex).let {
                if (it < elements.size) {
                    val result = index(element, it)
                    if (result != null) return result
                }
            }
        }

        return null
    }

    // ... Add more heap operations as needed ...
}

fun main() {
    // Example usage
    val maxHeap = Heap(isMinHeap = false, comparator = compareBy { it })
    maxHeap.insert(3)
    maxHeap.insert(1)
    maxHeap.insert(4)
    maxHeap.insert(1)
    maxHeap.insert(5)
    maxHeap.insert(9)

    println("Max Heap: ${maxHeap.elements}")

    val minHeap = Heap(isMinHeap = true, comparator = compareBy { it })
    minHeap.insert(3)
    minHeap.insert(1)
    minHeap.insert(4)
    minHeap.insert(1)
    minHeap.insert(5)
    minHeap.insert(9)

    println("Min Heap: ${minHeap.elements}")
}
