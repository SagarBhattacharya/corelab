# Dynamic Vector

## Introduction

A generic dynamically resizable vector that stores elements in contiguous heap memory.

Unlike a fixed-capacity vector, the backing storage grows automatically as elements 
are inserted, allowing the collection to accommodate an arbitrary number of values 
while preserving constant-time indexed access.

## Design

- Generic over any type `T`.
- Stores elements contiguously in heap-allocated memory.
- Uses `MaybeUninit<T>` to manage partially initialized storage.
- Owns the heap allocation through `NonNull<MaybeUninit<T>>`.
- Grows automatically by reallocating the underlying buffer when capacity is exhausted.
- Releases initialized elements before deallocating memory.

The implementation maintains the following invariants:

- `ptr` references `capacity` contiguous `MaybeUninit<T>` elements.
- Elements in `0..len` are initialized.
- Elements in `len..capacity` remain uninitialized.
- `len <= capacity` always holds.
- When `capacity == 0`, `ptr` is a dangling pointer and must never be dereferenced.

## Implementation Details

Memory is allocated manually using Rust's global allocator through `std::alloc`. 
The vector begins with zero capacity and allocates an initial buffer on the first insertion.

Whenever the current capacity is exhausted, the allocation is resized 
using `realloc()`, doubling the available storage.

```
0 → 4 → 8 → 16 → 32 → ...
```

This growth strategy minimizes the number of reallocations while providing amortized constant-time insertion.

Values are written into uninitialized memory using `MaybeUninit<T>`, removed using `assume_init()`, 
and cleaned up through a custom `Drop` implementation that destroys only the initialized portion 
of the allocation before releasing the backing memory.

## Complexity

| Operation |     Complexity     |
|-----------|:------------------:|
| `push`    | **O(1)** amortized |
| `pop`     |      **O(1)**      |
| `get`     |      **O(1)**      |
| `get_mut` |      **O(1)**      |
| `clear`   |      **O(n)**      |