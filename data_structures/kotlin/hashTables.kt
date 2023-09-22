class HashTable<Key, Value>(private val capacity: Int) {

    private val buckets: ArrayList<ArrayList<Pair<Key, Value>>> = ArrayList(capacity)

    init {
        require(capacity > 0) { "Capacity should be greater than 0" }

        // Initialize the buckets
        repeat(capacity) { buckets.add(ArrayList()) }
    }

    private fun bucketIndex(key: Key): Int {
        val hash = key.hashCode()
        return hash.rem(capacity)
    }

    fun getValue(key: Key): Value? {
        val index = bucketIndex(key)
        for (pair in buckets[index]) {
            if (pair.first == key) {
                return pair.second
            }
        }
        return null
    }

    fun setValue(key: Key, value: Value) {
        val index = bucketIndex(key)

        // Check if the key already exists, and update the value
        for (i in buckets[index].indices) {
            if (buckets[index][i].first == key) {
                buckets[index][i] = Pair(key, value)
                return
            }
        }

        // Key doesn't exist, add a new entry
        buckets[index].add(Pair(key, value))
    }

    fun removeValue(key: Key) {
        val index = bucketIndex(key)
        buckets[index].removeIf { it.first == key }
    }

    fun removeAll() {
        for (bucket in buckets) {
            bucket.clear()
        }
    }

    fun count(): Int {
        return buckets.sumBy { it.size }
    }

    fun contains(key: Key): Boolean {
        val index = bucketIndex(key)
        return buckets[index].any { it.first == key }
    }

    fun allKeys(): List<Key> {
        val keys = ArrayList<Key>()
        for (bucket in buckets) {
            for (pair in bucket) {
                keys.add(pair.first)
            }
        }
        return keys
    }

    fun allValues(): List<Value> {
        val values = ArrayList<Value>()
        for (bucket in buckets) {
            for (pair in bucket) {
                values.add(pair.second)
            }
        }
        return values
    }

    fun merge(otherTable: HashTable<Key, Value>) {
        for (bucket in otherTable.buckets) {
            for ((key, value) in bucket) {
                setValue(key, value)
            }
        }
    }

    fun resize(newCapacity: Int) {
        require(newCapacity > 0) { "New capacity should be greater than 0" }

        val newBuckets: ArrayList<ArrayList<Pair<Key, Value>>> = ArrayList(newCapacity)

        repeat(newCapacity) { newBuckets.add(ArrayList()) }

        for (bucket in buckets) {
            for ((key, value) in bucket) {
                val newIndex = bucketIndex(key)
                newBuckets[newIndex].add(Pair(key, value))
            }
        }

        buckets.clear()
        buckets.addAll(newBuckets)
    }

    // ... Add more hash table operations as needed ...
}

fun main() {
    val table = HashTable<Int, String>(10)

    table.setValue(1, "Value1")
    table.setValue(2, "Value2")
    table.setValue(3, "Value3")

    println("Value for key 2: ${table.getValue(2)}")

    table.removeValue(2)
    println("Count after removal: ${table.count()}")

    println("Keys in the table: ${table.allKeys()}")
}
