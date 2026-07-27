# Circular Queue

## Introduction

A generic fixed-capacity queue implemented using a circular buffer.

The queue follows the First-In, First-Out (FIFO) principle, where elements are 
inserted at the rear and removed from the front. By storing elements in a circular 
buffer, the implementation avoids shifting elements during insertion and removal 
while maintaining constant-time operations.

## Design

- Generic over any type `T` with a compile-time capacity `N`.
- Stores elements contiguously using `[MaybeUninit<T>; N]`.
- Tracks the logical queue using `head`, `tail`, and `len`.
- Uses modulo arithmetic to wrap indices around the buffer.
- Owns all initialized elements and releases them through a custom `Drop` implementation.

The implementation maintains the following invariants:

- `head` always points to the front element.
- `tail` always points to the next insertion position.
- Elements are stored within the logical range defined by `head`, `tail`, and `len`.
- Elements outside the logical queue remain uninitialized.
- `len <= N` always holds.

## Implementation Details

The queue is backed by a fixed-size array of `MaybeUninit<T>`, allowing storage to be 
reserved without constructing unused elements.

Insertion writes a value at the current `tail` position before advancing the index 
modulo the buffer capacity. Removal reads the value at the current `head` position, 
advances the head index, and decreases the logical length.

Because the indices wrap around the underlying array, freed slots are reused without 
moving existing elements, providing constant-time enqueue and dequeue operations 
while preserving contiguous storage.

## Complexity

| Operation   | Complexity |
|-------------|:----------:|
| `enqueue`   |  **O(1)**  |
| `dequeue`   |  **O(1)**  |
| `front`     |  **O(1)**  |
| `back`      |  **O(1)**  |
| `clear`     |  **O(n)**  |