
fn kth_smallest(arr: &[i32], k: usize) -> i32 {
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    sorted_arr[k - 1]
}
