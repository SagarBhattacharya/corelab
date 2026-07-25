# Stack

## Overview

A Stack is a linear data structure that follows the Last-In, First-Out (LIFO) principle. 
Elements are inserted and removed only from the top of the stack.

## Operations

| Operation |     Complexity |
|-----------|---------------:|
| Push      | O(1) amortized |
| Pop       | O(1) amortized |
| Peek      |           O(1) |
| Len       |           O(1) |

## Design

- Backed by `Vec<T>`.
- Elements are appended and removed from the end.
- Provides constant-time access to the top element.

## Advantages

- Simple implementation.
- Efficient insertion and removal.
- Cache-friendly.

## Limitations

- Only the top element is accessible.
- Does not support random removal.

## Applications

- Function call stack.
- Undo/redo systems.
- Expression evaluation.
- Depth-first search.