pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut l = 0;
    let mut r = array.len();
    if r == 0 {
        return None;
    }

    while l < r {
        let m = ((l + r) / 2) as usize;
        if array[m] == key {
            return Some(m);
        } else if array[m] > key {
            r = m;
        } else {
            l = m + 1;
        }
    }
    None
}
