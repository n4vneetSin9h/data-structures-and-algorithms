# Hash-Based Search Algorithm

## Description

Hash-based search algorithms utilize hash tables or hash maps for efficient key-value pair lookups. Hash tables use a hash function to compute an index into an array of buckets or slots, from which the correct value can be found.

## Steps

1. Choose a hash function to map keys to indices in the hash table.
2. Hash the key to obtain the hash value.
3. Use the hash value to determine the index (bucket) in the hash table.
4. Access the bucket and retrieve the associated value.

## Pseudo Code

HashTable data structure includes key-value pairs and a hash function to compute indices.

```plaintext
class HashTable:
    def __init__(self):
        initialize the hash table

    def hash_function(self, key):
        compute hash value based on the key

    def insert(self, key, value):
        hash the key
        determine the index using the hash value
        store the key-value pair at the determined index

    def search(self, key):
        hash the key
        determine the index using the hash value
        retrieve the value associated with the key from the determined index
```

# Hash-Based Search Algorithm: Time and Space Complexity Analysis

## Time Complexity Analysis

- **Best Case**:
- Insertion: O(1) - In the best-case scenario where the hash function distributes keys evenly across the buckets, insertion takes O(1) time as there are no collisions.
- Search: O(1) - In the best-case scenario with a perfect hash function and no collisions, searching for a key takes O(1) time.
- Deletion: O(1) - Similarly, in the best-case scenario, deletion also takes O(1) time with no collisions.

- **Worst Case**:
- Insertion: O(n) - In the worst-case scenario, where all keys hash to the same index (e.g., poor hash function leading to frequent collisions), each insertion may take O(n) time as it needs to handle collisions.
- Search: O(n) - In the worst-case scenario with frequent collisions, searching for a key may also take O(n) time due to traversing through the collided elements.
Deletion: O(n) - Similar to search, deletion may take O(n) time in the worst-case due to collisions.

- **Average Case**:
- Insertion: O(1) - On average, assuming a good hash function and an evenly distributed set of keys, insertion takes O(1) time considering occasional collisions.
- Search: O(1) - On average, with a well-distributed set of keys and a good hash function, searching takes O(1) time accounting for occasional collisions.
- Deletion: O(1) - Similarly, on average, deletion takes O(1) time considering a good hash function and an evenly distributed set of keys.

## Space Complexity Analysis

O(n + m) - In terms of space, where n is the number of elements (key-value pairs) and m is the capacity of the hash table, the space complexity is generally O(n + m). This accounts for storing the elements (O(n)) and the hash table itself (O(m)).

# Code

## C++

```cpp
#include <iostream>
#include <vector>

int binarySearch(const std::vector<int>& arr, int target) {
    int left = 0;
    int right = arr.size() - 1;

    while (left <= right) {
        int mid = left + (right - left) / 2;

        // Check if the target is at the middle
        if (arr[mid] == target)
            return mid;

        // If target is greater, ignore left half
        if (arr[mid] < target)
            left = mid + 1;

        // If target is smaller, ignore right half
        else
            right = mid - 1;
    }

    // Target was not found in the array
    return -1;
}

int main() {
    std::vector<int> arr = {2, 5, 8, 12, 16, 23, 38, 45, 56, 72};
    int target = 23;
    
    int result = binarySearch(arr, target);

    if (result != -1)
        std::cout << "Element " << target << " found at index " << result << "\n";
    else
        std::cout << "Element " << target << " not found in the array.\n";

    return 0;
}
```

## Rust

```rs
fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    None
}

fn main() {
    let arr = vec![2, 5, 8, 12, 16, 23, 38, 45, 56, 72];
    let target = 23;
    
    match binary_search(&arr, target) {
        Some(index) => println!("Element {} found at index {}", target, index),
        None => println!("Element {} not found in the array", target),
    }
}
```

## Python

```py
def binary_search(arr, target):
    left = 0
    right = len(arr) - 1

    while left <= right:
        mid = left + (right - left) // 2

        if arr[mid] == target:
            return mid
        elif arr[mid] < target:
            left = mid + 1
        else:
            right = mid - 1

    return -1

arr = [2, 5, 8, 12, 16, 23, 38, 45, 56, 72]
target = 23

result = binary_search(arr, target)

if result != -1:
    print(f"Element {target} found at index {result}")
else:
    print(f"Element {target} not found in the array")
```

