# Linear Search Algorithm

## Description

Linear Search is a simple algorithm for finding a specific element within an array or list. It sequentially checks each element in the array until a match is found or the end of the array is reached.

## Steps

1. Start at the beginning of the array.
2. Traverse the array element by element.
3. Compare the current element with the target:
   - If they match, the target is found, and we return the current index.
   - If not, move to the next element.
4. Repeat step 3 until the end of the array is reached or the target is found.

## Pseudo Code

```plaintext
LinearSearch(array, target):
    for i from 0 to length(array) - 1:
        if array[i] equals target:
            return i  // Target found at index i

    return -1  // Target not found
```

# Linear Search Algorithm: Time and Space Complexity Analysis

## Time Complexity Analysis

The time complexity of the Linear Search Algorithm is analyzed based on the number of comparisons made during the search. Let's denote the length of the array as `n`.

- **Best Case**: O(1)
  - The best case occurs when the target element is found at the first comparison.

- **Worst Case**: O(n)
  - The worst case occurs when the target element is not present in the array, and we have to traverse the entire array.

- **Average Case**: O(n)
  - The average case is also O(n) when the target element is equally likely to be in any position in the array.

## Space Complexity Analysis

The space complexity of the Linear Search Algorithm is constant, denoted as O(1). This is because the algorithm uses a fixed amount of additional memory to store variables, regardless of the size of the input array.

## Functionality

- The `linearSearch` function takes an array of a generic type `T` and a target element target.
- It iterates through the array, comparing each element with the target.
- You can modify the `array` and `target` in the example usage to test different scenarios.


# Code

## C++

```cpp
#include <iostream>
#include <vector>

int linearSearch(const std::vector<int>& arr, int target) {
    for (int i = 0; i < arr.size(); ++i) {
        if (arr[i] == target)
            return i;  // Target found at index i
    }

    return -1;  // Target not found
}

int main() {
    std::vector<int> arr = {2, 5, 8, 12, 16, 23, 38, 45, 56, 72};
    int target = 23;

    int result = linearSearch(arr, target);

    if (result != -1)
        std::cout << "Element " << target << " found at index " << result << "\n";
    else
        std::cout << "Element " << target << " not found in the array\n";

    return 0;
}

```

## Rust

```rs
fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    for (i, &element) in arr.iter().enumerate() {
        if element == target {
            return Some(i);
        }
    }
    None
}

fn main() {
    let arr = [2, 5, 8, 12, 16, 23, 38, 45, 56, 72];
    let target = 23;

    match linear_search(&arr, target) {
        Some(index) => println!("Element {} found at index {}", target, index),
        None => println!("Element {} not found in the array", target),
    }
}

```

## Python

```py
def linear_search(arr, target):
    for i, element in enumerate(arr):
        if element == target:
            return i  # Target found at index i
    return -1  # Target not found

arr = [2, 5, 8, 12, 16, 23, 38, 45, 56, 72]
target = 23

result = linear_search(arr, target)

if result != -1:
    print(f"Element {target} found at index {result}")
else:
    print(f"Element {target} not found in the array")

```

## Swift

```swift
func linearSearch<T: Equatable>(_ array: [T], target: T) -> Int? {
    for (index, element) in array.enumerated() {
        if element == target {
            return index  // Target found at index
        }
    }
    return nil  // Target not found
}

// Example usage
let array = [2, 5, 8, 12, 16, 23, 38, 45, 56, 72]
if let targetIndex = linearSearch(array, target: 23) {
    print("Element found at index: \(targetIndex)")
} else {
    print("Element not found in the array.")
}

```

## Kotlin

```kotlin
fun linearSearch(arr: List<Int>, target: Int): Int {
    for (i in arr.indices) {
        if (arr[i] == target)
            return i  // Target found at index i
    }
    return -1  // Target not found
}

fun main() {
    val arr = listOf(2, 5, 8, 12, 16, 23, 38, 45, 56, 72)
    val target = 23

    val result = linearSearch(arr, target)

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

func linearSearch(arr []int, target int) int {
    for i, element := range arr {
        if element == target {
            return i  // Target found at index i
        }
    }
    return -1  // Target not found
}

func main() {
    arr := []int{2, 5, 8, 12, 16, 23, 38, 45, 56, 72}
    target := 23

    result := linearSearch(arr, target)

    if result != -1 {
        fmt.Printf("Element %d found at index %d\n", target, result)
    } else {
        fmt.Printf("Element %d not found in the array\n", target)
    }
}

```

## Java

```java
public class LinearSearch {
    public static int linearSearch(int[] arr, int target) {
        for (int i = 0; i < arr.length; i++) {
            if (arr[i] == target)
                return i;  // Target found at index i
        }

        return -1;  // Target not found
    }

    public static void main(String[] args) {
        int[] arr = {2, 5, 8, 12, 16, 23, 38, 45, 56, 72};
        int target = 23;

        int result = linearSearch(arr, target);

        if (result != -1)
            System.out.println("Element " + target + " found at index " + result);
        else
            System.out.println("Element " + target + " not found in the array");
    }
}

```

## JavaScript

```js
function linearSearch(arr, target) {
    for (let i = 0; i < arr.length; i++) {
        if (arr[i] === target)
            return i;  // Target found at index i
    }
    return -1;  // Target not found
}

const arr = [2, 5, 8, 12, 16, 23, 38, 45, 56, 72];
const target = 23;

const result = linearSearch(arr, target);

if (result !== -1)
    console.log(`Element ${target} found at index ${result}`);
else
    console.log(`Element ${target} not found in the array`);

```

## C#

```cs
using System;

public class LinearSearch
{
    public static int LinearSearchFunc(int[] arr, int target)
    {
        for (int i = 0; i < arr.Length; i++)
        {
            if (arr[i] == target)
                return i;  // Target found at index i
        }

        return -1;  // Target not found
    }

    public static void Main(string[] args)
    {
        int[] arr = { 2, 5, 8, 12, 16, 23, 38, 45, 56, 72 };
        int target = 23;

        int result = LinearSearchFunc(arr, target);

        if (result != -1)
            Console.WriteLine("Element " + target + " found at index " + result);
        else
            Console.WriteLine("Element " + target + " not found in the array");
    }
}

```

## Ruby

```rb
def linear_search(arr, target)
  arr.each_with_index do |element, index|
    return index if element == target  # Target found at index
  end
  -1  # Target not found
end

arr = [2, 5, 8, 12, 16, 23, 38, 45, 56, 72]
target = 23

result = linear_search(arr, target)

if result != -1
  puts "Element #{target} found at index #{result}"
else
  puts "Element #{target} not found in the array"
end

```

