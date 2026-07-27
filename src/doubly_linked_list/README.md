# Doubly Linked List

## Introduction

A generic doubly linked list implemented using manually managed heap-allocated nodes.

Each node stores links to both its predecessor and successor, allowing efficient 
insertion, removal, and traversal from either end of the list. Unlike the singly 
linked list implementation, this version uses raw pointers to express the ownership 
model of the data structure more directly while exposing a completely safe public API.

## Design

- Generic over any type `T`.
- Stores each node independently on the heap.
- Uses `NonNull<Node<T>>` for forward and backward links.
- The list exclusively owns every node.
- Maintains pointers to both the head and tail nodes.
- Traverses from the nearest end when locating an indexed position.
- Encapsulates all pointer manipulation within a small `unsafe` implementation.

The implementation maintains the following invariants:

- `head.is_none() == tail.is_none() == (len == 0)`
- `head.prev == None`
- `tail.next == None`
- Every node is reachable from the head by following `next`.
- Every node is reachable from the tail by following `prev`.
- Adjacent nodes maintain consistent `prev` and `next` links.
- `len` equals the number of nodes in the list.
- Every stored pointer references a valid heap allocation owned by the list.

## Implementation Details

Nodes are allocated using `Box::into_raw()` and internally represented as `NonNull<Node<T>>`, 
separating ownership from connectivity. Ownership remains with the list, while nodes 
merely reference one another through raw pointers.

Insertion and removal update the neighboring links while preserving the structural 
invariants of the list. Removed nodes are reclaimed using `Box::from_raw()`, allowing 
ownership to be safely recovered before the node is destroyed.

Although the implementation relies on manual pointer manipulation, all `unsafe` operations 
are confined to a small internal implementation, leaving the public interface entirely safe.

## Complexity

| Operation      | Complexity |
|----------------|:----------:|
| `push_front`   |  **O(1)**  |
| `push_back`    |  **O(1)**  |
| `pop_front`    |  **O(1)**  |
| `pop_back`     |  **O(1)**  |
| `insert`       |  **O(n)**  |
| `remove`       |  **O(n)**  |
| `len`          |  **O(1)**  |
| `is_empty`     |  **O(1)**  |