## Swift

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

## Kotlin

```kotlin
fun binarySearch(arr: List<Int>, target: Int): Int {
    var left = 0
    var right = arr.size - 1

    while (left <= right) {
        val mid = left + (right - left) / 2

        when {
            arr[mid] == target -> return mid
            arr[mid] < target -> left = mid + 1
            else -> right = mid - 1
        }
    }

    return -1
}

fun main() {
    val arr = listOf(2, 5, 8, 12, 16, 23, 38, 45, 56, 72)
    val target = 23

    val result = binarySearch(arr, target)

    if (result != -1) {
        println("Element $target found at index $result")
    } else {
        println("Element $target not found in the array")
    }
}
```

## Go

```go
package main

import "fmt"

func binarySearch(arr []int, target int) int {
	left := 0
	right := len(arr) - 1

	for left <= right {
		mid := left + (right-left)/2

		if arr[mid] == target {
			return mid
		} else if arr[mid] < target {
			left = mid + 1
		} else {
			right = mid - 1
		}
	}

	return -1
}

func main() {
	arr := []int{2, 5, 8, 12, 16, 23, 38, 45, 56, 72}
	target := 23

	result := binarySearch(arr, target)

	if result != -1 {
		fmt.Printf("Element %d found at index %d\n", target, result)
	} else {
		fmt.Printf("Element %d not found in the array\n", target)
	}
}
```

## Java

```java
public class BinarySearch {
    public static int binarySearch(int[] arr, int target) {
        int left = 0;
        int right = arr.length - 1;

        while (left <= right) {
            int mid = left + (right - left) / 2;

            if (arr[mid] == target) {
                return mid;
            } else if (arr[mid] < target) {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        return -1;
    }

    public static void main(String[] args) {
        int[] arr = {2, 5, 8, 12, 16, 23, 38, 45, 56, 72};
        int target = 23;

        int result = binarySearch(arr, target);

        if (result != -1) {
            System.out.println("Element " + target + " found at index " + result);
        } else {
            System.out.println("Element " + target + " not found in the array");
        }
    }
}
```

## JavaScript

```js
function binarySearch(arr, target) {
    let left = 0;
    let right = arr.length - 1;

    while (left <= right) {
        const mid = Math.floor(left + (right - left) / 2);

        if (arr[mid] === target) {
            return mid;
        } else if (arr[mid] < target) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    return -1;
}

const arr = [2, 5, 8, 12, 16, 23, 38, 45, 56, 72];
const target = 23;
const result = binarySearch(arr, target);

if (result !== -1) {
    console.log(`Element ${target} found at index ${result}`);
} else {
    console.log(`Element ${target} not found in the array`);
}
```

## C#

```cs
using System;

public class BinarySearch
{
    public static int BinarySearchFunc(int[] arr, int target)
    {
        int left = 0;
        int right = arr.Length - 1;

        while (left <= right)
        {
            int mid = left + (right - left) / 2;

            if (arr[mid] == target)
                return mid;

            if (arr[mid] < target)
                left = mid + 1;
            else
                right = mid - 1;
        }

        return -1;
    }

    public static void Main(string[] args)
    {
        int[] arr = { 2, 5, 8, 12, 16, 23, 38, 45, 56, 72 };
        int target = 23;

        int result = BinarySearchFunc(arr, target);

        if (result != -1)
            Console.WriteLine("Element " + target + " found at index " + result);
        else
            Console.WriteLine("Element " + target + " not found in the array");
    }
}
```

## Ruby

```rb
def binary_search(arr, target)
  left = 0
  right = arr.length - 1

  while left <= right
    mid = left + (right - left) / 2

    if arr[mid] == target
      return mid
    elsif arr[mid] < target
      left = mid + 1
    else
      right = mid - 1
    end
  end

  -1
end

arr = [2, 5, 8, 12, 16, 23, 38, 45, 56, 72]
target = 23

result = binary_search(arr, target)

if result != -1
  puts "Element #{target} found at index #{result}"
else
  puts "Element #{target} not found in the array"
end
```

