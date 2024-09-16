use std::cmp::Ordering;

fn binary_search(array: &[i32], key: i32, start: usize, end: usize) -> Option<usize> {
    if start >= end {
        return None;
    };

    let mid = start + (end - start) / 2;

    match array[mid].cmp(&key) {
        Ordering::Equal => Some(mid),
        Ordering::Less => binary_search(array, key, mid + 1, end),
        Ordering::Greater => binary_search(array, key, start, mid),
    }
}

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    binary_search(array, key, 0, array.len())
}
