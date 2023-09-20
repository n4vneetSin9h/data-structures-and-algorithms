use std::collections::HashMap;

#[derive(Default)]
struct TrieNode {
    value: Option<char>,
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    // Insertion
    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;
        for ch in word.chars() {
            current_node = current_node.children.entry(ch).or_insert(TrieNode {
                value: Some(ch),
                ..Default::default()
            });
        }
        current_node.is_end_of_word = true;
    }

    // Search
    pub fn search(&self, word: &str) -> bool {
        let mut current_node = &self.root;
        for ch in word.chars() {
            if let Some(node) = current_node.children.get(&ch) {
                current_node = node;
            } else {
                return false;
            }
        }
        current_node.is_end_of_word
    }

    // Prefix Search
    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut current_node = &self.root;
        for ch in prefix.chars() {
            if let Some(node) = current_node.children.get(&ch) {
                current_node = node;
            } else {
                return false;
            }
        }
        true
    }

    // Deletion
    pub fn delete(&mut self, word: &str) {
        self.delete_helper(&mut self.root, word, 0);
    }

    fn delete_helper(&mut self, current_node: &mut TrieNode, word: &str, index: usize) -> bool {
        if index == word.len() {
            if current_node.is_end_of_word {
                current_node.is_end_of_word = false;
                return current_node.children.is_empty();
            }
            return false;
        }

        if let Some(ch) = word.chars().nth(index) {
            if let Some(next_node) = current_node.children.get_mut(&ch) {
                if self.delete_helper(next_node, word, index + 1) {
                    current_node.children.remove(&ch);
                    return current_node.children.is_empty();
                }
            }
        }

        false
    }

    // Count Words
    pub fn count_words(&self) -> usize {
        self.count_words_helper(&self.root)
    }

    fn count_words_helper(&self, current_node: &TrieNode) -> usize {
        let mut count = 0;
        if current_node.is_end_of_word {
            count += 1;
        }

        for (_, child_node) in &current_node.children {
            count += self.count_words_helper(child_node);
        }

        count
    }

    // List Words
    pub fn list_words(&self) -> Vec<String> {
        let mut words = Vec::new();
        self.list_words_helper(&self.root, String::new(), &mut words);
        words
    }

    fn list_words_helper(&self, current_node: &TrieNode, prefix: String, words: &mut Vec<String>) {
        if current_node.is_end_of_word {
            words.push(prefix.clone());
        }

        for (ch, child_node) in &current_node.children {
            let new_prefix = format!("{}{}", prefix, ch);
            self.list_words_helper(child_node, new_prefix, words);
        }
    }

    // Clear Trie
    pub fn clear(&mut self) {
        self.root = TrieNode::default();
    }

    // Check if Empty
    pub fn is_empty(&self) -> bool {
        self.root.children.is_empty()
    }
}

fn main() {
    let mut trie = Trie::new();
    trie.insert("hello");
    trie.insert("hey");
    trie.insert("how");

    println!("Trie contains 'hello': {}", trie.search("hello")); // true
    println!("Trie contains 'he': {}", trie.search("he")); // false
    println!("Trie starts with 'he': {}", trie.starts_with("he")); // true

    trie.delete("hello");
    println!("Trie contains 'hello' after deletion: {}", trie.search("hello")); // false
    println!("Trie contains 'hey' after deletion: {}", trie.search("hey")); // true
    println!("Trie contains 'he' after deletion: {}", trie.search("he")); // false

    println!("Count of words in trie: {}", trie.count_words()); // 2
    println!("List of words in trie: {:?}", trie.list_words()); // ["hey", "how"]
    println!("Is trie empty: {}", trie.is_empty()); // false

    trie.clear();
    println!("Is trie empty after clearing: {}", trie.is_empty()); // true
}
