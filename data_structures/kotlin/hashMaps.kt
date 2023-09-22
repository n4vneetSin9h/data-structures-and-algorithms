data class KeyValue<T : Any, U>(val key: T, var value: U)

class HashMap<T : Any, U>(capacity: Int) {
    private val buckets: Array<MutableList<KeyValue<T, U>>>
    private val capacity: Int

    init {
        this.capacity = maxOf(1, capacity)
        buckets = Array(this.capacity) { mutableListOf() }
    }

    // Insertion
    fun setValue(value: U, key: T) {
        val index = bucketIndex(key)
        for ((i, kv) in buckets[index].withIndex()) {
            if (kv.key == key) {
                buckets[index][i].value = value
                return
            }
        }
        buckets[index].add(KeyValue(key, value))
    }

    // Retrieval
    fun getValue(key: T): U? {
        val index = bucketIndex(key)
        for (kv in buckets[index]) {
            if (kv.key == key) {
                return kv.value
            }
        }
        return null
    }

    // Removal
    fun removeValue(key: T) {
        val index = bucketIndex(key)
        buckets[index].removeIf { it.key == key }
    }

    // Count
    fun count(): Int {
        return buckets.sumBy { it.size }
    }

    // Keys and Values
    fun allKeys(): List<T> {
        return buckets.flatMap { it.map { kv -> kv.key } }
    }

    fun allValues(): List<U> {
        return buckets.flatMap { it.map { kv -> kv.value } }
    }

    // Helper Functions
    fun isEmpty(): Boolean {
        return buckets.all { it.isEmpty() }
    }

    fun removeAll() {
        buckets.forEach { it.clear() }
    }

    fun contains(key: T): Boolean {
        val index = bucketIndex(key)
        return buckets[index].any { it.key == key }
    }

    fun loadFactor(): Double {
        return count().toDouble() / capacity.toDouble()
    }

    fun keyValuePairs(): Set<KeyValue<T, U>> {
        return buckets.flatMapTo(mutableSetOf()) { it }
    }

    // Update Values
    fun updateValue(value: U, key: T) {
        val index = bucketIndex(key)
        for ((i, kv) in buckets[index].withIndex()) {
            if (kv.key == key) {
                buckets[index][i].value = value
                return
            }
        }
        buckets[index].add(KeyValue(key, value))
    }

    fun updateValues(dictionary: Map<T, U>) {
        for ((key, value) in dictionary) {
            updateValue(value, key)
        }
    }

    // ... Add more hashmap operations as needed ...

    private fun bucketIndex(key: T): Int {
        // Simplified hash function for demonstration purposes
        return key.hashCode() % capacity
    }
}

fun main() {
    val hashmap = HashMap<String, Int>(10)
    hashmap.setValue(5, "five")
    hashmap.setValue(10, "ten")

    println("Value for key 'five': ${hashmap.getValue("five")}")
    println("Value for key 'ten': ${hashmap.getValue("ten")}")

    hashmap.removeValue("five")
    println("Contains key 'five': ${hashmap.contains("five")}")
    println("Total count: ${hashmap.count()}")

    val keys = hashmap.allKeys()
    println("All keys: $keys")

    val values = hashmap.allValues()
    println("All values: $values")

    println("Is empty: ${hashmap.isEmpty()}")
    println("Load factor: ${hashmap.loadFactor()}")
}
