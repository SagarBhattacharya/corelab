# Binary Search

## Overview
Binary Search efficiently finds an element in a sorted collection by repeatedly dividing the search range in half.
Instead of checking every element, it eliminates half of the remaining search space after each comparison.

---

## Requirements
- Input must be sorted.
- Elements must implement `Ord`.
- Works on immutable slices `(&[T])`.

---

## Algorithm
1. Maintain a search range `[low, high)`.
2. Compute the middle index.
3. Compare the middle element with the target.
4. Narrow the search range.
5. Repeat until the range is empty.

---

## Design
- Uses an exclusive upper bound to avoid index underflow.
- Operates on slices instead of `Vec<T>`.
- Returns `Option<usize>` instead of a sentinel value.

---

## Complexity

| Metric | Complexity   |
|--------|--------------|
| Time   | **O(log n)** |
| Space  | **O(1)**     |

---

## Advantages
- Extremely fast on sorted data.
- Constant memory usage.
- Generic over any ordered type.

---

## Limitations
- Requires sorted input.
- Not efficient for linked lists.
- Cannot search unsorted collections.