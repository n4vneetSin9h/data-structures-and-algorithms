# Interpolation Search Algorithm

## Description

Interpolation Search is an efficient algorithm used for finding a specific element within a sorted array or list of elements. It estimates the position of the target element based on the assumption of a uniform distribution of values and uses this estimation to narrow down the search range.

## Steps

1. Start with defining pointers `low` and `high` pointing to the start and end of the array, respectively.
2. Estimate the position of the target using interpolation formula: `pos = low + ((target - arr[low]) * (high - low)) / (arr[high] - arr[low])`.
3. Compare the estimated element at the calculated position with the target:
   - If they match, the target is found, and we return the position.
   - If the target is greater than the estimated element, update `low = pos + 1` to search in the right portion.
   - If the target is smaller than the estimated element, update `high = pos - 1` to search in the left portion.
4. Repeat steps 2 and 3 until `low` is less than or equal to `high`, indicating that the target is not present in the array.

## Pseudo Code

```plaintext
InterpolationSearch(array, target):
    low = 0
    high = length(array) - 1

    while low <= high and target >= array[low] and target <= array[high]:
        pos = low + ((target - array[low]) * (high - low)) / (array[high] - array[low])

        if array[pos] == target:
            return pos
        else if array[pos] < target:
            low = pos + 1
        else:
            high = pos - 1

    return -1  // Target not found
```
# Interpolation Search Algorithm: Time and Space Complexity Analysis

## Time Complexity Analysis

The time complexity of the Interpolation Search Algorithm is analyzed based on the number of comparisons made during the search. Let's denote the length of the array as `n`.

- **Best Case**: O(1)
  - The best case occurs when the target element is found at the estimated position in the first comparison.

- **Worst Case**: O(n)
  - The worst case occurs when the array elements are not uniformly distributed, resulting in a large number of comparisons.

- **Average Case**: O(log (log n))
  - The average case can be approximated to O(log(log n)) for uniformly distributed elements.

## Space Complexity Analysis

The space complexity of the Interpolation Search Algorithm is constant, denoted as O(1). This is because the algorithm uses a fixed amount of additional memory to store variables, regardless of the size of the input array.

## Functionality

The `interpolationSearch` function takes a sorted array of a numeric type and a target element to search for.
It uses an interpolation formula to estimate the position of the target element and adjusts the search range accordingly.
You can modify the `sortedArray` and `target` in the example usage to test different scenarios.

# Code

## C++

```cpp
#include <iostream>
#include <vector>

int interpolationSearch(const std::vector<int>& arr, int target) {
    int low = 0;
    int high = arr.size() - 1;

    while (low <= high && target >= arr[low] && target <= arr[high]) {
        int pos = low + ((target - arr[low]) * (high - low)) / (arr[high] - arr[low]);

        if (arr[pos] == target)
            return pos;
        else if (arr[pos] < target)
            low = pos + 1;
        else
            high = pos - 1;
    }

    return -1;  // Target not found
}

int main() {
    std::vector<int> arr = {2, 5, 8, 12, 16, 23, 38, 45, 56, 72};
    int target = 23;

    int result = interpolationSearch(arr, target);

    if (result != -1)
        std::cout << "Element " << target << " found at index " << result << "\n";
    else
        std::cout << "Element " << target << " not found in the array.\n";

    return 0;
}
```

## Rust

```rs
fn interpolation_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high && target >= arr[low] && target <= arr[high] {
        let pos = low + ((target - arr[low]) * (high as i32 - low as i32)) as usize / (arr[high] - arr[low]) as usize;

        if arr[pos] == target {
            return Some(pos);
        } else if arr[pos] < target {
            low = pos + 1;
        } else {
            high = pos - 1;
        }
    }

    None
}

fn main() {
    let arr = vec![2, 5, 8, 12, 16, 23, 38, 45, 56, 72];
    let target = 23;

    match interpolation_search(&arr, target) {
        Some(index) => println!("Element {} found at index {}", target, index),
        None => println!("Element {} not found in the array", target),
    }
}
```

## Python

```py
def interpolation_search(arr, target):
    low = 0
    high = len(arr) - 1

    while low <= high and target >= arr[low] and target <= arr[high]:
        pos = low + int(((target - arr[low]) * (high - low)) / (arr[high] - arr[low]))

        if arr[pos] == target:
            return pos
        elif arr[pos] < target:
            low = pos + 1
        else:
            high = pos - 1

    return -1

arr = [2, 5, 8, 12, 16, 23, 38, 45, 56, 72]
target = 23

result = interpolation_search(arr, target)

if result != -1:
    print(f"Element {target} found at index {result}")
else:
    print(f"Element {target} not found in the array")
```

## Swift

