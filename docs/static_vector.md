# StaticVec

## Overview
StaticVec is a fixed-capacity vector that stores elements inline without heap allocation. 
The maximum capacity is determined at compile time using const generics.

Unlike `Vec<T>`, its capacity cannot grow.

---

## Requirements

- Capacity must be known at compile time.
- Uses `MaybeUninit<T>` to safely manage partially initialized memory.
- Stores elements contiguously.

---

## Memory Layout

```
┌────────────────────────────┐
│ MaybeUninit<T>; N          │
├────────────────────────────┤
│ initialized │ uninitialized│
└────────────────────────────┘
↑
len
```

Only the first `len` elements are initialized.

---

## Invariants

- Elements in `0..len` are initialized.
- Elements in `len..capacity` are uninitialized.

All operations preserve these invariants.

---

## Design

- Uses `MaybeUninit<T>` to avoid constructing unused elements.
- Maintains a logical length separate from capacity.
- Implements `Drop` to correctly destroy initialized elements.

---

## Operations

| Operation | Complexity |
|-----------|-----------:|
| push      |       O(1) |
| pop       |       O(1) |
| get       |       O(1) |
| get_mut   |       O(1) |
| clear     |       O(n) |

---

## Advantages

- No heap allocation.
- Constant-time insertion and removal at the end.
- Predictable memory usage.
- Suitable for embedded and systems programming.

---

## Limitations

- Capacity is fixed.
- Cannot grow dynamically.
- Wasted memory if underutilized.