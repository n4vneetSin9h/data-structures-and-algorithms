# Binary Search Algorithm

## Description

Binary Search is an efficient algorithm for finding a specific element within a sorted array or list of elements. It works by repeatedly dividing in half the portion of the array that could contain the target value, ultimately narrowing down the search until the target is found or determined to be not in the array.

## Steps

1. Start with defining pointers `left` and `right` pointing to the start and end of the array, respectively.
2. Calculate the mid index as `mid = (left + right) / 2`.
3. Compare the element at the mid index with the target:
   - If they match, the target is found, and we return the mid index.
   - If the target is less than the element at the mid index, update `right = mid - 1` to search in the left half.
   - If the target is greater than the element at the mid index, update `left = mid + 1` to search in the right half.
4. Repeat steps 2 and 3 until `left` is greater than `right`, indicating that the target is not present in the array.

## Pseudo Code

```plaintext
BinarySearch(array, target):
    left = 0
    right = length(array) - 1

    while left <= right:
        mid = left + (right - left) / 2

        if array[mid] == target:
            return mid
        else if array[mid] < target:
            left = mid + 1
        else:
            right = mid - 1

    return -1  // Target not found
```

# Binary Search Algorithm: Time and Space Complexity Analysis

## Time Complexity Analysis

The time complexity of the Binary Search Algorithm is analyzed based on the number of comparisons made during the search. Let's denote the length of the array as `n`.

- **Best Case**: O(1)
  - The best case occurs when the target element is found at the middle of the array in the first comparison.

- **Worst Case**: O(log n)
  - The worst case occurs when the target element is not present in the array, and we have to perform log₂(n) comparisons to determine this, effectively halving the search space at each step.

- **Average Case**: O(log n)
  - The average case is also O(log n) assuming a random distribution of the target element.

## Space Complexity Analysis

The space complexity of the Binary Search Algorithm is constant, denoted as O(1). This is because the algorithm uses a fixed amount of additional memory to store variables and pointers, regardless of the size of the input array.

## SWIFT

In this code:

- The binarySearch function takes a sorted array of a generic type T that conforms to the Comparable protocol and a target element target.
- It uses a while loop to iteratively narrow down the search range by adjusting the left and right pointers based on comparisons.
- If the target is found, it returns the index of the target element. If the target is not present, it returns nil.
- You can modify the sortedArray and target in the example usage to test different scenarios.

```swift
func binarySearch<T: Comparable>(_ array: [T], target: T) -> Int? {
    var left = 0
    var right = array.count - 1
    
    while left <= right {
        let mid = left + (right - left) / 2
        
        if array[mid] == target {
            return mid
        } else if array[mid] < target {
            left = mid + 1
        } else {
            right = mid - 1
        }
    }
    
    return nil
}

// Example usage
let sortedArray = [1, 3, 5, 7, 9, 11, 13, 15, 17]
if let targetIndex = binarySearch(sortedArray, target: 11) {
    print("Element found at index: \(targetIndex)")
} else {
    print("Element not found in the array.")
}
```


