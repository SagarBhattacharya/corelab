# Static Vector

## Introduction

A generic fixed-capacity vector that stores elements inline without heap allocation.

The maximum capacity is determined at compile time using const generics, making the 
data structure suitable for applications where memory usage must remain predictable. 
Unlike a dynamic vector, the capacity cannot grow after construction.

## Design

- Generic over any type `T` with a compile-time capacity `N`.
- Stores elements contiguously using `[MaybeUninit<T>; N]`.
- Maintains a logical length independent of physical capacity.
- Tracks initialized and uninitialized memory separately.
- Owns all initialized elements and releases them through a custom `Drop` implementation.

The implementation maintains the following invariants:

- Elements in `0..len` are initialized.
- Elements in `len..N` remain uninitialized.
- `len <= N` always holds.

## Implementation Details

The backing storage consists of an array of `MaybeUninit<T>`, allowing 
memory to be reserved without immediately constructing values.

Elements are written into uninitialized slots during insertion and read 
back using `assume_init()` during removal. A custom `Drop` implementation 
destroys only the initialized portion of the array, ensuring correct resource 
management while avoiding undefined behavior.

Because the storage is allocated inline, all element accesses are contiguous 
in memory and no heap allocation occurs during the lifetime of the vector.

## Complexity

| Operation | Complexity |
|-----------|:----------:|
| `push`    |  **O(1)**  |
| `pop`     |  **O(1)**  |
| `get`     |  **O(1)**  |
| `get_mut` |  **O(1)**  |
| `clear`   |  **O(n)**  |