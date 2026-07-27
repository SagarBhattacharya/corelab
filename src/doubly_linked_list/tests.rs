#![cfg(test)]

use super::DoublyLinkedList;

#[test]
fn new_list() {
    let list = DoublyLinkedList::<i32>::new();

    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
}

#[test]
fn push_front() {
    let mut list = DoublyLinkedList::new();

    list.push_front(1);
    list.push_front(2);
    list.push_front(3);

    assert_eq!(list.len(), 3);

    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), None);
}

#[test]
fn push_back() {
    let mut list = DoublyLinkedList::new();

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    assert_eq!(list.len(), 3);

    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(3));
}

#[test]
fn pop_back() {
    let mut list = DoublyLinkedList::new();

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), None);
}

#[test]
fn insert_front() {
    let mut list = DoublyLinkedList::new();

    list.insert(0, 1).unwrap();
    list.insert(0, 2).unwrap();

    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(1));
}

#[test]
fn insert_back() {
    let mut list = DoublyLinkedList::new();

    list.insert(0, 1).unwrap();
    list.insert(1, 2).unwrap();
    list.insert(2, 3).unwrap();

    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(3));
}

#[test]
fn insert_middle() {
    let mut list = DoublyLinkedList::new();

    list.push_back(1);
    list.push_back(3);

    list.insert(1, 2).unwrap();

    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(3));
}

#[test]
fn remove_front() {
    let mut list = DoublyLinkedList::new();

    list.push_back(1);
    list.push_back(2);

    assert_eq!(list.remove(0), Some(1));
    assert_eq!(list.remove(0), Some(2));
    assert_eq!(list.remove(0), None);
}

#[test]
fn remove_back() {
    let mut list = DoublyLinkedList::new();

    list.push_back(1);
    list.push_back(2);

    assert_eq!(list.remove(1), Some(2));
    assert_eq!(list.remove(0), Some(1));
}

#[test]
fn remove_middle() {
    let mut list = DoublyLinkedList::new();

    for i in 1..=5 {
        list.push_back(i);
    }

    assert_eq!(list.remove(2), Some(3));

    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), Some(4));
    assert_eq!(list.pop_front(), Some(5));
}

#[test]
fn alternating_operations() {
    let mut list = DoublyLinkedList::new();

    list.push_back(1);
    list.push_front(0);
    list.push_back(2);

    assert_eq!(list.pop_front(), Some(0));

    list.push_front(-1);

    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), Some(-1));
}

#[test]
fn stress_test() {
    let mut list = DoublyLinkedList::new();

    for i in 0..1000 {
        list.push_back(i);
    }

    for i in 0..1000 {
        assert_eq!(list.pop_front(), Some(i));
    }

    assert!(list.is_empty());
}
