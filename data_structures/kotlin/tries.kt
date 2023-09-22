class TrieNode {
    var value: Char? = null
    val children: MutableMap<Char, TrieNode> = mutableMapOf()
    var isEndOfWord: Boolean = false
}

class Trie {
    private val root: TrieNode = TrieNode()

    // Insertion
    fun insert(word: String) {
        var currentNode = root
        for (char in word) {
            currentNode = currentNode.children.getOrPut(char) { TrieNode().apply { value = char } }
        }
        currentNode.isEndOfWord = true
    }

    // Search
    fun search(word: String): Boolean {
        var currentNode = root
        for (char in word) {
            currentNode = currentNode.children[char] ?: return false
        }
        return currentNode.isEndOfWord
    }

    // Prefix Search
    fun startsWith(prefix: String): Boolean {
        var currentNode = root
        for (char in prefix) {
            currentNode = currentNode.children[char] ?: return false
        }
        return true
    }

    // Deletion
    fun delete(word: String) {
        delete(root, word, 0)
    }

    private fun delete(currentNode: TrieNode?, word: String, index: Int): Boolean {
        val currentNode = currentNode ?: return false

        if (index == word.length) {
            if (currentNode.isEndOfWord) {
                currentNode.isEndOfWord = false
                return currentNode.children.isEmpty()
            }
            return false
        }

        val char = word[index]
        currentNode.children[char]?.let { nextNode ->
            if (delete(nextNode, word, index + 1)) {
                currentNode.children.remove(char)
                return currentNode.children.isEmpty()
            }
        }

        return false
    }

    // Count Words
    fun countWords(): Int {
        return countWords(root)
    }

    private fun countWords(node: TrieNode?): Int {
        node ?: return 0

        var count = 0
        if (node.isEndOfWord) {
            count++
        }

        for (childNode in node.children.values) {
            count += countWords(childNode)
        }

        return count
    }

    // List Words
    fun listWords(): List<String> {
        val words = mutableListOf<String>()
        listWords(root, "", words)
        return words
    }

    private fun listWords(node: TrieNode?, prefix: String, words: MutableList<String>) {
        node ?: return

        if (node.isEndOfWord) {
            words.add(prefix)
        }

        for ((char, childNode) in node.children) {
            listWords(childNode, prefix + char, words)
        }
    }

    // Clear Trie
    fun clear() {
        root.children.clear()
    }

    // Check if Empty
    fun isEmpty(): Boolean {
        return root.children.isEmpty()
    }
}

fun main() {
    val trie = Trie()
    trie.insert("hello")
    trie.insert("hey")
    trie.insert("how")

    println("Trie contains 'hello': ${trie.search("hello")}")  // true
    println("Trie contains 'he': ${trie.search("he")}")        // false
    println("Trie starts with 'he': ${trie.startsWith("he")}") // true

    trie.delete("hello")
    println("Trie contains 'hello' after deletion: ${trie.search("hello")}") // false
    println("Trie contains 'hey' after deletion: ${trie.search("hey")}")       // true
    println("Trie contains 'he' after deletion: ${trie.search("he")}")          // false

    println("Count of words in trie: ${trie.countWords()}")  // 2
    println("List of words in trie: ${trie.listWords()}")    // [hey, how]
    println("Is trie empty: ${trie.isEmpty()}")              // false

    trie.clear()
    println("Is trie empty after clearing: ${trie.isEmpty()}") // true
}
