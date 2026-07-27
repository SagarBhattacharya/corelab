use corelab::binary_heap::MaxBinaryHeap;

#[test]
fn new_heap() {
    let heap = MaxBinaryHeap::<i32>::new();

    assert_eq!(heap.len(), 0);
    assert!(heap.is_empty());
    assert_eq!(heap.peek(), None);
}

#[test]
fn insert_one() {
    let mut heap = MaxBinaryHeap::new();

    heap.insert(42);

    assert_eq!(heap.len(), 1);
    assert_eq!(heap.peek(), Some(&42));
}

#[test]
fn insert_ascending() {
    let mut heap = MaxBinaryHeap::new();

    for i in 1..=10 {
        heap.insert(i);
    }

    assert_eq!(heap.peek(), Some(&10));
}

#[test]
fn insert_descending() {
    let mut heap = MaxBinaryHeap::new();

    for i in (1..=10).rev() {
        heap.insert(i);
    }

    assert_eq!(heap.peek(), Some(&10));
}

#[test]
fn extract_sorted() {
    let mut heap = MaxBinaryHeap::new();

    for &v in &[5, 3, 8, 1, 9, 2, 7] {
        heap.insert(v);
    }

    let mut result = Vec::new();

    while let Some(v) = heap.extract() {
        result.push(v);
    }

    assert_eq!(result, vec![9, 8, 7, 5, 3, 2, 1]);
}

#[test]
fn duplicate_values() {
    let mut heap = MaxBinaryHeap::new();

    heap.insert(5);
    heap.insert(5);
    heap.insert(5);

    assert_eq!(heap.extract(), Some(5));
    assert_eq!(heap.extract(), Some(5));
    assert_eq!(heap.extract(), Some(5));
    assert_eq!(heap.extract(), None);
}

#[test]
fn single_element() {
    let mut heap = MaxBinaryHeap::new();

    heap.insert(10);

    assert_eq!(heap.extract(), Some(10));
    assert_eq!(heap.extract(), None);
}

#[test]
fn empty_heap() {
    let mut heap = MaxBinaryHeap::<i32>::new();

    assert_eq!(heap.extract(), None);
    assert_eq!(heap.peek(), None);
}

#[test]
fn stress_test() {
    let mut heap = MaxBinaryHeap::new();

    for i in 0..1000 {
        heap.insert(i);
    }

    for i in (0..1000).rev() {
        assert_eq!(heap.extract(), Some(i));
    }

    assert!(heap.is_empty());
}

#[test]
fn alternating_insert_extract() {
    let mut heap = MaxBinaryHeap::new();

    heap.insert(5);
    assert_eq!(heap.extract(), Some(5));

    heap.insert(2);
    heap.insert(9);
    assert_eq!(heap.extract(), Some(9));

    heap.insert(7);
    heap.insert(1);
    assert_eq!(heap.extract(), Some(7));

    heap.insert(10);

    assert_eq!(heap.extract(), Some(10));
    assert_eq!(heap.extract(), Some(2));
    assert_eq!(heap.extract(), Some(1));
    assert_eq!(heap.extract(), None);
}
