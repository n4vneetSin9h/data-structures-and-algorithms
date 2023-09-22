# Exponential Search Algorithm

## Description

Exponential Search is an efficient algorithm used to find a specific element within a sorted array or list of elements. It works by first finding a range in which the target element might exist, and then performing a binary search within that range.

## Steps

1. Start with defining a variable `bound` and initialize it to 1.
2. While the `bound` is less than the length of the array and the element at `bound` is less than the target:
3. Double the value of bound.
4. Perform a binary search in the range `[bound/2, min(bound, length of the array)]` to find the target element.

## Pseudo Code

```plaintext
ExponentialSearch(array, target):
    bound = 1

    // Find the range in which the target might exist
    while bound < length(array) and array[bound] < target:
        bound *= 2

    // Perform a binary search in the range
    left = bound / 2
    right = min(bound, length(array)) - 1

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
# Exponential Search Algorithm: Time and Space Complexity Analysis

## Time Complexity Analysis

- **Best Case**: O(1)

- **Worst Case**: O(log n)

- **Average Case**: O(log (log n))

## Space Complexity Analysis

The space complexity of the Exponential Search Algorithm is constant, denoted as O(1).

# Code

## C++

```cpp
#include <iostream>

double power(double base, int exponent) {
    if (exponent == 0)
        return 1;
    else if (exponent % 2 == 0)
        return power(base * base, exponent / 2);
    else
        return base * power(base * base, (exponent - 1) / 2);
}

int main() {
    double base = 2.0;
    int exponent = 10;
    std::cout << base << " raised to the power of " << exponent << " is " << power(base, exponent) << std::endl;
    return 0;
}
```

## Rust

```rs
fn power(base: f64, exponent: i32) -> f64 {
    if exponent == 0 {
        1.0
    } else if exponent % 2 == 0 {
        let half_power = power(base, exponent / 2);
        half_power * half_power
    } else {
        base * power(base, exponent - 1)
    }
}

fn main() {
    let base = 2.0;
    let exponent = 10;
    println!("{} raised to the power of {} is {}", base, exponent, power(base, exponent));
}
```

## Python

```py
def power(base, exponent):
    if exponent == 0:
        return 1
    elif exponent % 2 == 0:
        half_power = power(base, exponent // 2)
        return half_power * half_power
    else:
        return base * power(base, exponent - 1)

base = 2.0
exponent = 10
print(f"{base} raised to the power of {exponent} is {power(base, exponent)}")
```

## Swift

```swift
func power(_ base: Double, _ exponent: Int) -> Double {
    if exponent == 0 {
        return 1
    } else if exponent % 2 == 0 {
        let halfPower = power(base, exponent / 2)
        return halfPower * halfPower
    } else {
        return base * power(base, exponent - 1)
    }
}

let base = 2.0
let exponent = 10
print("\(base) raised to the power of \(exponent) is \(power(base, exponent))")
```

## Kotlin

```kotlin
fun power(base: Double, exponent: Int): Double {
    if (exponent == 0) {
        return 1.0
    } else if (exponent % 2 == 0) {
        val halfPower = power(base, exponent / 2)
        return halfPower * halfPower
    } else {
        return base * power(base, exponent - 1)
    }
}

fun main() {
    val base = 2.0
    val exponent = 10
    println("$base raised to the power of $exponent is ${power(base, exponent)}")
}
```

## Go

```go
package main

import "fmt"

func power(base float64, exponent int) float64 {
    if exponent == 0 {
        return 1.0
    } else if exponent%2 == 0 {
        halfPower := power(base, exponent/2)
        return halfPower * halfPower
    } else {
        return base * power(base, exponent-1)
    }
}

func main() {
    base := 2.0
    exponent := 10
    fmt.Printf("%.1f raised to the power of %d is %.1f\n", base, exponent, power(base, exponent))
}
```

## Java

```java
public class ExponentialAlgorithm {
    public static double power(double base, int exponent) {
        if (exponent == 0) {
            return 1.0;
        } else if (exponent % 2 == 0) {
            double halfPower = power(base, exponent / 2);
            return halfPower * halfPower;
        } else {
            return base * power(base, exponent - 1);
        }
    }

    public static void main(String[] args) {
        double base = 2.0;
        int exponent = 10;
        System.out.printf("%.1f raised to the power of %d is %.1f\n", base, exponent, power(base, exponent));
    }
}
```

## JavaScript

```js
function power(base, exponent) {
    if (exponent === 0) {
        return 1;
    } else if (exponent % 2 === 0) {
        const halfPower = power(base, exponent / 2);
        return halfPower * halfPower;
    } else {
        return base * power(base, exponent - 1);
    }
}

const base = 2.0;
const exponent = 10;
console.log(`${base} raised to the power of ${exponent} is ${power(base, exponent)}`);
```

## C#

```cs
using System;

public class Program
{
    public static double Power(double baseNum, int exponent)
    {
        if (exponent == 0)
            return 1;
        else if (exponent % 2 == 0)
        {
            double halfPower = Power(baseNum, exponent / 2);
            return halfPower * halfPower;
        }
        else
            return baseNum * Power(baseNum, exponent - 1);
    }

    public static void Main()
    {
        double baseNum = 2.0;
        int exponent = 10;
        Console.WriteLine($"{baseNum} raised to the power of {exponent} is {Power(baseNum, exponent)}");
    }
}
```

## Ruby

```rb
def power(base, exponent)
  if exponent == 0
    return 1
  elsif exponent.even?
    half_power = power(base, exponent / 2)
    return half_power * half_power
  else
    return base * power(base, exponent - 1)
  end
end

base = 2.0
exponent = 10
puts "#{base} raised to the power of #{exponent} is #{power(base, exponent)}"
```

