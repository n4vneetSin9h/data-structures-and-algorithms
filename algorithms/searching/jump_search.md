# Jump Search Algorithm

## Description

Jump Search is a searching algorithm used to find a specific element within a **sorted array**. It works by making jumps through the array, aiming to reduce the number of elements to be checked by performing a linear search within the reduced range.

## Steps

1. Start at the beginning of the array.
2. Determine the **jump size** (usually √n, where n is the length of the array).
3. Jump to the next element using the jump size until you find an element greater than the target or reach the end of the array.
4. Perform a **linear search** in the block from the previous step to find the target.

## Pseudo Code

```plaintext
JumpSearch(array, target):
    n = length(array)
    jump = sqrt(n)  // Set the jump size

    prev = 0
    while prev < n and array[prev] < target:
        prev = min(prev + jump, n)

    // Perform a linear search in the block [prev - jump, prev]
    for i from prev - jump to min(prev, n) - 1:
        if array[i] equals target:
            return i  // Target found at index i

    return -1  // Target not found
```

# Jump Search Algorithm: Time and Space Complexity Analysis

## Time Complexity Analysis

- **Best Case**: O(1)

- **Worst Case**: O(√n)

- **Average Case**: O(√n)

## Space Complexity Analysis

The space complexity of the Jump Search Algorithm is constant, denoted as O(1).

# Code

## C++

```cpp
#include <iostream>
#include <cmath>

int jumpSearch(int arr[], int n, int x) {
    int step = sqrt(n);
    int prev = 0;
    while (arr[std::min(step, n) - 1] < x) {
        prev = step;
        step += sqrt(n);
        if (prev >= n)
            return -1;
    }
    while (arr[prev] < x) {
        prev++;
        if (prev == std::min(step, n))
            return -1;
    }
    if (arr[prev] == x)
        return prev;
    return -1;
}

int main() {
    int arr[] = {1, 3, 5, 7, 9, 11, 13, 15, 17, 19};
    int n = sizeof(arr) / sizeof(arr[0]);
    int x = 7;
    int index = jumpSearch(arr, n, x);
    if (index != -1)
        std::cout << "Element " << x << " found at index " << index << std::endl;
    else
        std::cout << "Element " << x << " not found in the array" << std::endl;
    return 0;
}
```

## Rust

```rs
fn jump_search(arr: &[i32], x: i32) -> Option<usize> {
    let n = arr.len();
    let step = (n as f64).sqrt() as usize;
    let mut prev = 0;

    while arr[usize::min(step, n) - 1] < x {
        prev = step;
        step += (n as f64).sqrt() as usize;
        if prev >= n {
            return None;
        }
    }

    while arr[prev] < x {
        prev += 1;
        if prev == usize::min(step, n) {
            return None;
        }
    }

    if arr[prev] == x {
        Some(prev)
    } else {
        None
    }
}

fn main() {
    let arr = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    let x = 7;
    if let Some(index) = jump_search(&arr, x) {
        println!("Element {} found at index {}", x, index);
    } else {
        println!("Element {} not found in the array", x);
    }
}
```

## Python

```py
import math

def jump_search(arr, x):
    n = len(arr)
    step = int(math.sqrt(n))
    prev = 0

    while arr[min(step, n) - 1] < x:
        prev = step
        step += int(math.sqrt(n))
        if prev >= n:
            return -1

    while arr[prev] < x:
        prev += 1
        if prev == min(step, n):
            return -1

    if arr[prev] == x:
        return prev

    return -1

arr = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19]
x = 7
index = jump_search(arr, x)

if index != -1:
    print(f"Element {x} found at index {index}")
else:
    print(f"Element {x} not found in the array")
```

## Swift

```swift
func jumpSearch(arr: [Int], x: Int) -> Int? {
    let n = arr.count
    let step = Int(sqrt(Double(n)))
    var prev = 0

    while arr[min(step, n) - 1] < x {
        prev = step
        step += Int(sqrt(Double(n)))
        if prev >= n {
            return nil
        }
    }

    while arr[prev] < x {
        prev += 1
        if prev == min(step, n) {
            return nil
        }
    }

    if arr[prev] == x {
        return prev
    }

    return nil
}

let arr = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19]
let x = 7

if let index = jumpSearch(arr: arr, x: x) {
    print("Element \(x) found at index \(index)")
} else {
    print("Element \(x) not found in the array")
}
```

## Kotlin

