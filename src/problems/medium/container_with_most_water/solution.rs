pub struct Solution {}

/* Submission Code Begins */
use std::cmp::Ordering;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut ans = 0;

        while left < right {
            let h = height[left].min(height[right]);
            let a = h * ((right - left) as i32);

            ans = ans.max(a);

            match height[left].cmp(&height[right]) {
                Ordering::Less | Ordering::Equal => {
                    left += 1;
                }
                Ordering::Greater => {
                    right -= 1;
                }
            }
        }

        ans
    }
}
/* Submission Code Ends */
