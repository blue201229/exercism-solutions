pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut start_idx = 0;
    let mut last_idx = array.len();
    let mut search_idx = (last_idx - start_idx) / 2;
    while last_idx - start_idx > 1 && array[search_idx] != key {
        if array[search_idx] < key {
            start_idx = search_idx;
        } else {
            last_idx = search_idx;
        }
        search_idx = start_idx + (last_idx - start_idx) / 2;
    }
    if array.get(search_idx) == Some(&key) {
        Some(search_idx)
    } else {
        None
    }
}