#![cfg(test)]

use super::LinkedList;
use std::cell::Cell;
use std::rc::Rc;

#[test]
fn new_list() {
    let list = LinkedList::<i32>::new();

    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
    assert_eq!(list.get(0), None);
}

#[test]
fn insert_front() {
    let mut list = LinkedList::new();

    list.insert(0, 10).unwrap();
    list.insert(0, 20).unwrap();
    list.insert(0, 30).unwrap();

    assert_eq!(list.len(), 3);

    assert_eq!(list.get(0), Some(&30));
    assert_eq!(list.get(1), Some(&20));
    assert_eq!(list.get(2), Some(&10));
}

#[test]
fn insert_back() {
    let mut list = LinkedList::new();

    list.insert(0, 1).unwrap();
    list.insert(1, 2).unwrap();
    list.insert(2, 3).unwrap();

    assert_eq!(list.get(0), Some(&1));
    assert_eq!(list.get(1), Some(&2));
    assert_eq!(list.get(2), Some(&3));
}

#[test]
fn insert_middle() {
    let mut list = LinkedList::new();

    list.insert(0, 1).unwrap();
    list.insert(1, 3).unwrap();
    list.insert(1, 2).unwrap();

    assert_eq!(list.get(0), Some(&1));
    assert_eq!(list.get(1), Some(&2));
    assert_eq!(list.get(2), Some(&3));
}

#[test]
fn remove_front() {
    let mut list = LinkedList::new();

    list.insert(0, 1).unwrap();
    list.insert(1, 2).unwrap();
    list.insert(2, 3).unwrap();

    assert_eq!(list.remove(0), Some(1));

    assert_eq!(list.get(0), Some(&2));
    assert_eq!(list.get(1), Some(&3));
    assert_eq!(list.len(), 2);
}

#[test]
fn remove_middle() {
    let mut list = LinkedList::new();

    for i in 1..=5 {
        list.insert(list.len(), i).unwrap();
    }

    assert_eq!(list.remove(2), Some(3));

    assert_eq!(list.get(0), Some(&1));
    assert_eq!(list.get(1), Some(&2));
    assert_eq!(list.get(2), Some(&4));
    assert_eq!(list.get(3), Some(&5));
}

#[test]
fn remove_back() {
    let mut list = LinkedList::new();

    for i in 1..=3 {
        list.insert(list.len(), i).unwrap();
    }

    assert_eq!(list.remove(2), Some(3));

    assert_eq!(list.get(0), Some(&1));
    assert_eq!(list.get(1), Some(&2));
    assert_eq!(list.len(), 2);
}

#[test]
fn get_mut() {
    let mut list = LinkedList::new();

    list.insert(0, 10).unwrap();

    *list.get_mut(0).unwrap() = 42;

    assert_eq!(list.get(0), Some(&42));
}

#[test]
fn out_of_bounds() {
    let mut list = LinkedList::<i32>::new();

    assert!(list.insert(1, 5).is_err());

    assert_eq!(list.remove(0), None);

    assert_eq!(list.get(0), None);

    assert_eq!(list.get_mut(0), None);
}

#[test]
fn stress_test() {
    let mut list = LinkedList::new();

    for i in 0..100 {
        list.insert(list.len(), i).unwrap();
    }

    assert_eq!(list.len(), 100);

    for i in 0..100 {
        assert_eq!(list.remove(0), Some(i));
    }

    assert!(list.is_empty());
}

#[test]
fn alternating_insert_remove() {
    let mut list = LinkedList::new();

    list.insert(0, 1).unwrap();
    list.insert(1, 2).unwrap();
    list.insert(2, 3).unwrap();

    assert_eq!(list.remove(1), Some(2));

    list.insert(1, 4).unwrap();

    assert_eq!(list.get(0), Some(&1));
    assert_eq!(list.get(1), Some(&4));
    assert_eq!(list.get(2), Some(&3));

    assert_eq!(list.remove(0), Some(1));
    assert_eq!(list.remove(0), Some(4));
    assert_eq!(list.remove(0), Some(3));

    assert!(list.is_empty());
}

#[derive(Debug)]
struct DropCounter {
    counter: Rc<Cell<usize>>,
}

impl Drop for DropCounter {
    fn drop(&mut self) {
        self.counter.set(self.counter.get() + 1);
    }
}

#[test]
fn drops_everything() {
    let counter = Rc::new(Cell::new(0));

    {
        let mut list = LinkedList::new();

        for _ in 0..10 {
            list.insert(
                list.len(),
                DropCounter {
                    counter: counter.clone(),
                },
            )
            .unwrap();
        }
    }

    assert_eq!(counter.get(), 10);
}
