
fn merge_sorted_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }
    while i < arr1.len() {
        merged.push(arr1[i]);
        i += 1;
    }
    while j < arr2.len() {
        merged.push(arr2[j]);
        j += 1;
    }
    merged
}

