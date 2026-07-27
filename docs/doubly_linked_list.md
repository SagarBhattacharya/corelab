# Doubly Linked List

A generic doubly linked list implemented using manually managed heap-allocated nodes.

Unlike the singly linked list, each node stores links to both its predecessor and successor, allowing efficient insertion, removal, and traversal from either end of the list.

This implementation intentionally uses `NonNull<T>` and encapsulated `unsafe` code instead of `Rc<RefCell<T>>`. A doubly linked list has a single logical owner—the list itself—making manual pointer management a more appropriate ownership model while keeping the public API entirely safe.

## Features

- Generic over any `T`
- O(1) insertion/removal at both ends
- O(1) insertion/removal once a node is located
- Bidirectional traversal with automatic head/tail optimization
- Safe public API with localized `unsafe` internals
- Automatic cleanup through `Drop`

## Operations

| Operation      | Complexity |
|----------------|:----------:|
| `push_front`   |    O(1)    |
| `push_back`    |    O(1)    |
| `pop_front`    |    O(1)    |
| `pop_back`     |    O(1)    |
| `insert`       |    O(n)    |
| `remove`       |    O(n)    |
| `is_empty`     |    O(1)    |
| `len`          |    O(1)    |

## Design

Each node stores:

- A value of type `T`
- A pointer to the previous node
- A pointer to the next node

The list maintains pointers to both the head and tail nodes, allowing traversal to begin from the nearest end when accessing an indexed position.

Memory is allocated using `Box::into_raw()` when inserting nodes and reclaimed using `Box::from_raw()` when removing them. Raw pointers are represented using `NonNull<Node<T>>`, while all unsafe operations are confined to a small internal implementation.

## Invariants

The implementation maintains the following invariants:

- `head.is_none() == tail.is_none() == (len == 0)`
- `head.prev == None`
- `tail.next == None`
- Every node is reachable from the head by following `next`
- Every node is reachable from the tail by following `prev`
- Adjacent nodes maintain consistent `prev` and `next` links
- `len` equals the number of nodes in the list
- Every stored pointer references a valid heap allocation owned by the list

---

## Advantages

- Dynamic size.
- Constant-time insertion and removal at the front and back.
- Does not require contiguous memory.
- Safe ownership without `unsafe` code.

---

## Limitations

- Sequential access only.
- Linear-time lookup by index.
- Poor cache locality compared to contiguous containers.
