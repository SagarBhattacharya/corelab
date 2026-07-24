use corelab::static_vector::StaticVec;
use std::cell::Cell;
use std::rc::Rc;

#[test]
fn new_static_vec() {
    let vec = StaticVec::<i32, 8>::new();

    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 8);
    assert!(vec.is_empty());
    assert!(!vec.is_full());
}

#[test]
fn push_and_pop() {
    let mut vec = StaticVec::<i32, 4>::new();

    vec.push(1).unwrap();
    vec.push(2).unwrap();
    vec.push(3).unwrap();

    assert_eq!(vec.len(), 3);

    assert_eq!(vec.pop(), Some(3));
    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.pop(), Some(1));
    assert_eq!(vec.pop(), None);
}

#[test]
fn capacity_limit() {
    let mut vec = StaticVec::<i32, 2>::new();

    assert!(vec.push(10).is_ok());
    assert!(vec.push(20).is_ok());

    assert_eq!(vec.push(30), Err(30));
}

#[test]
fn get_and_get_mut() {
    let mut vec = StaticVec::<i32, 4>::new();

    vec.push(10).unwrap();
    vec.push(20).unwrap();

    assert_eq!(vec.get(0), Some(&10));
    assert_eq!(vec.get(1), Some(&20));
    assert_eq!(vec.get(2), None);

    *vec.get_mut(1).unwrap() = 99;

    assert_eq!(vec.get(1), Some(&99));
}

#[test]
fn clear_vector() {
    let mut vec = StaticVec::<i32, 4>::new();

    vec.push(1).unwrap();
    vec.push(2).unwrap();

    vec.clear();

    assert_eq!(vec.len(), 0);
    assert!(vec.is_empty());

    assert_eq!(vec.pop(), None);
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
        let mut vec = StaticVec::<DropCounter, 4>::new();

        vec.push(DropCounter {
            counter: counter.clone(),
        })
        .unwrap();

        vec.push(DropCounter {
            counter: counter.clone(),
        })
        .unwrap();
    }

    assert_eq!(counter.get(), 2);
}
