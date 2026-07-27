#![cfg(test)]

use super::Stack;

#[test]
fn new_stack() {
    let mut stack = Stack::<i32>::new();

    assert_eq!(stack.len(), 0);
    assert!(stack.is_empty());
    assert_eq!(stack.peek(), None);
    assert_eq!(stack.pop(), None);
}

#[test]
fn push_and_pop() {
    let mut stack = Stack::new();

    stack.push(10);
    stack.push(20);
    stack.push(30);

    assert_eq!(stack.len(), 3);

    assert_eq!(stack.pop(), Some(30));
    assert_eq!(stack.pop(), Some(20));
    assert_eq!(stack.pop(), Some(10));
    assert_eq!(stack.pop(), None);
}

#[test]
fn peek() {
    let mut stack = Stack::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    assert_eq!(stack.peek(), Some(&3));

    // Peek should not remove the element.
    assert_eq!(stack.len(), 3);

    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.peek(), Some(&2));
}

#[test]
fn empty_stack() {
    let mut stack = Stack::<i32>::new();

    assert_eq!(stack.peek(), None);
    assert_eq!(stack.pop(), None);

    assert!(stack.is_empty());
}

#[test]
fn stress_test() {
    let mut stack = Stack::new();

    for i in 0..10_000 {
        stack.push(i);
    }

    assert_eq!(stack.len(), 10_000);

    for i in (0..10_000).rev() {
        assert_eq!(stack.pop(), Some(i));
    }

    assert!(stack.is_empty());
}
