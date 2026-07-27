# Singly Linked List

## Overview

A Singly Linked List is a linear data structure where each node stores a value and a 
pointer to the next node. Unlike contiguous containers such as vectors, elements 
are allocated independently and linked together through pointers.

This implementation uses heap-allocated nodes with `Box<T>` to provide 
safe ownership without requiring manual memory management.

---

## Requirements

- Supports generic element types.
- Stores each node independently on the heap.
- Uses `Box<Node<T>>` for exclusive ownership.
- Maintains a single forward link between adjacent nodes.

---

## Representation

```
head
 │
 ▼
+-----+-----+     +-----+-----+     +-----+------+
|  A  |  ●──┼────►|  B  |  ●──┼────►|  C  | None |
+-----+-----+     +-----+-----+     +-----+------+
```

Each node owns its successor, forming a chain that terminates with `None`.

---

## Invariant

The list maintains the following properties:

- `head` owns the first node.
- Each node exclusively owns its successor.
- The final node points to `None`.
- `len` equals the number of reachable nodes.
- The list contains no cycles.

---

## Operations

| Operation | Complexity |
|-----------|-----------:|
| Insert    |       O(n) |
| Remove    |       O(n) |
| Get       |       O(n) |
| Get Mut   |       O(n) |
| Len       |       O(1) |
| Is Empty  |       O(1) |

---

## Design

- Uses heap-allocated nodes linked through `Option<Box<Node<T>>>`.
- Traverses the list to locate insertion and removal positions.
- Performs structural modifications by manipulating ownership of links.
- Uses `Option::take()` to move ownership without cloning.

---

## Advantages

- Dynamic size.
- Constant-time insertion and removal at the front.
- Does not require contiguous memory.
- Safe ownership without `unsafe` code.

---

## Limitations

- Sequential access only.
- Linear-time lookup by index.
- Poor cache locality compared to contiguous containers.

---

## Applications

- Stack implementation.
- Adjacency lists in graphs.
- Hash table chaining.
- Free lists in memory allocators.
- Undo/history structures.