```swift
func interpolationSearch(_ arr: [Int], target: Int) -> Int? {
    var low = 0
    var high = arr.count - 1

    while low <= high && target >= arr[low] && target <= arr[high] {
        let pos = low + ((target - arr[low]) * (high - low)) / (arr[high] - arr[low])

        if arr[pos] == target {
            return pos
        } else if arr[pos] < target {
            low = pos + 1
        } else {
            high = pos - 1
        }
    }

    return nil
}

let arr = [2, 5, 8, 12, 16, 23, 38, 45, 56, 72]
let target = 23

if let result = interpolationSearch(arr, target) {
    print("Element \(target) found at index \(result)")
} else {
    print("Element \(target) not found in the array")
}
```

## Kotlin

```kotlin
fun interpolationSearch(arr: List<Int>, target: Int): Int {
    var low = 0
    var high = arr.size - 1

    while (low <= high && target >= arr[low] && target <= arr[high]) {
        val pos = low + (((target - arr[low]) * (high - low)) / (arr[high] - arr[low]))

        if (arr[pos] == target) {
            return pos
        } else if (arr[pos] < target) {
            low = pos + 1
        } else {
            high = pos - 1
        }
    }

    return -1
}

fun main() {
    val arr = listOf(2, 5, 8, 12, 16, 23, 38, 45, 56, 72)
    val target = 23

    val result = interpolationSearch(arr, target)

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

func interpolationSearch(arr []int, target int) int {
	low := 0
	high := len(arr) - 1

	for low <= high && target >= arr[low] && target <= arr[high] {
		pos := low + ((target - arr[low]) * (high - low)) / (arr[high] - arr[low])

		if arr[pos] == target {
			return pos
		} else if arr[pos] < target {
			low = pos + 1
		} else {
			high = pos - 1
		}
	}

	return -1
}

func main() {
	arr := []int{2, 5, 8, 12, 16, 23, 38, 45, 56, 72}
	target := 23

	result := interpolationSearch(arr, target)

	if result != -1 {
		fmt.Printf("Element %d found at index %d\n", target, result)
	} else {
		fmt.Printf("Element %d not found in the array\n", target)
	}
}
```

## Java

```java
public class InterpolationSearch {
    public static int interpolationSearch(int[] arr, int target) {
        int low = 0;
        int high = arr.length - 1;

        while (low <= high && target >= arr[low] && target <= arr[high]) {
            int pos = low + ((target - arr[low]) * (high - low)) / (arr[high] - arr[low]);

            if (arr[pos] == target) {
                return pos;
            } else if (arr[pos] < target) {
                low = pos + 1;
            } else {
                high = pos - 1;
            }
        }

        return -1;
    }

    public static void main(String[] args) {
        int[] arr = {2, 5, 8, 12, 16, 23, 38, 45, 56, 72};
        int target = 23;

        int result = interpolationSearch(arr, target);

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
function interpolationSearch(arr, target) {
    let low = 0;
    let high = arr.length - 1;

    while (low <= high && target >= arr[low] && target <= arr[high]) {
        const pos = low + Math.floor(((target - arr[low]) * (high - low)) / (arr[high] - arr[low]));

        if (arr[pos] === target) {
            return pos;
        } else if (arr[pos] < target) {
            low = pos + 1;
        } else {
            high = pos - 1;
        }
    }

    return -1;
}

const arr = [2, 5, 8, 12, 16, 23, 38, 45, 56, 72];
const target = 23;
const result = interpolationSearch(arr, target);

if (result !== -1) {
    console.log(`Element ${target} found at index ${result}`);
} else {
    console.log(`Element ${target} not found in the array`);
}
```

## C#

```cs
using System;

public class InterpolationSearch
{
    public static int InterpolationSearchFunc(int[] arr, int target)
    {
        int low = 0;
        int high = arr.Length - 1;

        while (low <= high && target >= arr[low] && target <= arr[high])
        {
            int pos = low + ((target - arr[low]) * (high - low)) / (arr[high] - arr[low]);

            if (arr[pos] == target)
                return pos;

            if (arr[pos] < target)
                low = pos + 1;
            else
                high = pos - 1;
        }

        return -1;
    }

    public static void Main(string[] args)
    {
        int[] arr = { 2, 5, 8, 12, 16, 23, 38, 45, 56, 72 };
        int target = 23;

        int result = InterpolationSearchFunc(arr, target);

        if (result != -1)
            Console.WriteLine("Element " + target + " found at index " + result);
        else
            Console.WriteLine("Element " + target + " not found in the array");
    }
}
```

## Ruby

```rb
def interpolation_search(arr, target)
  low = 0
  high = arr.length - 1

  while low <= high && target >= arr[low] && target <= arr[high]
    pos = low + ((target - arr[low]) * (high - low)) / (arr[high] - arr[low])

    if arr[pos] == target
      return pos
    elsif arr[pos] < target
      low = pos + 1
    else
      high = pos - 1
    end
  end

  -1
end

arr = [2, 5, 8, 12, 16, 23, 38, 45, 56, 72]
target = 23

result = interpolation_search(arr, target)

if result != -1
  puts "Element #{target} found at index #{result}"
else
  puts "Element #{target} not found in the array"
end
```