```kotlin
import kotlin.math.min
import kotlin.math.sqrt

fun jumpSearch(arr: IntArray, x: Int): Int? {
    val n = arr.size
    val step = sqrt(n.toDouble()).toInt()
    var prev = 0

    while (arr[min(step, n) - 1] < x) {
        prev = step
        step += sqrt(n.toDouble()).toInt()
        if (prev >= n) {
            return null
        }
    }

    while (arr[prev] < x) {
        prev++
        if (prev == min(step, n)) {
            return null
        }
    }

    return if (arr[prev] == x) prev else null
}

fun main() {
    val arr = intArrayOf(1, 3, 5, 7, 9, 11, 13, 15, 17, 19)
    val x = 7

    val index = jumpSearch(arr, x)
    if (index != null) {
        println("Element $x found at index $index")
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

func jumpSearch(arr []int, x int) int {
	n := len(arr)
	step := int(math.Sqrt(float64(n)))
	prev := 0

	for arr[min(step, n)-1] < x {
		prev = step
		step += int(math.Sqrt(float64(n)))
		if prev >= n {
			return -1
		}
	}

	for arr[prev] < x {
		prev++
		if prev == min(step, n) {
			return -1
		}
	}

	if arr[prev] == x {
		return prev
	}

	return -1
}

func main() {
	arr := []int{1, 3, 5, 7, 9, 11, 13, 15, 17, 19}
	x := 7

	index := jumpSearch(arr, x)
	if index != -1 {
		fmt.Printf("Element %d found at index %d\n", x, index)
	} else {
		fmt.Printf("Element %d not found in the array\n", x)
	}
}
```

## Java

```java
import java.util.Arrays;

public class JumpSearch {

    public static int jumpSearch(int[] arr, int x) {
        int n = arr.length;
        int step = (int) Math.sqrt(n);
        int prev = 0;

        while (arr[Math.min(step, n) - 1] < x) {
            prev = step;
            step += (int) Math.sqrt(n);
            if (prev >= n)
                return -1;
        }

        while (arr[prev] < x) {
            prev++;
            if (prev == Math.min(step, n))
                return -1;
        }

        if (arr[prev] == x)
            return prev;

        return -1;
    }

    public static void main(String[] args) {
        int[] arr = {1, 3, 5, 7, 9, 11, 13, 15, 17, 19};
        int x = 7;
        int index = jumpSearch(arr, x);
        if (index != -1)
            System.out.println("Element " + x + " found at index " + index);
        else
            System.out.println("Element " + x + " not found in the array");
    }
}
```

## JavaScript

```js
function jumpSearch(arr, x) {
    const n = arr.length;
    const step = Math.floor(Math.sqrt(n));
    let prev = 0;

    while (arr[Math.min(step, n) - 1] < x) {
        prev = step;
        step += Math.floor(Math.sqrt(n));
        if (prev >= n)
            return -1;
    }

    while (arr[prev] < x) {
        prev++;
        if (prev === Math.min(step, n))
            return -1;
    }

    if (arr[prev] === x)
        return prev;

    return -1;
}

const arr = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
const x = 7;
const index = jumpSearch(arr, x);

if (index !== -1)
    console.log(`Element ${x} found at index ${index}`);
else
    console.log(`Element ${x} not found in the array`);
```

## C#

```cs
using System;

public class JumpSearch
{
    public static int JumpSearch(int[] arr, int x)
    {
        int n = arr.Length;
        int step = (int)Math.Floor(Math.Sqrt(n));
        int prev = 0;

        while (arr[Math.Min(step, n) - 1] < x)
        {
            prev = step;
            step += (int)Math.Floor(Math.Sqrt(n));
            if (prev >= n)
                return -1;
        }

        while (arr[prev] < x)
        {
            prev++;
            if (prev == Math.Min(step, n))
                return -1;
        }

        if (arr[prev] == x)
            return prev;
        else
            return -1;
    }

    public static void Main()
    {
        int[] arr = { 1, 3, 5, 7, 9, 11, 13, 15, 17, 19 };
        int x = 7;
        int index = JumpSearch(arr, x);
        if (index != -1)
            Console.WriteLine($"Element {x} found at index {index}");
        else
            Console.WriteLine($"Element {x} not found in the array");
    }
}
```

## Ruby

```rb
def jump_search(arr, x)
  n = arr.length
  step = Math.sqrt(n).to_i
  prev = 0

  while arr[[step, n].min - 1] < x
    prev = step
    step += Math.sqrt(n).to_i
    return -1 if prev >= n
  end

  while arr[prev] < x
    prev += 1
    return -1 if prev == [step, n].min
  end

  return prev if arr[prev] == x
  return -1
end

arr = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19]
x = 7
index = jump_search(arr, x)

if index != -1
  puts "Element #{x} found at index #{index}"
else
  puts "Element #{x} not found in the array"
end
```

