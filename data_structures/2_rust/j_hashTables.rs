use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

struct HashTable<Key, Value> {
    buckets: Vec<Vec<(Key, Value)>>,
}

impl<Key, Value> HashTable<Key, Value>
where
    Key: Hash + Eq,
{
    fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity should be greater than 0");
        HashTable {
            buckets: vec![Vec::new(); capacity],
        }
    }

    fn bucket_index(&self, key: &Key) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        hash as usize % self.buckets.len()
    }

    fn get_value(&self, key: &Key) -> Option<&Value> {
        let index = self.bucket_index(key);
        for (k, v) in &self.buckets[index] {
            if k == key {
                return Some(v);
            }
        }
        None
    }

    fn set_value(&mut self, key: Key, value: Value) {
        let index = self.bucket_index(&key);

        // Check if the key already exists, and update the value
        for (i, (k, v)) in self.buckets[index].iter_mut().enumerate() {
            if k == &key {
                self.buckets[index][i] = (key, value);
                return;
            }
        }

        // Key doesn't exist, add a new entry
        self.buckets[index].push((key, value));
    }

    fn remove_value(&mut self, key: &Key) {
        let index = self.bucket_index(key);
        self.buckets[index].retain(|(k, _)| k != key);
    }

    fn remove_all(&mut self) {
        for bucket in &mut self.buckets {
            bucket.clear();
        }
    }

    fn count(&self) -> usize {
        self.buckets.iter().map(|bucket| bucket.len()).sum()
    }

    fn contains(&self, key: &Key) -> bool {
        let index = self.bucket_index(key);
        self.buckets[index].iter().any(|(k, _)| k == key)
    }

    fn all_keys(&self) -> Vec<Key> {
        let keys: Vec<Key> = self
            .buckets
            .iter()
            .flat_map(|bucket| bucket.iter().map(|(k, _)| k.clone()))
            .collect();
        keys
    }

    fn all_values(&self) -> Vec<Value> {
        let values: Vec<Value> = self
            .buckets
            .iter()
            .flat_map(|bucket| bucket.iter().map(|(_, v)| v.clone()))
            .collect();
        values
    }

    fn merge(&mut self, other_table: &HashTable<Key, Value>) {
        for bucket in &other_table.buckets {
            for (key, value) in bucket {
                self.set_value(key.clone(), value.clone());
            }
        }
    }

    fn resize(&mut self, new_capacity: usize) {
        let mut new_buckets = vec![Vec::new(); new_capacity];
        for bucket in &self.buckets {
            for (key, value) in bucket {
                let index = {
                    let mut hasher = DefaultHasher::new();
                    key.hash(&mut hasher);
                    let hash = hasher.finish();
                    hash as usize % new_capacity
                };
                new_buckets[index].push((key.clone(), value.clone()));
            }
        }
        self.buckets = new_buckets;
    }

    // ... Add more hash table operations as needed ...
}

fn main() {
    let mut table: HashTable<i32, String> = HashTable::new(10);

    table.set_value(1, "Value1".to_string());
    table.set_value(2, "Value2".to_string());
    table.set_value(3, "Value3".to_string());

    if let Some(value) = table.get_value(&2) {
        println!("Value for key 2: {}", value);
    }

    table.remove_value(&2);
    println!("Count after removal: {}", table.count());

    let keys = table.all_keys();
    println!("Keys in the table: {:?}", keys);

    // ... Add more tests and usage ...
}
