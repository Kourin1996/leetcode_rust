pub struct Solution {}

/* Submission Code Begins */
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s
            .to_lowercase()
            .chars()
            .filter_map(|c| if c.is_alphanumeric() { Some(c) } else { None })
            .collect();
        let n = chars.len();

        let rev_chars = &mut chars.clone()[(n / 2 + (if n % 2 == 0 { 0 } else { 1 }))..];
        rev_chars.reverse();

        chars[0..n / 2] == *rev_chars
    }
}
/* Submission Code Ends */
