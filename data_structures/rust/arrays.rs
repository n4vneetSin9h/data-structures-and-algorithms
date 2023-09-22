struct CustomArray<T> {
    elements: Vec<T>, // Internal storage for elements
}

impl<T> CustomArray<T> {
    // Initialize the custom array with a specified initial size
    fn new(initial_size: usize) -> CustomArray<T> {
        CustomArray {
            elements: Vec::with_capacity(initial_size),
        }
    }

    // Append an element to the end of the custom array
    fn append(&mut self, element: T) {
        self.elements.push(element);
    }

    // Insert an element at the specified index
    fn insert(&mut self, element: T, index: usize) {
        self.elements.insert(index, element);
    }

    // Remove the first occurrence of the specified element
    fn remove(&mut self, element: &T) {
        if let Some(index) = self.elements.iter().position(|x| x == element) {
            self.elements.remove(index);
        }
    }

    // Remove and return the element at the specified index
    fn pop(&mut self, index: usize) -> Option<T> {
        if index < self.elements.len() {
            Some(self.elements.remove(index))
        } else {
            None
        }
    }

    // Get the element at the specified index
    fn get(&self, index: usize) -> Option<&T> {
        self.elements.get(index)
    }

    // Set the element at the specified index to the given element
    fn set(&mut self, index: usize, element: T) {
        if index < self.elements.len() {
            self.elements[index] = element;
        }
    }

    // Return the current size of the custom array
    fn size(&self) -> usize {
        self.elements.len()
    }

    // Check if the custom array is empty
    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    // Return the index of the first occurrence of the specified element, or None if not found
    fn index(&self, element: &T) -> Option<usize> {
        self.elements.iter().position(|x| x == element)
    }

    // Return the number of occurrences of the specified element in the custom array
    fn count(&self, element: &T) -> usize {
        self.elements.iter().filter(|&&x| x == *element).count()
    }

    // Reverse the order of elements in the custom array
    fn reverse(&mut self) {
        self.elements.reverse();
    }

    // Sort the elements in the custom array in ascending order
    fn sort(&mut self)
    where
        T: Ord,
    {
        self.elements.sort();
    }

    // Return a new vector containing elements from the specified start index to the end index
    fn slice(&self, start: usize, end: usize) -> Vec<&T> {
        if start <= end && end < self.elements.len() {
            self.elements[start..=end].to_vec()
        } else {
            Vec::new()
        }
    }
}

fn main() {
    let mut custom_array = CustomArray::new(5);
    custom_array.append(10);
    custom_array.append(20);
    custom_array.insert(15, 1);

    println!("Size of array: {}", custom_array.size());

    // Print each element
    for element in custom_array.elements.iter() {
        println!("{}", element);
    }
}
