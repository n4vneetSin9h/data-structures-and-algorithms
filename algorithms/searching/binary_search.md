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

- **Best Case**: O(1)

- **Worst Case**: O(log n)

- **Average Case**: O(log n)

## Space Complexity Analysis

The space complexity of the Binary Search Algorithm is constant, denoted as O(1).

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

