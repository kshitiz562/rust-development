
fn is_palindrome(s: &str) -> bool {
    let s: Vec<char> = s.chars().collect();
    s.iter().eq(s.iter().rev())
}
