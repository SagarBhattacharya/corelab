mod tests;

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.get_node(index).map(|node| &node.value)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.get_node_mut(index).map(|node| &mut node.value)
    }

    pub fn insert(&mut self, index: usize, value: T) -> Result<(), T> {
        if index > self.len {
            return Err(value);
        }

        let mut new_node = Box::new(Node { value, next: None });
        let current = if index == 0 {
            &mut self.head
        } else {
            &mut self.get_node_mut(index - 1).unwrap().next
        };

        new_node.next = current.take();
        *current = Some(new_node);
        self.len += 1;
        Ok(())
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }

        let current = if index == 0 {
            &mut self.head
        } else {
            &mut self.get_node_mut(index - 1)?.next
        };

        let mut removed = current.take()?;
        *current = removed.next.take();
        self.len -= 1;
        Some(removed.value)
    }

    fn get_node(&self, mut index: usize) -> Option<&Node<T>> {
        let mut curr = self.head.as_deref();

        while let Some(node) = curr {
            if index == 0 {
                return Some(node);
            }
            index -= 1;
            curr = node.next.as_deref();
        }

        None
    }

    fn get_node_mut(&mut self, mut index: usize) -> Option<&mut Node<T>> {
        let mut curr = self.head.as_deref_mut();

        while let Some(node) = curr {
            if index == 0 {
                return Some(node);
            }
            index -= 1;
            curr = node.next.as_deref_mut();
        }

        None
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn clear(&mut self) {
        while self.remove(0).is_some() {}
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}
