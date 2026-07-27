# Linked List

## Introduction

A generic singly linked list implemented using heap-allocated nodes with exclusive ownership.

Unlike contiguous containers such as vectors, each element is allocated independently on 
the heap and connected through forward links. The implementation uses `Box<Node<T>>` to 
model ownership directly, allowing structural modifications without requiring `unsafe` code.

## Design

- Generic over any type `T`.
- Stores each node independently on the heap.
- Uses `Option<Box<Node<T>>>` to represent forward links.
- Maintains exclusive ownership of every node through the ownership chain.
- Performs insertion and removal by transferring ownership of links using `Option::take()`.

The implementation maintains the following invariants:

- `head` owns the first node.
- Each node exclusively owns its successor.
- The final node always points to `None`.
- `len` equals the number of reachable nodes.
- The list contains no cycles.

## Implementation Details

Each node owns the following node through a `Box`, forming a chain of
exclusive ownership from the head to the tail.

Traversal proceeds sequentially by following forward links until the 
requested position is reached. Structural modifications are performed by 
moving ownership of links rather than copying nodes, allowing insertion and 
removal without cloning or manual memory management.

Because ownership is represented directly by the pointer structure, the 
implementation remains entirely safe and requires no `unsafe` code.

## Complexity

| Operation  | Complexity |
|------------|:----------:|
| `insert`   |  **O(n)**  |
| `remove`   |  **O(n)**  |
| `get`      |  **O(n)**  |
| `get_mut`  |  **O(n)**  |
| `len`      |  **O(1)**  |
| `is_empty` |  **O(1)**  |