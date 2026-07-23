use std::cmp::Ordering;

pub fn binary_search<T: Ord>(list: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = list.len();

    while low < high {
        let mid = low + (high - low) / 2;
        match list[mid].cmp(target) {
            Ordering::Less => low = mid + 1,
            Ordering::Equal => return Some(mid),
            Ordering::Greater => high = mid,
        }
    }

    None
}
