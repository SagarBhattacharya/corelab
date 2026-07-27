use std::ptr::NonNull;

type Link<T> = NonNull<Node<T>>;

struct Node<T> {
    value: T,
    prev: Option<Link<T>>,
    next: Option<Link<T>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Link<T> {
        let node = Box::new(Self {
            value,
            prev: None,
            next: None,
        });

        NonNull::new(Box::into_raw(node)).expect("new node should be non-null")
    }
}

pub struct DoublyLinkedList<T> {
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
    len: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    fn set_head_tail(&mut self, value: Option<Link<T>>) {
        self.head = value;
        self.tail = value;
    }

    pub fn push_front(&mut self, value: T) {
        let mut new_node = Node::new(value);
        if self.len == 0 {
            self.set_head_tail(Some(new_node));
        } else {
            let mut current = self.head.replace(new_node).unwrap();
            unsafe {
                current.as_mut().prev = Some(new_node);
                new_node.as_mut().next = Some(current);
            }
        }
        self.len += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let mut new_node = Node::new(value);
        if self.len == 0 {
            self.set_head_tail(Some(new_node));
        } else {
            let mut previous = self.tail.replace(new_node).unwrap();
            unsafe {
                previous.as_mut().next = Some(new_node);
                new_node.as_mut().prev = Some(previous);
            }
        }
        self.len += 1;
    }

    pub fn insert(&mut self, index: usize, value: T) -> Result<(), T> {
        if index > self.len {
            return Err(value);
        }

        if index == 0 {
            self.push_front(value);
            return Ok(());
        }

        if index == self.len {
            self.push_back(value);
            return Ok(());
        }

        let mut new_node = Node::new(value);
        unsafe {
            let mut curr = self.get_node(index).unwrap();
            let mut prev = curr.as_ref().prev.unwrap();

            new_node.as_mut().prev = Some(prev);
            new_node.as_mut().next = Some(curr);

            prev.as_mut().next = Some(new_node);
            curr.as_mut().prev = Some(new_node);
        }

        self.len += 1;
        Ok(())
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let removed = unsafe { Box::from_raw(self.head?.as_ptr()) };

        if self.len == 1 {
            self.set_head_tail(None);
        } else {
            self.head = removed.next;
            unsafe {
                self.head.unwrap().as_mut().prev = None;
            }
        }

        self.len -= 1;
        Some(removed.value)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let removed = unsafe { Box::from_raw(self.tail?.as_ptr()) };

        if self.len == 1 {
            self.set_head_tail(None);
        } else {
            self.tail = removed.prev;
            unsafe {
                self.tail.unwrap().as_mut().next = None;
            }
        }

        self.len -= 1;
        Some(removed.value)
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        if index == 0 {
            return self.pop_front();
        }
        if index == self.len - 1 {
            return self.pop_back();
        }

        let curr = self.get_node(index)?;
        unsafe {
            let mut prev = curr.as_ref().prev.unwrap();
            let mut next = curr.as_ref().next.unwrap();

            prev.as_mut().next = Some(next);
            next.as_mut().prev = Some(prev);
        }

        let removed = unsafe { Box::from_raw(curr.as_ptr()) };

        self.len -= 1;
        Some(removed.value)
    }

    fn get_node(&self, index: usize) -> Option<Link<T>> {
        if index >= self.len {
            return None;
        }

        if index < self.len / 2 {
            let mut curr = self.head;
            for _ in 0..index {
                unsafe {
                    curr = curr?.as_ref().next;
                }
            }
            curr
        } else {
            let mut curr = self.tail;
            for _ in 0..(self.len - 1 - index) {
                unsafe {
                    curr = curr?.as_ref().prev;
                }
            }
            curr
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_back().is_some() {}
    }
}

impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}
