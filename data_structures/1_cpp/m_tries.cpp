#include <iostream>
#include <unordered_map>
#include <vector>

class TrieNode {
public:
    char value;
    std::unordered_map<char, TrieNode*> children;
    bool isEndOfWord;

    TrieNode(char val = '\0') : value(val), isEndOfWord(false) {}
};

class Trie {
private:
    TrieNode* root;

public:
    Trie() {
        root = new TrieNode();
    }

    // Insertion
    void insert(const std::string& word) {
        TrieNode* currentNode = root;
        for (char ch : word) {
            if (currentNode->children.find(ch) != currentNode->children.end()) {
                currentNode = currentNode->children[ch];
            } else {
                TrieNode* newNode = new TrieNode(ch);
                currentNode->children[ch] = newNode;
                currentNode = newNode;
            }
        }
        currentNode->isEndOfWord = true;
    }

    // Search
    bool search(const std::string& word) {
        TrieNode* currentNode = root;
        for (char ch : word) {
            if (currentNode->children.find(ch) != currentNode->children.end()) {
                currentNode = currentNode->children[ch];
            } else {
                return false;
            }
        }
        return currentNode->isEndOfWord;
    }

    // Prefix Search
    bool startsWith(const std::string& prefix) {
        TrieNode* currentNode = root;
        for (char ch : prefix) {
            if (currentNode->children.find(ch) != currentNode->children.end()) {
                currentNode = currentNode->children[ch];
            } else {
                return false;
            }
        }
        return true;
    }

    // Deletion
    void deleteWord(const std::string& word) {
        deleteWord(root, word, 0);
    }

    bool deleteWord(TrieNode* currentNode, const std::string& word, int index) {
        if (currentNode == nullptr) {
            return false;
        }

        if (index == word.length()) {
            if (currentNode->isEndOfWord) {
                currentNode->isEndOfWord = false;
                return currentNode->children.empty();
            }
            return false;
        }

        char ch = word[index];
        auto it = currentNode->children.find(ch);

        if (it != currentNode->children.end() && deleteWord(it->second, word, index + 1)) {
            delete it->second;
            currentNode->children.erase(it);
            return currentNode->children.empty();
        }

        return false;
    }

    // Count Words
    int countWords() {
        return countWords(root);
    }

    int countWords(TrieNode* node) {
        if (node == nullptr) {
            return 0;
        }

        int count = 0;
        if (node->isEndOfWord) {
            count++;
        }

        for (auto& child : node->children) {
            count += countWords(child.second);
        }

        return count;
    }

    // List Words
    std::vector<std::string> listWords() {
        return listWords(root, "");
    }

    std::vector<std::string> listWords(TrieNode* node, const std::string& prefix) {
        std::vector<std::string> words;

        if (node == nullptr) {
            return words;
        }

        if (node->isEndOfWord) {
            words.push_back(prefix);
        }

        for (auto& child : node->children) {
            std::string newPrefix = prefix + child.first;
            auto childWords = listWords(child.second, newPrefix);
            words.insert(words.end(), childWords.begin(), childWords.end());
        }

        return words;
    }

    // Clear Trie
    void clear() {
        clear(root);
        root = new TrieNode();
    }

    void clear(TrieNode* node) {
        if (node == nullptr) {
            return;
        }

        for (auto& child : node->children) {
            clear(child.second);
        }

        delete node;
    }

    // Check if Empty
    bool isEmpty() {
        return root->children.empty();
    }

    // Destructor
    ~Trie() {
        clear(root);
    }
};

int main() {
    Trie trie;
    trie.insert("hello");
    trie.insert("hey");
    trie.insert("how");

    std::cout << "Trie contains 'hello': " << trie.search("hello") << std::endl;  // true
    std::cout << "Trie contains 'he': " << trie.search("he") << std::endl;        // false
    std::cout << "Trie starts with 'he': " << trie.startsWith("he") << std::endl; // true

    trie.deleteWord("hello");
    std::cout << "Trie contains 'hello' after deletion: " << trie.search("hello") << std::endl; // false
    std::cout << "Trie contains 'hey' after deletion: " << trie.search("hey") << std::endl;       // true
    std::cout << "Trie contains 'he' after deletion: " << trie.search("he") << std::endl;          // false

    std::cout << "Count of words in trie: " << trie.countWords() << std::endl; // 2

    std::cout << "List of words in trie: ";
    auto words = trie.listWords();
    for (const auto& word : words) {
        std::cout << word << " ";
    }
    std::cout << std::endl;

    std::cout << "Is trie empty: " << trie.isEmpty() << std::endl; // false

    trie.clear();
    std::cout << "Is trie empty after clearing: " << trie.isEmpty() << std::endl; // true

    return 0;
}
