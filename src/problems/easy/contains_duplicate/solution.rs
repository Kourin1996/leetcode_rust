pub struct Solution {}

/* Submission Code Starts */
use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for (i, n) in nums.iter().enumerate() {
            set.insert(n);

            if set.len() < i + 1 {
                return true;
            }
        }

        false
    }
}
/* Submission Code Ends */
