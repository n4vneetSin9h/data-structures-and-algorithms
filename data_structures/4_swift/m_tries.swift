class TrieNode {
    var value: Character?
    var children: [Character: TrieNode] = [:]
    var isEndOfWord: Bool = false
}

class Trie {
    private var root: TrieNode

    init() {
        root = TrieNode()
    }

    // MARK: - Insertion
    func insert(_ word: String) {
        var currentNode = root
        for char in word {
            if let childNode = currentNode.children[char] {
                currentNode = childNode
            } else {
                let newNode = TrieNode()
                newNode.value = char
                currentNode.children[char] = newNode
                currentNode = newNode
            }
        }
        currentNode.isEndOfWord = true
    }

    // MARK: - Search
    func search(_ word: String) -> Bool {
        var currentNode = root
        for char in word {
            if let childNode = currentNode.children[char] {
                currentNode = childNode
            } else {
                return false
            }
        }
        return currentNode.isEndOfWord
    }

    // MARK: - Prefix Search
    func startsWith(_ prefix: String) -> Bool {
        var currentNode = root
        for char in prefix {
            if let childNode = currentNode.children[char] {
                currentNode = childNode
            } else {
                return false
            }
        }
        return true
    }

    // MARK: - Deletion
    func delete(_ word: String) {
        delete(root, word, index: 0)
    }

    private func delete(_ currentNode: TrieNode?, _ word: String, index: Int) -> Bool {
        guard let currentNode = currentNode else { return false }

        if index == word.count {
            if currentNode.isEndOfWord {
                currentNode.isEndOfWord = false
                return currentNode.children.isEmpty
            }
            return false
        }

        let char = word[word.index(word.startIndex, offsetBy: index)]
        if let nextNode = currentNode.children[char],
           delete(nextNode, word, index: index + 1) {
            currentNode.children[char] = nil
            return currentNode.children.isEmpty
        }

        return false
    }

    // MARK: - Count Words
    func countWords() -> Int {
        return countWords(root)
    }

    private func countWords(_ node: TrieNode?) -> Int {
        guard let node = node else { return 0 }

        var count = 0
        if node.isEndOfWord {
            count += 1
        }

        for (_, childNode) in node.children {
            count += countWords(childNode)
        }

        return count
    }

    // MARK: - List Words
    func listWords() -> [String] {
        return listWords(root, prefix: "")
    }

    private func listWords(_ node: TrieNode?, prefix: String) -> [String] {
        guard let node = node else { return [] }

        var words = [String]()

        if node.isEndOfWord {
            words.append(prefix)
        }

        for (char, childNode) in node.children {
            let newPrefix = prefix + String(char)
            words += listWords(childNode, prefix: newPrefix)
        }

        return words
    }

    // MARK: - Clear Trie
    func clear() {
        root = TrieNode()
    }

    // MARK: - Check if Empty
    func isEmpty() -> Bool {
        return root.children.isEmpty
    }

    // ... Add more trie operations as needed ...
}

// Example usage
let trie = Trie()
trie.insert("hello")
trie.insert("hey")
trie.insert("how")
print("Trie contains 'hello':", trie.search("hello"))  // true
print("Trie contains 'he':", trie.search("he"))        // false
print("Trie starts with 'he':", trie.startsWith("he")) // true
trie.delete("hello")
print("Trie contains 'hello' after deletion:", trie.search("hello")) // false
print("Trie contains 'hey' after deletion:", trie.search("hey"))       // true
print("Trie contains 'he' after deletion:", trie.search("he"))          // false
print("Count of words in trie:", trie.countWords())                      // 2
print("List of words in trie:", trie.listWords())                        // ["hey", "how"]
print("Is trie empty:", trie.isEmpty())                                  // false
trie.clear()
print("Is trie empty after clearing:", trie.isEmpty())                    // true
