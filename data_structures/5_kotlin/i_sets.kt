class CustomSet<T : Any> {
    private val elements: MutableMap<T, Boolean> = mutableMapOf()  // HashMap to mimic a set

    // Basic Set Operations

    /**
     * Check if the set is empty.
     */
    fun isEmpty(): Boolean {
        return elements.isEmpty()
    }

    /**
     * Get the number of elements in the set.
     */
    fun count(): Int {
        return elements.size
    }

    /**
     * Insert an element into the set.
     */
    fun insert(element: T) {
        elements[element] = true
    }

    /**
     * Remove an element from the set.
     * @return The removed element if it existed in the set, null otherwise.
     */
    fun remove(element: T): T? {
        return if (elements.remove(element) != null) element else null
    }

    /**
     * Check if the set contains a specific element.
     */
    fun contains(element: T): Boolean {
        return elements.containsKey(element)
    }

    // Set Operations

    /**
     * Perform the union operation with another set and return a new set.
     */
    fun union(otherSet: CustomSet<T>): CustomSet<T> {
        val newSet = CustomSet<T>()
        newSet.elements.putAll(elements)
        newSet.elements.putAll(otherSet.elements)
        return newSet
    }

    /**
     * Perform the intersection operation with another set and return a new set.
     */
    fun intersection(otherSet: CustomSet<T>): CustomSet<T> {
        val newSet = CustomSet<T>()
        for (element in elements.keys) {
            if (otherSet.contains(element)) {
                newSet.insert(element)
            }
        }
        return newSet
    }

    /**
     * Perform the difference operation with another set and return a new set.
     */
    fun difference(otherSet: CustomSet<T>): CustomSet<T> {
        val newSet = CustomSet<T>()
        newSet.elements.putAll(elements)
        for (element in otherSet.elements.keys) {
            newSet.elements.remove(element)
        }
        return newSet
    }

    /**
     * Check if this set is a subset of another set.
     */
    fun isSubsetOf(otherSet: CustomSet<T>): Boolean {
        return elements.keys.all { otherSet.contains(it) }
    }

    /**
     * Check if this set is a superset of another set.
     */
    fun isSupersetOf(otherSet: CustomSet<T>): Boolean {
        return otherSet.isSubsetOf(this)
    }

    /**
     * Check if this set is disjoint with another set.
     */
    fun isDisjointWith(otherSet: CustomSet<T>): Boolean {
        return elements.keys.all { !otherSet.contains(it) }
    }

    /**
     * Apply a closure to each element in the set.
     */
    fun forEach(closure: (T) -> Unit) {
        elements.keys.forEach { closure(it) }
    }

    /**
     * Remove all elements from the set.
     */
    fun removeAll() {
        elements.clear()
    }

    /**
     * Remove all occurrences of an element from the set.
     */
    fun removeAllOccurrences(element: T) {
        elements.remove(element)
    }

    /**
     * Get the set with common elements between this set and another set.
     */
    fun symmetricDifference(otherSet: CustomSet<T>): CustomSet<T> {
        val diff1 = this.difference(otherSet)
        val diff2 = otherSet.difference(this)
        return diff1.union(diff2)
    }

    /**
     * Insert elements from another set into this set.
     */
    fun formUnion(otherSet: CustomSet<T>) {
        elements.putAll(otherSet.elements)
    }

    /**
     * Remove elements of this set that are not in another set.
     */
    fun formIntersection(otherSet: CustomSet<T>) {
        elements.keys.retainAll { otherSet.contains(it) }
    }

    /**
     * Remove elements of this set that are in another set.
     */
    fun subtract(otherSet: CustomSet<T>) {
        elements.keys.removeAll { otherSet.contains(it) }
    }

    // ... Add more set operations as needed ...
}

// Example usage
fun main() {
    val set1 = CustomSet<Int>()
    set1.insert(1)
    set1.insert(2)
    set1.insert(3)

    val set2 = CustomSet<Int>()
    set2.insert(2)
    set2.insert(3)
    set2.insert(4)

    val unionSet = set1.union(set2)
    println("Union set: ${unionSet.elements.keys}")

    val intersectionSet = set1.intersection(set2)
    println("Intersection set: ${intersectionSet.elements.keys}")
}
