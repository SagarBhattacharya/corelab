use std::alloc::Layout;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

pub struct DynamicVec<T> {
    ptr: NonNull<MaybeUninit<T>>,
    len: usize,
    capacity: usize,
}

impl<T> DynamicVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };

        let layout = Layout::array::<MaybeUninit<T>>(new_capacity).expect("capacity overflow");

        let new_ptr = if self.capacity == 0 {
            unsafe { std::alloc::alloc(layout) }
        } else {
            let old_layout = Layout::array::<MaybeUninit<T>>(self.capacity).unwrap();
            unsafe { std::alloc::realloc(self.ptr.as_ptr() as *mut u8, old_layout, layout.size()) }
        };

        self.ptr = match NonNull::new(new_ptr as *mut MaybeUninit<T>) {
            None => std::alloc::handle_alloc_error(layout),
            Some(p) => p,
        };
        self.capacity = new_capacity;
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.capacity {
            self.grow();
        }

        unsafe {
            self.ptr
                .as_ptr()
                .add(self.len)
                .write(MaybeUninit::new(value));
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        Some(unsafe { self.ptr.as_ptr().add(self.len).read().assume_init() })
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        Some(unsafe { (&*self.ptr.as_ptr().add(index)).assume_init_ref() })
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }
        Some(unsafe { (&mut *self.ptr.as_ptr().add(index)).assume_init_mut() })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.capacity
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn clear(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<T> Drop for DynamicVec<T> {
    fn drop(&mut self) {
        self.clear();

        if self.capacity != 0 {
            let layout = Layout::array::<MaybeUninit<T>>(self.capacity).unwrap();
            unsafe {
                std::alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<T> Default for DynamicVec<T> {
    fn default() -> Self {
        Self::new()
    }
}
