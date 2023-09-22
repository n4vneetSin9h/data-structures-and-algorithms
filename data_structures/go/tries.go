package main

import "fmt"

type TrieNode struct {
	value       rune
	children    map[rune]*TrieNode
	isEndOfWord bool
}

type Trie struct {
	root *TrieNode
}

func NewTrieNode() *TrieNode {
	return &TrieNode{
		children: make(map[rune]*TrieNode),
	}
}

func NewTrie() *Trie {
	return &Trie{
		root: NewTrieNode(),
	}
}

// Insertion
func (t *Trie) Insert(word string) {
	currentNode := t.root
	for _, char := range word {
		childNode, ok := currentNode.children[char]
		if !ok {
			newNode := NewTrieNode()
			newNode.value = char
			currentNode.children[char] = newNode
			childNode = newNode
		}
		currentNode = childNode
	}
	currentNode.isEndOfWord = true
}

// Search
func (t *Trie) Search(word string) bool {
	currentNode := t.root
	for _, char := range word {
		childNode, ok := currentNode.children[char]
		if !ok {
			return false
		}
		currentNode = childNode
	}
	return currentNode.isEndOfWord
}

// Prefix Search
func (t *Trie) StartsWith(prefix string) bool {
	currentNode := t.root
	for _, char := range prefix {
		childNode, ok := currentNode.children[char]
		if !ok {
			return false
		}
		currentNode = childNode
	}
	return true
}

// Deletion
func (t *Trie) Delete(word string) {
	t.delete(t.root, word, 0)
}

func (t *Trie) delete(currentNode *TrieNode, word string, index int) bool {
	if currentNode == nil {
		return false
	}

	if index == len(word) {
		if currentNode.isEndOfWord {
			currentNode.isEndOfWord = false
			return len(currentNode.children) == 0
		}
		return false
	}

	char := rune(word[index])
	nextNode, ok := currentNode.children[char]
	if ok && t.delete(nextNode, word, index+1) {
		delete(currentNode.children, char)
		return len(currentNode.children) == 0
	}

	return false
}

// Count Words
func (t *Trie) CountWords() int {
	return t.countWords(t.root)
}

func (t *Trie) countWords(node *TrieNode) int {
	if node == nil {
		return 0
	}

	count := 0
	if node.isEndOfWord {
		count++
	}

	for _, childNode := range node.children {
		count += t.countWords(childNode)
	}

	return count
}

// List Words
func (t *Trie) ListWords() []string {
	words := []string{}
	t.listWords(t.root, "", &words)
	return words
}

func (t *Trie) listWords(node *TrieNode, prefix string, words *[]string) {
	if node == nil {
		return
	}

	if node.isEndOfWord {
		*words = append(*words, prefix)
	}

	for char, childNode := range node.children {
		t.listWords(childNode, prefix+string(char), words)
	}
}

// Clear Trie
func (t *Trie) Clear() {
	t.root.children = make(map[rune]*TrieNode)
}

// Check if Empty
func (t *Trie) IsEmpty() bool {
	return len(t.root.children) == 0
}

// PrintTries prints all words in the trie
func (t *Trie) PrintTries(prefix string) {
	t.printTries(t.root, prefix)
}

func (t *Trie) printTries(node *TrieNode, prefix string) {
	if node == nil {
		return
	}

	if node.isEndOfWord {
		fmt.Println(prefix)
	}

	for char, childNode := range node.children {
		t.printTries(childNode, prefix+string(char))
	}
}
