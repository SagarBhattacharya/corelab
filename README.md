# CoreLab

![Rust CI](https://github.com/SagarBhattacharya/corelab/actions/workflows/rust.yml/badge.svg)

A curated collection of **idiomatic Rust implementations** of classic data structures and algorithms.

Each implementation focuses on:
- Idiomatic Rust
- Clear documentation
- Comprehensive testing
- Efficient implementations
- Modern testing techniques

---

## Goals

CoreLab is a long-term learning project aimed at building production-quality implementations of fundamental computer science concepts.

Rather than collecting interview solutions, each implementation is treated as a small library with attention to:

- API design
- Code quality
- Documentation
- Testing
- Performance
- Maintainability

---

## Repository Structure

```
corelab/
│
├── docs/
│   ├── binary_search.md
│   ├── stack.md
│   └── ...
│
├── src/
│   ├── binary_search.rs
│   ├── stack.rs
│   └── ...
│
├── tests/
│   ├── binary_search.rs
│   ├── stack.rs
│   └── ...
│
└── README.md
```

---

## Implementations

| Category  | Algorithm / Data Structure | Status |
|-----------|----------------------------|:------:|
| Searching | Binary Search              |   ✅    |
| Linear    | Stack                      |   ⏳    |
| Linear    | Queue                      |   ⏳    |
| Sorting   | Merge Sort                 |   ⏳    |
| Sorting   | Quick Sort                 |   ⏳    |
| Trees     | Binary Search Tree         |   ⏳    |
| Trees     | AVL Tree                   |   ⏳    |
| Trees     | Red-Black Tree             |   ⏳    |
| Graphs    | BFS                        |   ⏳    |
| Graphs    | DFS                        |   ⏳    |
| Graphs    | Dijkstra                   |   ⏳    |

> This table will continue to grow as new implementations are added.

---

## Testing Philosophy

Depending on the implementation, different testing techniques are used:

- Unit Tests
- Edge Case Tests
- Table-Driven Tests
- Differential Testing
- Property-Based Testing
- Benchmarking
- Fuzz Testing

---

## Running Locally

Clone the repository

```bash
git clone https://github.com/SagarBhattacharya/corelab.git
cd corelab
```

Run formatting checks

```bash
cargo fmt --check
```

Run Clippy

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

Run tests

```bash
cargo test
```

---

## Continuous Integration

Every push and pull request is automatically verified using GitHub Actions.

The CI pipeline currently performs:

- Formatting checks
- Clippy linting
- Compilation
- Test execution

---

## Learning Progress

This repository is developed incrementally.

Each implementation is written from scratch with accompanying documentation and tests before moving to the next topic.

---

## License

MIT