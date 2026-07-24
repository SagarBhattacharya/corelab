use corelab::dynamic_vector::DynamicVec;
use std::cell::Cell;
use std::rc::Rc;

#[test]
fn new_dynamic_vec() {
    let vec = DynamicVec::<i32>::new();

    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 0);
    assert!(vec.is_empty());
    assert!(vec.is_full()); // len == capacity == 0
}

#[test]
fn push_and_pop() {
    let mut vec = DynamicVec::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);

    assert_eq!(vec.len(), 3);

    assert_eq!(vec.pop(), Some(3));
    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.pop(), Some(1));
    assert_eq!(vec.pop(), None);
}

#[test]
fn grows_automatically() {
    let mut vec = DynamicVec::new();

    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100);
    assert!(vec.capacity() >= 100);

    for i in (0..100).rev() {
        assert_eq!(vec.pop(), Some(i));
    }

    assert!(vec.is_empty());
}

#[test]
fn get_elements() {
    let mut vec = DynamicVec::new();

    vec.push(10);
    vec.push(20);
    vec.push(30);

    assert_eq!(vec.get(0), Some(&10));
    assert_eq!(vec.get(1), Some(&20));
    assert_eq!(vec.get(2), Some(&30));

    assert_eq!(vec.get(3), None);
}

#[test]
fn get_mut_elements() {
    let mut vec = DynamicVec::new();

    vec.push(10);
    vec.push(20);

    *vec.get_mut(1).unwrap() = 99;

    assert_eq!(vec.get(1), Some(&99));
}

#[test]
fn clear_vector() {
    let mut vec = DynamicVec::new();

    for i in 0..20 {
        vec.push(i);
    }

    vec.clear();

    assert_eq!(vec.len(), 0);
    assert!(vec.is_empty());

    assert_eq!(vec.pop(), None);
}

#[test]
fn stress_test() {
    let mut vec = DynamicVec::new();

    for i in 0..10_000 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 10_000);

    for i in (0..10_000).rev() {
        assert_eq!(vec.pop(), Some(i));
    }

    assert!(vec.is_empty());
}

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
        let mut vec = DynamicVec::new();

        for _ in 0..10 {
            vec.push(DropCounter {
                counter: counter.clone(),
            });
        }
    }

    assert_eq!(counter.get(), 10);
}
