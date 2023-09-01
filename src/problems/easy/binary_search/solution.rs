pub struct Solution {}

/* Submission Code Begins */
use std::cmp::Ordering;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;

        while left <= right {
            let mid = (left + right) / 2;

            match nums[mid as usize].cmp(&target) {
                Ordering::Equal => {
                    return mid as i32;
                }
                Ordering::Greater => {
                    right = mid - 1;
                }
                Ordering::Less => {
                    left = mid + 1;
                }
            }
        }

        -1
    }
}
/* Submission Code Ends */
