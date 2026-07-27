mod tests;

use std::mem::MaybeUninit;

pub struct Queue<T, const N: usize> {
    buffer: [MaybeUninit<T>; N],
    len: usize,
    head: usize,
    tail: usize,
}

impl<T, const N: usize> Queue<T, N> {
    pub fn new() -> Self {
        Self {
            buffer: std::array::from_fn(|_| MaybeUninit::uninit()),
            len: 0,
            head: 0,
            tail: 0,
        }
    }

    pub fn enqueue(&mut self, value: T) -> Result<(), T> {
        if self.len == N {
            return Err(value);
        }

        self.buffer[self.tail] = MaybeUninit::new(value);
        self.tail = (self.tail + 1) % N;
        self.len += 1;
        Ok(())
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let value = unsafe { self.buffer[self.head].assume_init_read() };
        self.head = (self.head + 1) % N;
        self.len -= 1;

        Some(value)
    }

    pub fn front(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            Some(unsafe { self.buffer[self.head].assume_init_ref() })
        }
    }

    pub fn back(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            let idx = (self.tail + N - 1) % N;
            Some(unsafe { self.buffer[idx].assume_init_ref() })
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == N
    }

    pub fn clear(&mut self) {
        while self.dequeue().is_some() {}
    }
}

impl<T, const N: usize> Drop for Queue<T, N> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T, const N: usize> Default for Queue<T, N> {
    fn default() -> Self {
        Self::new()
    }
}
