class DisjointSets<T : Any> {
    private val parent = mutableMapOf<T, T>()
    private val rank = mutableMapOf<T, Int>()

    fun makeSet(element: T) {
        if (!parent.containsKey(element)) {
            parent[element] = element
            rank[element] = 0
        }
    }

    tailrec fun find(element: T): T {
        val parentElement = parent[element] ?: return element

        return if (parentElement == element) {
            parentElement
        } else {
            parent[element] = find(parentElement)
            parent[element]!!
        }
    }

    fun union(element1: T, element2: T) {
        val parent1 = find(element1)
        val parent2 = find(element2)

        if (parent1 != parent2) {
            val rank1 = rank[parent1] ?: 0
            val rank2 = rank[parent2] ?: 0

            if (rank1 > rank2) {
                parent[parent2] = parent1
            } else if (rank1 < rank2) {
                parent[parent1] = parent2
            } else {
                parent[parent2] = parent1
                rank[parent1] = rank1 + 1
            }
        }
    }

    fun contains(element: T): Boolean {
        return parent.containsKey(element)
    }

    fun setSize(element: T): Int {
        val representative = find(element)
        return parent.values.count { find(it) == representative }
    }

    fun elementsInSameSet(element: T): List<T> {
        val representative = find(element)
        return parent.keys.filter { find(it) == representative }
    }

    fun allSets(): List<List<T>> {
        val sets = mutableMapOf<T, MutableList<T>>()
        val setRepresentatives = mutableSetOf<T>()

        for (element in parent.keys) {
            val representative = find(element)
            if (representative !in setRepresentatives) {
                sets[representative] = mutableListOf()
                setRepresentatives.add(representative)
            }

            sets[representative]!!.add(element)
        }

        return sets.values.toList()
    }

    fun reset() {
        parent.clear()
        rank.clear()
    }

    fun isSameSet(element1: T, element2: T): Boolean {
        val parent1 = find(element1)
        val parent2 = find(element2)
        return parent1 == parent2
    }

    fun removeSet(element: T) {
        val representative = find(element)
        parent.entries.removeIf { find(it.value) == representative }
        rank.entries.removeIf { find(it.key) == representative }
    }

    fun pathToRoot(element: T): List<T> {
        val path = mutableListOf<T>()
        var currentNode = element
        val representative = find(element)

        while (currentNode != representative) {
            path.add(currentNode)
            val parent = parent[currentNode] ?: return emptyList()
            currentNode = parent
        }
        path.add(representative)
        return path
    }

    fun getRepresentatives(): List<T> {
        return parent.values.map { find(it) }.distinct()
    }

    fun findAndCompress(element: T): T {
        val representative = find(element)
        compressPath(element, representative)
        return representative
    }

    private tailrec fun compressPath(element: T, representative: T) {
        val currentNode = element
        if (currentNode != representative) {
            val parent = parent[currentNode] ?: return
            parent[currentNode] = representative
            compressPath(parent, representative)
        }
    }

    fun printSets() {
        val sets = mutableMapOf<T, MutableList<T>>()

        for (element in parent.keys) {
            val representative = find(element)
            sets.computeIfAbsent(representative) { mutableListOf() }
            sets[representative]!!.add(element)
        }

        for ((representative, elements) in sets) {
            println("Set with representative $representative: $elements")
        }
    }
}

fun main() {
    // Example usage
    val disjointSets = DisjointSets<Int>()

    disjointSets.makeSet(1)
    disjointSets.makeSet(2)
    disjointSets.makeSet(3)
    disjointSets.makeSet(4)

    disjointSets.union(1, 2)
    disjointSets.union(3, 4)
    disjointSets.union(2, 3)

    println("Is 1 in the same set as 2: ${disjointSets.isSameSet(1, 2)}") // true
    println("Is 1 in the same set as 3: ${disjointSets.isSameSet(1, 3)}") // true
    println("Is 1 in the same set as 4: ${disjointSets.isSameSet(1, 4)}") // true
    println("Is 2 in the same set as 4: ${disjointSets.isSameSet(2, 4)}") // true
    println("Is 1 in the same set as 5: ${disjointSets.isSameSet(1, 5)}") // false

    disjointSets.removeSet(3)

    println("Is 1 in the same set as 2: ${disjointSets.isSameSet(1, 2)}") // true
    println("Is 1 in the same set as 3: ${disjointSets.isSameSet(1, 3)}") // false
    println("Is 1 in the same set as 4: ${disjointSets.isSameSet(1, 4)}") // true
    println("Is 2 in the same set as 4: ${disjointSets.isSameSet(2, 4)}") // true

    disjointSets.printSets()
}
