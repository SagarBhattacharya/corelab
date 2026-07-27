# Stack

## Introduction

A generic stack implementation following the Last-In, First-Out (LIFO) principle.

Elements are inserted and removed exclusively from the top of the stack, making it 
suitable for problems that require processing the most recently added item first.

## Design

- Generic over any type `T`.
- Implemented as a thin abstraction over Rust's `Vec<T>`.
- The end of the underlying vector represents the top of the stack.
- Leverages the amortized constant-time insertion and removal provided by the dynamic vector.

## Implementation Details

The stack delegates storage management to the underlying `Vec<T>`.

- `push()` appends an element to the end of the vector.
- `pop()` removes and returns the last element.
- `peek()` returns an immutable reference to the top element without removing it.

By storing the stack top at the end of the vector, all core operations avoid shifting elements and execute in constant amortized time.

## Complexity

| Operation  |     Complexity     |
|------------|:------------------:|
| `push`     | **O(1)** amortized |
| `pop`      | **O(1)** amortized |
| `peek`     |      **O(1)**      |
| `len`      |      **O(1)**      |
| `is_empty` |      **O(1)**      |