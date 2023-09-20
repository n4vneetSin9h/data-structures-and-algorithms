
/*
In this code:

- The binarySearch function takes a sorted array of a generic type T that conforms to the Comparable protocol and a target element target.
- It uses a while loop to iteratively narrow down the search range by adjusting the left and right pointers based on comparisons.
- If the target is found, it returns the index of the target element. If the target is not present, it returns nil.
- You can modify the sortedArray and target in the example usage to test different scenarios.
*/

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


