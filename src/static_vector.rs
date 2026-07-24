use std::mem::MaybeUninit;

pub struct StaticVec<T, const N: usize> {
    array: [MaybeUninit<T>; N],
    size: usize,
}

impl<T, const N: usize> StaticVec<T, N> {
    pub fn new() -> Self {
        Self {
            array: std::array::from_fn(|_| MaybeUninit::uninit()),
            size: 0,
        }
    }

    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.size == N {
            return Err(value);
        }

        self.array[self.size] = MaybeUninit::new(value);
        self.size += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        self.size -= 1;
        let value = unsafe { self.array[self.size].assume_init_read() };
        Some(value)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size {
            return None;
        }
        Some(unsafe { self.array[index].assume_init_ref() })
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.size {
            return None;
        }
        Some(unsafe { self.array[index].assume_init_mut() })
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == N
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn clear(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<T, const N: usize> Drop for StaticVec<T, N> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<T, const N: usize> Default for StaticVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}
