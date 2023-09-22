class TrieNode:
    def __init__(self):
        self.value = None
        self.children = {}
        self.is_end_of_word = False

class Trie:
    def __init__(self):
        self.root = TrieNode()

    # Insertion
    def insert(self, word):
        current_node = self.root
        for char in word:
            if char in current_node.children:
                current_node = current_node.children[char]
            else:
                new_node = TrieNode()
                new_node.value = char
                current_node.children[char] = new_node
                current_node = new_node
        current_node.is_end_of_word = True

    # Search
    def search(self, word):
        current_node = self.root
        for char in word:
            if char in current_node.children:
                current_node = current_node.children[char]
            else:
                return False
        return current_node.is_end_of_word

    # Prefix Search
    def starts_with(self, prefix):
        current_node = self.root
        for char in prefix:
            if char in current_node.children:
                current_node = current_node.children[char]
            else:
                return False
        return True

    # Deletion
    def delete(self, word):
        self._delete(self.root, word, 0)

    def _delete(self, current_node, word, index):
        if index == len(word):
            if current_node.is_end_of_word:
                current_node.is_end_of_word = False
                return len(current_node.children) == 0
            return False

        char = word[index]
        if char in current_node.children and self._delete(current_node.children[char], word, index + 1):
            del current_node.children[char]
            return len(current_node.children) == 0

        return False

    # Count Words
    def count_words(self):
        return self._count_words(self.root)

    def _count_words(self, node):
        count = 0
        if node.is_end_of_word:
            count += 1

        for child_node in node.children.values():
            count += self._count_words(child_node)

        return count

    # List Words
    def list_words(self):
        words = []
        self._list_words(self.root, "", words)
        return words

    def _list_words(self, node, prefix, words):
        if node.is_end_of_word:
            words.append(prefix)

        for char, child_node in node.children.items():
            self._list_words(child_node, prefix + char, words)

    # Clear Trie
    def clear(self):
        self.root = TrieNode()

    # Check if Empty
    def is_empty(self):
        return not bool(self.root.children)

# Example usage
trie = Trie()
trie.insert("hello")
trie.insert("hey")
trie.insert("how")

print("Trie contains 'hello':", trie.search("hello"))  # True
print("Trie contains 'he':", trie.search("he"))        # False
print("Trie starts with 'he':", trie.starts_with("he")) # True

trie.delete("hello")
print("Trie contains 'hello' after deletion:", trie.search("hello"))  # False
print("Trie contains 'hey' after deletion:", trie.search("hey"))       # True
print("Trie contains 'he' after deletion:", trie.search("he"))          # False

print("Count of words in trie:", trie.count_words())  # 2
print("List of words in trie:", trie.list_words())    # ['hey', 'how']
print("Is trie empty:", trie.is_empty())              # False

trie.clear()
print("Is trie empty after clearing:", trie.is_empty())  # True
