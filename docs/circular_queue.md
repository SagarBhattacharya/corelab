# Queue (Circular Buffer)

## Overview

A Queue is a linear data structure that follows the **First-In, First-Out (FIFO)** principle. 
Elements are inserted at the rear and removed from the front.

This implementation uses a fixed-size circular buffer to provide constant-time 
enqueue and dequeue operations without shifting elements.

---

## Requirements

- Capacity is determined at compile time using const generics.
- Uses `MaybeUninit<T>` to safely manage partially initialized memory.
- Stores elements in contiguous memory.
- Supports constant-time insertion and removal.

---

## Memory Layout

```
Capacity = 8

+----+----+----+----+----+----+----+----+
|    | B  | C  | D  |    |    |    | A  |
+----+----+----+----+----+----+----+----+
        ↑                   ↑
      head                tail
```

- `head` points to the first element.
- `tail` points to the next insertion position.
- Indices wrap around the buffer using modulo arithmetic.

---

## Invariants

The implementation maintains the following invariants:

- Elements are stored in the range defined by `head`, `tail`, and `len`.
- `head` always points to the front element.
- `tail` always points to the next insertion position.
- `len <= capacity`.
- Elements outside the logical queue remain uninitialized.

---

## Operations

| Operation | Complexity |
|-----------|-----------:|
| Enqueue   |       O(1) |
| Dequeue   |       O(1) |
| Front     |       O(1) |
| Back      |       O(1) |
| Clear     |       O(n) |

---

## Design

- Uses a circular buffer to avoid shifting elements.
- Tracks the queue using `head`, `tail`, and `len`.
- Uses modulo arithmetic to wrap indices.
- Implements `Drop` to correctly destroy initialized elements.

---

## Advantages

- Constant-time enqueue and dequeue.
- Cache-friendly contiguous storage.
- No element shifting.
- Predictable memory usage.

---

## Limitations

- Fixed capacity.
- Cannot grow dynamically.
- Capacity must be greater than zero.

---

## Applications

- Task scheduling.
- Producer-consumer systems.
- Breadth-First Search (BFS).
- Network packet buffering.
- Operating system scheduling.