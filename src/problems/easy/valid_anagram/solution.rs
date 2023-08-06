pub struct Solution {}

/* Submission Code Begins */
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut ss = s.chars().collect::<Vec<_>>();
        let mut ts = t.chars().collect::<Vec<_>>();

        ss.sort();
        ts.sort();

        ss == ts
    }
}
/* Submission Code Ends */
