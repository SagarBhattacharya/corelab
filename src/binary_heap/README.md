# Binary Heap (Max Heap)

## Introduction

A generic max binary heap implemented using a contiguous dynamic array.

A binary heap is a complete binary tree that satisfies the heap property,
where every parent node is greater than or equal to its children. By storing
the tree in an array, parent and child relationships are computed using indices
instead of explicit pointers, providing efficient logarithmic insertion and removal.

## Design

- Generic over any type implementing `Ord`.
- Stores elements contiguously using `Vec<T>`.
- Represents the heap as a complete binary tree mapped onto an array.
- Inserts new elements at the end of the array and restores the heap property using sift-up.
- Removes the root by replacing it with the last element and restoring the heap property using sift-down.

The implementation maintains the following invariants:

- The underlying tree is always complete.
- Every parent node is greater than or equal to its children.
- The maximum element is always stored at the root.

## Implementation Details

The binary tree is represented implicitly using array indices:

```text
parent(i) = (i - 1) / 2
left(i)   = 2 * i + 1
right(i)  = 2 * i + 2
```

Insertion appends the new element to the end of the array before repeatedly exchanging it
with its parent until the heap property is restored.

Removal exchanges the root with the final element, removes the last element, and repeatedly
swaps the new root with its larger child until the heap property is satisfied.

By storing the heap in contiguous memory, the implementation avoids explicit tree
nodes while remaining cache-friendly.

## Complexity

| Operation  |  Complexity  |
|------------|:------------:|
| `insert`   | **O(log n)** |
| `extract`  | **O(log n)** |
| `peek`     |   **O(1)**   |
| `len`      |   **O(1)**   |
| `is_empty` |   **O(1)**   |