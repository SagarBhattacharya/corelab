# DynamicVec

## Overview
DynamicVec is a dynamically resizable array that stores elements in contiguous heap memory.

Unlike `StaticVec`, its capacity grows automatically as elements are inserted, allowing 
it to store an arbitrary number of values while maintaining constant-time indexed access.

---

## Requirements

- Stores elements contiguously in heap memory.
- Uses `MaybeUninit<T>` to safely manage partially initialized memory.
- Uses `NonNull` to manage the heap allocation.
- Capacity grows automatically when full.

---

## Design

- Stores elements in heap memory.
- Uses `MaybeUninit<T>` to avoid constructing unused elements.
- Uses `NonNull` to safely own the allocated memory.
- Grows automatically using `realloc()`.
- Properly destroys initialized elements before deallocating memory.

---

## Memory Layout

```
Stack

ptr
len
capacity
   │
   ▼

Heap

+-------------------------------------------+
| T | T | T | ? | ? | ? | ? | ? |
+-------------------------------------------+
  ↑         ↑
initialized uninitialized
```

Only the first `len` elements are initialized.
Remaining capacity consists of uninitialized memory.

---

## Invariants

The implementation maintains the following invariants:

- `ptr` points to `capacity` contiguous `MaybeUninit<T>` values.
- Elements in `0..len` are initialized.
- Elements in `len..capacity` are uninitialized.
- `len <= capacity`.
- When `capacity == 0`, `ptr` is a dangling pointer and must never be dereferenced.

---

## Growth Strategy

Whenever the vector becomes full, its capacity doubles.

```
0 → 4 → 8 → 16 → 32 → ...
```

Doubling capacity minimizes reallocations while maintaining amortized constant-time insertion.

---

## Operations

| Operation |     Complexity |
|-----------|---------------:|
| Push      | O(1) amortized |
| Pop       |           O(1) |
| Get       |           O(1) |
| Get Mut   |           O(1) |
| Clear     |           O(n) |

---

## Advantages

- Dynamic capacity.
- Contiguous memory layout.
- Cache-friendly access.
- Efficient append operations.

---

## Limitations

- Reallocation invalidates raw pointers and references.
- Capacity only grows; it never shrinks.
- Reallocation may temporarily require additional memory.