# Binary Search

## Introduction

A generic implementation of the binary search algorithm for sorted slices.

Binary Search locates a target element by repeatedly dividing the search 
interval in half, reducing the search space after each comparison. It is 
one of the most fundamental searching algorithms and provides 
logarithmic-time lookups on sorted data.

## Design

- Implemented as a generic function over any type implementing `Ord`.
- Operates directly on immutable slices (`&[T]`) rather than concrete collection types.
- Uses an exclusive upper bound (`[low, high)`) to simplify boundary handling and prevent index underflow.
- Returns `Option<usize>` to explicitly represent successful and unsuccessful searches.

## Implementation Details

The algorithm maintains a search interval defined by two indices, `low` and `high`.

At each iteration:

1. Compute the middle index.
2. Compare the target with the middle element.
3. Discard the half of the search interval that cannot contain the target.
4. Repeat until the element is found or the search interval becomes empty.

The implementation is iterative, requiring only constant additional memory while avoiding recursive function calls.

## Complexity

| Metric |  Complexity  |
|--------|:------------:|
| Time   | **O(log n)** |
| Space  |   **O(1)**   |