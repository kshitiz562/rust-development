2. **Find the Index of the First Occurrence of a Given Number in a Sorted Array**

fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();
    while left < right {
        let mid = (left + right) / 2;
        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    if left < arr.len() && arr[left] == target {
        Some(left)
    } else {
        None
    }
}
