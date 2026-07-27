# Binary Heap (Max Heap)

## Overview

A Binary Heap is a complete binary tree that satisfies the **heap property**. 
In a **max heap**, every parent node is greater than or equal to its children.

This implementation stores the heap in a contiguous array, eliminating the need for explicit tree nodes or pointers.

---

## Requirements

- Elements must implement `Ord`.
- Uses a contiguous `Vec<T>` for storage.
- Maintains the max heap property after every insertion and removal.

---

## Memory Layout

```
Binary Tree

         90
       /    \
     70      60
    /  \    /  \
   40  30  20  10


Stored as Array

Index :  0   1   2   3   4   5   6
Value : 90  70  60  40  30  20  10
```

Node relationships are computed using array indices:

```
parent(i) = (i - 1) / 2

left(i)   = 2 * i + 1

right(i)  = 2 * i + 2
```

---

## Core Invariant

The heap maintains the following property:

- Every parent node is greater than or equal to both of its children.
- The tree is always complete.

This invariant is restored after every mutation using **sift-up** or **sift-down**.

---

## Operations

| Operation   | Complexity |
|-------------|-----------:|
| Insert      |   O(log n) |
| Extract Max |   O(log n) |
| Peek        |       O(1) |
| Len         |       O(1) |
| Is Empty    |       O(1) |

---

## Design

- Stores elements in contiguous memory.
- Inserts new elements at the end of the array.
- Uses **sift-up** to restore the heap property after insertion.
- Replaces the root with the last element during removal.
- Uses **sift-down** to restore the heap property after extraction.

---

## Advantages

- Efficient priority retrieval.
- Cache-friendly contiguous storage.
- No explicit tree nodes or pointers.
- Predictable logarithmic insertion and removal.

---

## Limitations

- Does not support efficient arbitrary element removal.
- Searching for an arbitrary element is O(n).
- Maintains partial ordering only, not a fully sorted sequence.

---

## Applications

- Priority Queues.
- Heap Sort.
- Task Scheduling.
- Event Simulation.
- Graph Algorithms (Dijkstra's and Prim's).