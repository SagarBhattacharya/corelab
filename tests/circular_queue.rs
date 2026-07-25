use corelab::circular_queue::Queue;
use std::cell::Cell;
use std::rc::Rc;

#[test]
fn new_queue() {
    let queue = Queue::<i32, 4>::new();

    assert_eq!(queue.len(), 0);
    assert_eq!(queue.capacity(), 4);

    assert!(queue.is_empty());
    assert!(!queue.is_full());

    assert_eq!(queue.front(), None);
    assert_eq!(queue.back(), None);
}

#[test]
fn enqueue_and_dequeue() {
    let mut queue = Queue::<i32, 4>::new();

    queue.enqueue(10).unwrap();
    queue.enqueue(20).unwrap();
    queue.enqueue(30).unwrap();

    assert_eq!(queue.len(), 3);

    assert_eq!(queue.dequeue(), Some(10));
    assert_eq!(queue.dequeue(), Some(20));
    assert_eq!(queue.dequeue(), Some(30));
    assert_eq!(queue.dequeue(), None);

    assert!(queue.is_empty());
}

#[test]
fn front_and_back() {
    let mut queue = Queue::<i32, 4>::new();

    queue.enqueue(1).unwrap();
    queue.enqueue(2).unwrap();
    queue.enqueue(3).unwrap();

    assert_eq!(queue.front(), Some(&1));
    assert_eq!(queue.back(), Some(&3));

    queue.dequeue();

    assert_eq!(queue.front(), Some(&2));
    assert_eq!(queue.back(), Some(&3));
}

#[test]
fn full_queue() {
    let mut queue = Queue::<i32, 2>::new();

    assert!(queue.enqueue(1).is_ok());
    assert!(queue.enqueue(2).is_ok());

    assert!(queue.is_full());

    assert_eq!(queue.enqueue(3), Err(3));
}

#[test]
fn wrap_around() {
    let mut queue = Queue::<i32, 4>::new();

    queue.enqueue(1).unwrap();
    queue.enqueue(2).unwrap();
    queue.enqueue(3).unwrap();

    assert_eq!(queue.dequeue(), Some(1));
    assert_eq!(queue.dequeue(), Some(2));

    queue.enqueue(4).unwrap();
    queue.enqueue(5).unwrap();

    assert_eq!(queue.front(), Some(&3));
    assert_eq!(queue.back(), Some(&5));

    assert_eq!(queue.dequeue(), Some(3));
    assert_eq!(queue.dequeue(), Some(4));
    assert_eq!(queue.dequeue(), Some(5));

    assert!(queue.is_empty());
}

#[test]
fn clear_queue() {
    let mut queue = Queue::<i32, 8>::new();

    for i in 0..8 {
        queue.enqueue(i).unwrap();
    }

    queue.clear();

    assert!(queue.is_empty());
    assert_eq!(queue.len(), 0);

    assert_eq!(queue.dequeue(), None);
    assert_eq!(queue.front(), None);
    assert_eq!(queue.back(), None);
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
        let mut queue = Queue::<DropCounter, 8>::new();

        for _ in 0..5 {
            queue
                .enqueue(DropCounter {
                    counter: counter.clone(),
                })
                .unwrap();
        }
    }

    assert_eq!(counter.get(), 5);
}

#[test]
fn stress_test() {
    let mut queue = Queue::<i32, 128>::new();

    for i in 0..128 {
        queue.enqueue(i).unwrap();
    }

    for i in 0..128 {
        assert_eq!(queue.dequeue(), Some(i));
    }

    assert!(queue.is_empty());
}
