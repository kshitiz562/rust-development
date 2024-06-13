
fn median_of_sorted_array(arr: &[i32]) -> f64 {
    let n = arr.len();
    if n % 2 == 0 {
        (arr[n / 2 - 1] + arr[n / 2]) as f64 / 2.0
    } else {
        arr[n / 2] as f64
    }
}
