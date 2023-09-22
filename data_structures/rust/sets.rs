use std::collections::HashMap;

#[derive(Clone)]
struct CustomSet<T: std::hash::Hash + Eq> {
    elements: HashMap<T, bool>,  // HashMap to mimic a set
}

impl<T: std::hash::Hash + Eq> CustomSet<T> {
    // Basic Set Operations

    /// Check if the set is empty.
    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Get the number of elements in the set.
    fn count(&self) -> usize {
        self.elements.len()
    }

    /// Insert an element into the set.
    fn insert(&mut self, element: T) {
        self.elements.insert(element, true);
    }

    /// Remove an element from the set.
    fn remove(&mut self, element: &T) -> Option<T> {
        if self.elements.remove(element).is_some() {
            Some(element.clone())
        } else {
            None
        }
    }

    /// Check if the set contains a specific element.
    fn contains(&self, element: &T) -> bool {
        self.elements.contains_key(element)
    }

    // Set Operations

    /// Perform the union operation with another set and return a new set.
    fn union(&self, other_set: &CustomSet<T>) -> CustomSet<T> {
        let mut new_set = self.clone();
        for element in other_set.elements.keys() {
            new_set.insert(element.clone());
        }
        new_set
    }

    /// Perform the intersection operation with another set and return a new set.
    fn intersection(&self, other_set: &CustomSet<T>) -> CustomSet<T> {
        let mut new_set = CustomSet { elements: HashMap::new() };
        for element in self.elements.keys() {
            if other_set.contains(element) {
                new_set.insert(element.clone());
            }
        }
        new_set
    }

    /// Perform the difference operation with another set and return a new set.
    fn difference(&self, other_set: &CustomSet<T>) -> CustomSet<T> {
        let mut new_set = self.clone();
        for element in other_set.elements.keys() {
            new_set.remove(element);
        }
        new_set
    }

    /// Check if this set is a subset of another set.
    fn is_subset_of(&self, other_set: &CustomSet<T>) -> bool {
        self.elements.keys().all(|element| other_set.contains(element))
    }

    /// Check if this set is a superset of another set.
    fn is_superset_of(&self, other_set: &CustomSet<T>) -> bool {
        other_set.is_subset_of(self)
    }

    /// Check if this set is disjoint with another set.
    fn is_disjoint_with(&self, other_set: &CustomSet<T>) -> bool {
        self.elements.keys().all(|element| !other_set.contains(element))
    }

    /// Apply a closure to each element in the set.
    fn for_each<F>(&self, closure: F)
    where
        F: Fn(&T),
    {
        for element in self.elements.keys() {
            closure(element);
        }
    }

    /// Remove all elements from the set.
    fn remove_all(&mut self) {
        self.elements.clear();
    }

    /// Remove all occurrences of an element from the set.
    fn remove_all_occurrences(&mut self, element: &T) {
        self.elements.remove(element);
    }

    /// Get the set with common elements between this set and another set.
    fn symmetric_difference(&self, other_set: &CustomSet<T>) -> CustomSet<T> {
        let diff1 = self.difference(other_set);
        let diff2 = other_set.difference(self);
        diff1.union(&diff2)
    }

    /// Insert elements from another set into this set.
    fn form_union(&mut self, other_set: &CustomSet<T>) {
        for element in other_set.elements.keys() {
            self.insert(element.clone());
        }
    }

    /// Remove elements of this set that are not in another set.
    fn form_intersection(&mut self, other_set: &CustomSet<T>) {
        let intersection = self.intersection(other_set);
        self.elements = intersection.elements;
    }

    /// Remove elements of this set that are in another set.
    fn subtract(&mut self, other_set: &CustomSet<T>) {
        for element in other_set.elements.keys() {
            self.remove(element);
        }
    }

    // ... Add more set operations as needed ...
}

// Example usage
fn main() {
    let mut set1 = CustomSet {
        elements: HashMap::new(),
    };
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);

    let mut set2 = CustomSet {
        elements: HashMap::new(),
    };
    set2.insert(2);
    set2.insert(3);
    set2.insert(4);

    let union_set = set1.union(&set2);
    println!("Union set: {:?}", union_set.elements.keys());

    let intersection_set = set1.intersection(&set2);
    println!("Intersection set: {:?}", intersection_set.elements.keys());
}
