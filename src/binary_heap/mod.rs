mod tests;

pub struct MaxBinaryHeap<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord> MaxBinaryHeap<T> {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn insert(&mut self, value: T) {
        self.data.push(value);
        self.sift_up(self.data.len() - 1);
    }

    pub fn extract(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let last_idx = self.data.len() - 1;
        self.data.swap(0, last_idx);

        let value = self.data.pop();
        if !self.data.is_empty() {
            self.sift_down(0);
        }
        value
    }

    pub fn peek(&self) -> Option<&T> {
        if self.data.is_empty() {
            return None;
        }
        self.data.first()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let p = parent(idx);
            if self.data[p] >= self.data[idx] {
                break;
            }

            self.data.swap(p, idx);
            idx = p;
        }
    }

    fn sift_down(&mut self, mut idx: usize) {
        while let Some(max_idx) = self.get_max_child(idx) {
            if self.data[idx] >= self.data[max_idx] {
                break;
            }
            self.data.swap(idx, max_idx);
            idx = max_idx;
        }
    }

    fn get_max_child(&self, parent_idx: usize) -> Option<usize> {
        let left_idx = left(parent_idx);
        let right_idx = right(parent_idx);
        let len = self.data.len();

        match (left_idx < len, right_idx < len) {
            (true, true) => {
                if self.data[left_idx] > self.data[right_idx] {
                    Some(left_idx)
                } else {
                    Some(right_idx)
                }
            }
            (true, false) => Some(left_idx),
            (_, _) => None,
        }
    }
}

fn parent(i: usize) -> usize {
    (i - 1) / 2
}
fn left(i: usize) -> usize {
    2 * i + 1
}
fn right(i: usize) -> usize {
    2 * i + 2
}

impl<T: Ord> Default for MaxBinaryHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}
