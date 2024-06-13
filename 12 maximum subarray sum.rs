fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max_so_far = arr[0];
    let mut max_ending_here = arr[0];
    for &x in arr.iter().skip(1) {
        max_ending_here = max_ending_here.max(x + max_ending_here);
        max_so_far = max_so_far.max(max_ending_here);
    }
    max_so_far
}
