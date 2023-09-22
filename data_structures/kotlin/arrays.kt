class CustomArray<T>(initialSize: Int) {
    // Internal storage for elements
    private val elements = MutableList(initialSize) { null }

    // Append an element to the end of the custom array
    fun append(element: T?) {
        elements.add(element)
    }

    // Insert an element at the specified index
    fun insert(element: T?, index: Int) {
        elements.add(index, element)
    }

    // Remove the first occurrence of the specified element
    fun remove(element: T) {
        elements.remove(element)
    }

    // Remove and return the element at the specified index
    fun pop(index: Int): T? {
        return if (index >= 0 && index < elements.size) {
            elements.removeAt(index)
        } else {
            null
        }
    }

    // Get the element at the specified index
    fun get(index: Int): T? {
        return if (index >= 0 && index < elements.size) {
            elements[index]
        } else {
            null
        }
    }

    // Set the element at the specified index to the given element
    fun set(element: T?, index: Int) {
        if (index >= 0 && index < elements.size) {
            elements[index] = element
        }
    }

    // Return the current size of the custom array
    fun size(): Int {
        return elements.size
    }

    // Check if the custom array is empty
    fun isEmpty(): Boolean {
        return elements.isEmpty()
    }

    // Return the index of the first occurrence of the specified element, or -1 if not found
    fun index(element: T): Int {
        return elements.indexOf(element)
    }

    // Return the number of occurrences of the specified element in the custom array
    fun count(element: T): Int {
        return elements.count { it == element }
    }

    // Reverse the order of elements in the custom array
    fun reverse() {
        elements.reverse()
    }

    // Sort the elements in the custom array in ascending order
    fun sort() {
        elements.sortBy { it?.toString() }
    }

    // Return a new list containing elements from the specified start index to the end index
    fun slice(start: Int, end: Int): List<T> {
        return if (start >= 0 && end < elements.size && start <= end) {
            elements.subList(start, end + 1).toList()
        } else {
            emptyList()
        }
    }
}

fun main() {
    val customArray = CustomArray<Int?>(5)
    customArray.append(10)
    customArray.append(20)
    customArray.insert(15, 1)

    println("Size of array: ${customArray.size()}")

    // Print each element
    customArray.forEach { println(it) }
}
