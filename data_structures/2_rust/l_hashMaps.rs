use std::collections::HashSet;
use std::hash::Hash;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

struct KeyValue<T: Hash + Eq, U> {
    key: T,
    value: U,
}

struct HashMap<T: Hash + Eq, U> {
    buckets: Vec<Vec<KeyValue<T, U>>>,
    capacity: usize,
}

impl<T: Hash + Eq, U> HashMap<T, U> {
    fn new(capacity: usize) -> Self {
        HashMap {
            capacity: capacity.max(1),
            buckets: vec![Vec::new(); capacity.max(1)],
        }
    }

    fn bucket_index(&self, key: &T) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() as usize % self.capacity
    }

    fn set_value(&mut self, value: U, key: T) {
        let index = self.bucket_index(&key);
        for kv in &mut self.buckets[index] {
            if kv.key == key {
                kv.value = value;
                return;
            }
        }
        self.buckets[index].push(KeyValue { key, value });
    }

    fn get_value(&self, key: &T) -> Option<&U> {
        let index = self.bucket_index(key);
        for kv in &self.buckets[index] {
            if kv.key == *key {
                return Some(&kv.value);
            }
        }
        None
    }

    fn remove_value(&mut self, key: &T) {
        let index = self.bucket_index(key);
        self.buckets[index].retain(|kv| kv.key != *key);
    }

    fn count(&self) -> usize {
        self.buckets.iter().map(|bucket| bucket.len()).sum()
    }

    fn all_keys(&self) -> Vec<&T> {
        self.buckets.iter().flat_map(|bucket| bucket.iter().map(|kv| &kv.key)).collect()
    }

    fn all_values(&self) -> Vec<&U> {
        self.buckets.iter().flat_map(|bucket| bucket.iter().map(|kv| &kv.value)).collect()
    }

    fn is_empty(&self) -> bool {
        self.buckets.iter().all(|bucket| bucket.is_empty())
    }

    fn remove_all(&mut self) {
        self.buckets = vec![Vec::new(); self.capacity];
    }

    fn contains(&self, key: &T) -> bool {
        let index = self.bucket_index(key);
        self.buckets[index].iter().any(|kv| kv.key == *key)
    }

    fn load_factor(&self) -> f64 {
        self.count() as f64 / self.capacity as f64
    }

    fn key_value_pairs(&self) -> HashSet<KeyValue<T, U>> {
        self.buckets.iter().flat_map(|bucket| bucket.iter().cloned()).collect()
    }

    fn update_value(&mut self, value: U, key: T) {
        let index = self.bucket_index(&key);
        for kv in &mut self.buckets[index] {
            if kv.key == key {
                kv.value = value;
                return;
            }
        }
        self.buckets[index].push(KeyValue { key, value });
    }

    fn update_values(&mut self, dictionary: std::collections::HashMap<T, U>) {
        for (key, value) in dictionary {
            self.update_value(value, key);
        }
    }
}

fn main() {
    let mut hashmap: HashMap<String, i32> = HashMap::new(10);
    hashmap.set_value(5, "five".to_string());
    hashmap.set_value(10, "ten".to_string());

    if let Some(value) = hashmap.get_value(&"five".to_string()) {
        println!("Value for key 'five': {}", value);
    }

    if let Some(value) = hashmap.get_value(&"ten".to_string()) {
        println!("Value for key 'ten': {}", value);
    }

    hashmap.remove_value(&"five".to_string());

    println!("Contains key 'five': {}", hashmap.contains(&"five".to_string()));
    println!("Total count: {}", hashmap.count());

    let keys = hashmap.all_keys();
    println!("All keys: {:?}", keys);

    let values = hashmap.all_values();
    println!("All values: {:?}", values);

    println!("Is empty: {}", hashmap.is_empty());
    println!("Load factor: {}", hashmap.load_factor());
}
