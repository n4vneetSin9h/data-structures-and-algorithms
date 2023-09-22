# Ternary Search Algorithm

## Description

Ternary Search is an efficient algorithm used to find a specific element within a sorted array or list of elements. It works by dividing the array into three parts and determining whether the target element is in the left, middle, or right portion. This allows for a faster reduction of the search range compared to binary search.

## Steps

1. Start with defining pointers left and right pointing to the start and end of the array, respectively.
2. While `left` is less than or equal to `right`:
   - Calculate two midpoints: `mid1 = left + (right - left) / 3` and `mid2 = right - (right - left) / 3`.
   - Compare the element at `mid1` and `mid2` with the target:
     - If the target is at `mid1`, return `mid1`.
     - If the target is at `mid2`, return `mid2`.
     - If the target is less than the element at `mid1`, update `right = mid1 - 1` to search in the left portion.
     - If the target is greater than the element at `mid2`, update `left = mid2 + 1` to search in the right portion.
     - Otherwise, update `left = mid1 + 1` and `right = mid2 - 1` to search in the middle portion.
3. If the target is not found, return -1.

## Pseudo Code

```plaintext
TernarySearch(array, target):
    left = 0
    right = length(array) - 1

    while left <= right:
        mid1 = left + (right - left) / 3
        mid2 = right - (right - left) / 3

        if array[mid1] == target:
            return mid1
        elif array[mid2] == target:
            return mid2
        elif target < array[mid1]:
            right = mid1 - 1
        elif target > array[mid2]:
            left = mid2 + 1
        else:
            left = mid1 + 1
            right = mid2 - 1

    return -1  // Target not found
```

# Ternary Search Algorithm: Time and Space Complexity Analysis

## Time Complexity Analysis

- **Best Case**: O(1)

- **Worst Case**: O(log₃ n)

- **Average Case**: O(log₃ n)

## Space Complexity Analysis

The space complexity of the Ternary Search Algorithm is constant, denoted as O(1).

# Code

## C++

```cpp
#include <iostream>

int ternarySearch(int arr[], int left, int right, int x) {
    if (right >= left) {
        int mid1 = left + (right - left) / 3;
        int mid2 = right - (right - left) / 3;

        if (arr[mid1] == x)
            return mid1;
        if (arr[mid2] == x)
            return mid2;

        if (x < arr[mid1])
            return ternarySearch(arr, left, mid1 - 1, x);
        else if (x > arr[mid2])
            return ternarySearch(arr, mid2 + 1, right, x);
        else
            return ternarySearch(arr, mid1 + 1, mid2 - 1, x);
    }
    return -1;
}

int main() {
    int arr[] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
    int n = sizeof(arr) / sizeof(arr[0]);
    int x = 5;
    int result = ternarySearch(arr, 0, n - 1, x);

    if (result != -1)
        std::cout << "Element " << x << " found at index " << result << std::endl;
    else
        std::cout << "Element " << x << " not found in the array" << std::endl;

    return 0;
}
```

## Rust

```rs
fn ternary_search(arr: &[i32], left: usize, right: usize, x: i32) -> Option<usize> {
    if right >= left {
        let mid1 = left + (right - left) / 3;
        let mid2 = right - (right - left) / 3;

        if arr[mid1] == x {
            return Some(mid1);
        }
        if arr[mid2] == x {
            return Some(mid2);
        }

        if x < arr[mid1] {
            return ternary_search(arr, left, mid1 - 1, x);
        } else if x > arr[mid2] {
            return ternary_search(arr, mid2 + 1, right, x);
        } else {
            return ternary_search(arr, mid1 + 1, mid2 - 1, x);
        }
    }
    None
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let x = 5;
    let result = ternary_search(&arr, 0, arr.len() - 1, x);

    match result {
        Some(index) => println!("Element {} found at index {}", x, index),
        None => println!("Element {} not found in the array", x),
    }
}
```

## Python

```py
def ternary_search(arr, left, right, x):
    if right >= left:
        mid1 = left + (right - left) // 3
        mid2 = right - (right - left) // 3

        if arr[mid1] == x:
            return mid1
        if arr[mid2] == x:
            return mid2

        if x < arr[mid1]:
            return ternary_search(arr, left, mid1 - 1, x)
        elif x > arr[mid2]:
            return ternary_search(arr, mid2 + 1, right, x)
        else:
            return ternary_search(arr, mid1 + 1, mid2 - 1, x)
    return -1

arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
x = 5
result = ternary_search(arr, 0, len(arr) - 1, x)

if result != -1:
    print(f"Element {x} found at index {result}")
else:
    print(f"Element {x} not found in the array")
```

## Swift

```swift
func ternarySearch(arr: [Int], left: Int, right: Int, x: Int) -> Int? {
    if right >= left {
        let mid1 = left + (right - left) / 3
        let mid2 = right - (right - left) / 3

        if arr[mid1] == x {
            return mid1
        }
        if arr[mid2] == x {
            return mid2
        }

        if x < arr[mid1] {
            return ternarySearch(arr: arr, left: left, right: mid1 - 1, x: x)
        } else if x > arr[mid2] {
            return ternarySearch(arr: arr, left: mid2 + 1, right: right, x: x)
        } else {
            return ternarySearch(arr: arr, left: mid1 + 1, right: mid2 - 1, x: x)
        }
    }
    return nil
}

let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
let x = 5

if let result = ternarySearch(arr: arr, left: 0, right: arr.count - 1, x: x) {
    print("Element \(x) found at index \(result)")
} else {
    print("Element \(x) not found in the array")
}
```

## Kotlin

