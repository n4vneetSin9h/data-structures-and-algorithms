use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct DisjointSets<T: Eq + PartialEq + Hash + Clone + Copy> {
    parent: HashMap<T, T>,
    rank: HashMap<T, i32>,
}

impl<T: Eq + PartialEq + Hash + Clone + Copy> DisjointSets<T> {
    fn new() -> Self {
        DisjointSets {
            parent: HashMap::new(),
            rank: HashMap::new(),
        }
    }

    fn make_set(&mut self, element: T) {
        if !self.parent.contains_key(&element) {
            self.parent.insert(element, element);
            self.rank.insert(element, 0);
        }
    }

    fn find(&mut self, element: T) -> Option<T> {
        let parent = *self.parent.get(&element).unwrap_or(&element);

        if parent == element {
            Some(parent)
        } else {
            let representative = self.find(parent)?;
            self.parent.insert(element, representative);
            Some(representative)
        }
    }

    fn union(&mut self, element1: T, element2: T) {
        let parent1 = self.find(element1).unwrap_or(element1);
        let parent2 = self.find(element2).unwrap_or(element2);

        if parent1 != parent2 {
            let rank1 = *self.rank.get(&parent1).unwrap_or(&0);
            let rank2 = *self.rank.get(&parent2).unwrap_or(&0);

            if rank1 > rank2 {
                self.parent.insert(parent2, parent1);
            } else if rank1 < rank2 {
                self.parent.insert(parent1, parent2);
            } else {
                self.parent.insert(parent2, parent1);
                self.rank.insert(parent1, rank1 + 1);
            }
        }
    }

    fn contains(&self, element: T) -> bool {
        self.parent.contains_key(&element)
    }

    fn set_size(&self, element: T) -> usize {
        let representative = self.find(element).unwrap_or(element);
        self.parent.values().filter(|&&val| self.find(val) == Some(representative)).count()
    }

    fn elements_in_same_set(&self, element: T) -> Vec<T> {
        let representative = self.find(element).unwrap_or(element);
        self.parent.keys().filter(|&&key| self.find(key) == Some(representative)).cloned().collect()
    }

    fn all_sets(&self) -> Vec<Vec<T>> {
        let mut sets: Vec<Vec<T>> = Vec::new();
        let mut set_representatives: HashSet<T> = HashSet::new();

        for &element in self.parent.keys() {
            let representative = self.find(element).unwrap();
            if !set_representatives.contains(&representative) {
                let set: Vec<T> = self.parent.keys()
                    .filter(|&&key| self.find(key) == Some(representative))
                    .cloned().collect();
                sets.push(set);
                set_representatives.insert(representative);
            }
        }

        sets
    }

    fn reset(&mut self) {
        self.parent.clear();
        self.rank.clear();
    }

    fn is_same_set(&mut self, element1: T, element2: T) -> bool {
        let parent1 = self.find(element1).unwrap_or(element1);
        let parent2 = self.find(element2).unwrap_or(element2);
        parent1 == parent2
    }

    fn remove_set(&mut self, element: T) {
        let representative = self.find(element).unwrap();
        self.parent.retain(|_, &mut val| self.find(val) != Some(representative));
        self.rank.retain(|_, &mut val| self.find(val) != Some(representative));
    }

    fn path_to_root(&mut self, element: T) -> Vec<T> {
        let mut path = Vec::new();
        let mut current_node = element;
        let representative = self.find(element).unwrap();

        while current_node != representative {
            path.push(current_node);
            let parent = self.find(current_node).unwrap();
            current_node = parent;
        }
        path.push(representative);
        path
    }

    fn get_representatives(&mut self) -> Vec<T> {
        let mut representatives_set: HashSet<T> = HashSet::new();
        let mut representatives: Vec<T> = Vec::new();

        for &element in self.parent.keys() {
            let representative = self.find(element).unwrap();
            if !representatives_set.contains(&representative) {
                representatives.push(representative);
                representatives_set.insert(representative);
            }
        }

        representatives
    }

    fn find_and_compress(&mut self, element: T) -> Option<T> {
        let representative = self.find(element)?;
        self.compress_path(element, representative);
        Some(representative)
    }

    fn compress_path(&mut self, element: T, representative: T) {
        let mut current_node = element;
        while current_node != representative {
            let parent = self.find(current_node).unwrap();
            self.parent.insert(current_node, representative);
            current_node = parent;
        }
    }

    fn print_sets(&self) {
        let mut sets: HashMap<T, Vec<T>> = HashMap::new();

        for &element in self.parent.keys() {
            let representative = self.find(element).unwrap();
            sets.entry(representative)
                .or_insert_with(Vec::new)
                .push(element);
        }

        for (representative, elements) in sets {
            println!("Set with representative {:?}: {:?}", representative, elements);
        }
    }
}

fn main() {
    // Example usage
    let mut disjoint_sets = DisjointSets::new();

    disjoint_sets.make_set(1);
    disjoint_sets.make_set(2);
    disjoint_sets.make_set(3);
    disjoint_sets.make_set(4);

    disjoint_sets.union(1, 2);
    disjoint_sets.union(3, 4);
    disjoint_sets.union(2, 3);

    println!("Is 1 in the same set as 2: {}", disjoint_sets.is_same_set(1, 2)); // true
    println!("Is 1 in the same set as 3: {}", disjoint_sets.is_same_set(1, 3)); // true
    println!("Is 1 in the same set as 4: {}", disjoint_sets.is_same_set(1, 4)); // true
    println!("Is 2 in the same set as 4: {}", disjoint_sets.is_same_set(2, 4)); // true
    println!("Is 1 in the same set as 5: {}", disjoint_sets.is_same_set(1, 5)); // false

    disjoint_sets.remove_set(3);

    println!("Is 1 in the same set as 2: {}", disjoint_sets.is_same_set(1, 2)); // true
    println!("Is 1 in the same set as 3: {}", disjoint_sets.is_same_set(1, 3)); // false
    println!("Is 1 in the same set as 4: {}", disjoint_sets.is_same_set(1, 4)); // true
    println!("Is 2 in the same set as 4: {}", disjoint_sets.is_same_set(2, 4)); // true

    disjoint_sets.print_sets();
}