```kotlin
fun ternarySearch(arr: IntArray, left: Int, right: Int, x: Int): Int? {
    if (right >= left) {
        val mid1 = left + (right - left) / 3
        val mid2 = right - (right - left) / 3

        if (arr[mid1] == x) {
            return mid1
        }
        if (arr[mid2] == x) {
            return mid2
        }

        if (x < arr[mid1]) {
            return ternarySearch(arr, left, mid1 - 1, x)
        } else if (x > arr[mid2]) {
            return ternarySearch(arr, mid2 + 1, right, x)
        } else {
            return ternarySearch(arr, mid1 + 1, mid2 - 1, x)
        }
    }
    return null
}

fun main() {
    val arr = intArrayOf(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
    val x = 5

    val result = ternarySearch(arr, 0, arr.size - 1, x)
    if (result != null) {
        println("Element $x found at index $result")
    } else {
        println("Element $x not found in the array")
    }
}
```

## Go

```go
package main

import (
	"fmt"
	"math"
)

func ternarySearch(arr []int, left, right, x int) int {
	if right >= left {
		mid1 := left + (right-left)/3
		mid2 := right - (right-left)/3

		if arr[mid1] == x {
			return mid1
		}
		if arr[mid2] == x {
			return mid2
		}

		if x < arr[mid1] {
			return ternarySearch(arr, left, mid1-1, x)
		} else if x > arr[mid2] {
			return ternarySearch(arr, mid2+1, right, x)
		} else {
			return ternarySearch(arr, mid1+1, mid2-1, x)
		}
	}
	return -1
}

func main() {
	arr := []int{1, 2, 3, 4, 5, 6, 7, 8, 9, 10}
	x := 5

	result := ternarySearch(arr, 0, len(arr)-1, x)
	if result != -1 {
		fmt.Printf("Element %d found at index %d\n", x, result)
	} else {
		fmt.Printf("Element %d not found in the array\n", x)
	}
}
```

## Java

```java
public class TernarySearch {

    public static int ternarySearch(int arr[], int left, int right, int x) {
        if (right >= left) {
            int mid1 = left + (right - left) / 3;
            int mid2 = right - (right - left) / 3;

            if (arr[mid1] == x) {
                return mid1;
            }
            if (arr[mid2] == x) {
                return mid2;
            }

            if (x < arr[mid1]) {
                return ternarySearch(arr, left, mid1 - 1, x);
            } else if (x > arr[mid2]) {
                return ternarySearch(arr, mid2 + 1, right, x);
            } else {
                return ternarySearch(arr, mid1 + 1, mid2 - 1, x);
            }
        }
        return -1;
    }

    public static void main(String[] args) {
        int arr[] = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
        int x = 5;
        int result = ternarySearch(arr, 0, arr.length - 1, x);

        if (result != -1) {
            System.out.println("Element " + x + " found at index " + result);
        } else {
            System.out.println("Element " + x + " not found in the array");
        }
    }
}
```

## JavaScript

```js
function ternarySearch(arr, left, right, x) {
    if (right >= left) {
        let mid1 = left + Math.floor((right - left) / 3);
        let mid2 = right - Math.floor((right - left) / 3);

        if (arr[mid1] === x) {
            return mid1;
        }
        if (arr[mid2] === x) {
            return mid2;
        }

        if (x < arr[mid1]) {
            return ternarySearch(arr, left, mid1 - 1, x);
        } else if (x > arr[mid2]) {
            return ternarySearch(arr, mid2 + 1, right, x);
        } else {
            return ternarySearch(arr, mid1 + 1, mid2 - 1, x);
        }
    }
    return -1;
}

const arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
const x = 5;
const result = ternarySearch(arr, 0, arr.length - 1, x);

if (result !== -1) {
    console.log(`Element ${x} found at index ${result}`);
} else {
    console.log(`Element ${x} not found in the array`);
}
```

## C#

```cs
using System;

public class TernarySearch
{
    public static int TernarySearchFunc(int[] arr, int left, int right, int x)
    {
        if (right >= left)
        {
            int mid1 = left + (right - left) / 3;
            int mid2 = right - (right - left) / 3;

            if (arr[mid1] == x)
                return mid1;
            if (arr[mid2] == x)
                return mid2;

            if (x < arr[mid1])
                return TernarySearchFunc(arr, left, mid1 - 1, x);
            else if (x > arr[mid2])
                return TernarySearchFunc(arr, mid2 + 1, right, x);
            else
                return TernarySearchFunc(arr, mid1 + 1, mid2 - 1, x);
        }
        return -1;
    }

    public static void Main()
    {
        int[] arr = { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };
        int x = 5;

        int result = TernarySearchFunc(arr, 0, arr.Length - 1, x);

        if (result != -1)
            Console.WriteLine($"Element {x} found at index {result}");
        else
            Console.WriteLine($"Element {x} not found in the array");
    }
}
```

## Ruby

```rb
def ternary_search(arr, left, right, x)
  if right >= left
    mid1 = left + (right - left) / 3
    mid2 = right - (right - left) / 3

    return mid1 if arr[mid1] == x
    return mid2 if arr[mid2] == x

    if x < arr[mid1]
      ternary_search(arr, left, mid1 - 1, x)
    elsif x > arr[mid2]
      ternary_search(arr, mid2 + 1, right, x)
    else
      ternary_search(arr, mid1 + 1, mid2 - 1, x)
    end
  else
    -1
  end
end

arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
x = 5
result = ternary_search(arr, 0, arr.length - 1, x)

if result != -1
  puts "Element #{x} found at index #{result}"
else
  puts "Element #{x} not found in the array"
end
```

